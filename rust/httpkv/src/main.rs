use std::collections::{HashMap, VecDeque};
use std::io::Read;
use std::sync::mpsc::{self, Receiver, Sender, SyncSender, TryRecvError};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};

use clap::Parser;
use futures::{future, FutureExt, TryFutureExt, TryStreamExt};
use futures_channel::oneshot;
use futures_executor::block_on;
use grpcio::{
    ChannelBuilder, ClientStreamingSink, EnvBuilder, Environment, Error as GrpcError,
    RequestStream, RpcContext, RpcStatus, RpcStatusCode, ServerBuilder, UnarySink,
};
use kvproto::raft_serverpb::{Done, RaftMessage};
use kvproto::tikvpb_grpc::{create_tikv, Tikv, TikvClient};
use protobuf::Message as PbMessage;
use raft::storage::MemStorage;
use raft::{prelude::*, StateRole};
use regex::Regex;
use slog::{error, info, o, Drain};
use thiserror::Error;

#[derive(Clone)]
struct TikvService {
    node: Arc<Mutex<Node>>,
    logger: slog::Logger,
    sender: Sender<Message>,
}

#[derive(Debug, Error)]
enum MyError {
    #[error("{0:?}")]
    Grpc(#[from] GrpcError),
}

impl Tikv for TikvService {
    fn raft_unary(&mut self, ctx: RpcContext, req: RaftMessage, sink: UnarySink<Done>) {
        // info!(self.logger, "received raft_unary message");
        let _ = self.sender.send(req.get_message().clone());
        let resp = Done::default();
        let f = sink.success(resp).map_err(MyError::from).map(|_| ());
        ctx.spawn(f)
    }

    fn raft(
        &mut self,
        ctx: RpcContext,
        stream: RequestStream<RaftMessage>,
        sink: ClientStreamingSink<Done>,
    ) {
        let store_id = self.node.lock().unwrap().id;
        let sender = self.sender.clone();
        ctx.spawn(async move {
            let res = stream.map_err(MyError::from).try_for_each(move |msg| {
                let to_store_id = msg.get_to_peer().get_store_id();
                if to_store_id != store_id {
                    future::err(MyError::Grpc(GrpcError::RpcFailure(RpcStatus::new(
                        RpcStatusCode::UNKNOWN,
                    ))))
                } else {
                    let _ = sender.send(msg.get_message().clone());
                    future::ready(Ok(()))
                }
            });
            let status = match res.await {
                Err(e) => {
                    let msg = format!("{:?}", e);
                    RpcStatus::with_message(RpcStatusCode::UNKNOWN, msg)
                }
                Ok(_) => RpcStatus::new(RpcStatusCode::UNKNOWN),
            };
            let _ = sink.fail(status).await;
        });
    }
}

#[derive(Parser, Debug)]
struct Args {
    /// node id
    #[arg(short, long)]
    id: u64,
}

fn main() {
    let decorator = slog_term::TermDecorator::new().build();
    let drain = slog_term::FullFormat::new(decorator).build().fuse();
    let drain = slog_async::Async::new(drain)
        .chan_size(4096)
        .overflow_strategy(slog_async::OverflowStrategy::Block)
        .build()
        .fuse();
    let logger = slog::Logger::root(drain, o!());

    let args = Args::parse();
    let (sender, receiver) = mpsc::channel();
    let node = Arc::new(Mutex::new(match args.id {
        1 => Node::create_raft_leader(args.id, receiver, &logger),
        _ => Node::create_raft_follower(args.id, receiver),
    }));

    let mut t = Instant::now();
    let proposals = Arc::new(Mutex::new(VecDeque::<Proposal>::new()));

    let node_clone = node.clone();
    let proposals_clone = Arc::clone(&proposals);
    let logger_clone = logger.clone();
    thread::spawn(move || loop {
        thread::sleep(Duration::from_millis(10));
        let mut node_clone = node_clone.lock().unwrap();
        loop {
            // Step raft messages.
            match node_clone.my_mailbox.try_recv() {
                Ok(msg) => node_clone.step(msg, &logger_clone),
                Err(TryRecvError::Empty) => break,
                Err(TryRecvError::Disconnected) => return,
            }
        }

        let raft_group = match node_clone.raft_group {
            Some(ref mut r) => r,
            // When Node::raft_group is `None` it means the node is not initialized.
            _ => continue,
        };

        // info!(logger, "main loop");

        if t.elapsed() > Duration::from_millis(100) {
            raft_group.tick();
            t = Instant::now();
        }

        if raft_group.raft.state == StateRole::Leader {
            // info!(logger, "leader branch");
            let mut proposals = proposals_clone.lock().unwrap();
            for p in proposals.iter_mut().skip_while(|p| p.proposed > 0) {
                propose(raft_group, p, &logger_clone);
            }
        }

        on_ready(raft_group, &logger_clone, &proposals_clone)
    });

    let logger_clone = logger.clone();
    if args.id == 1 {
        thread::spawn(move || {
            add_all_followers(proposals.as_ref(), &logger_clone);
        });
    }

    let env = Arc::new(Environment::new(1));
    let s = TikvService {
        node,
        logger,
        sender,
    };
    let service = create_tikv(s);
    let mut server = ServerBuilder::new(env)
        .register_service(service)
        .bind("127.0.0.1", node_id_to_port(args.id))
        .build()
        .unwrap();
    server.start();
    // for (host, port) in server.bind_addrs() {
    //     info!(&logger, "listening on {}:{}", host, port);
    // }
    let (tx, rx) = oneshot::channel();
    thread::spawn(move || {
        // info!(&logger, "Press ENTER to exit...");
        let _ = std::io::stdin().read(&mut [0]).unwrap();
        tx.send(())
    });
    let _ = block_on(rx);
    let _ = block_on(server.shutdown());
}

fn node_id_to_port(id: u64) -> u16 {
    match id {
        1 => 8080,
        2 => 8081,
        3 => 8082,
        _ => 8080,
    }
}

struct Node {
    id: u64,
    raft_group: Option<RawNode<MemStorage>>,
    my_mailbox: Receiver<Message>,
    kv_pairs: HashMap<u16, String>,
}

impl Node {
    fn create_raft_leader(id: u64, my_mailbox: Receiver<Message>, logger: &slog::Logger) -> Self {
        let logger = logger.new(o!("tag" => format!("peer_{}", id)));
        let cfg = Config {
            id,
            ..Default::default()
        };
        let mut s = Snapshot::default();
        s.mut_metadata().index = 1;
        s.mut_metadata().term = 1;
        s.mut_metadata().mut_conf_state().voters = vec![1];
        let storage = MemStorage::new();
        storage.wl().apply_snapshot(s).unwrap();
        let raft_group = Some(RawNode::new(&cfg, storage, &logger).unwrap());
        Node {
            id,
            raft_group,
            my_mailbox,
            kv_pairs: Default::default(),
        }
    }

    fn create_raft_follower(id: u64, my_mailbox: Receiver<Message>) -> Self {
        Node {
            id,
            raft_group: None,
            my_mailbox,
            kv_pairs: Default::default(),
        }
    }

    // Initialize raft for followers.
    fn initialize_raft_from_message(&mut self, msg: &Message, logger: &slog::Logger) {
        if !is_initial_msg(msg) {
            return;
        }
        let cfg = Config {
            id: msg.to,
            ..Default::default()
        };
        let logger = logger.new(o!("tag" => format!("peer_{}", msg.to)));
        let storage = MemStorage::new();
        self.raft_group = Some(RawNode::new(&cfg, storage, &logger).unwrap());
    }

    // Step a raft message, initialize the raft if need.
    fn step(&mut self, msg: Message, logger: &slog::Logger) {
        if self.raft_group.is_none() {
            if is_initial_msg(&msg) {
                self.initialize_raft_from_message(&msg, logger);
            } else {
                return;
            }
        }
        let raft_group = self.raft_group.as_mut().unwrap();
        let _ = raft_group.step(msg);
    }
}

// The message can be used to initialize a raft node or not.
fn is_initial_msg(msg: &Message) -> bool {
    let msg_type = msg.get_msg_type();
    msg_type == MessageType::MsgRequestVote
        || msg_type == MessageType::MsgRequestPreVote
        || (msg_type == MessageType::MsgHeartbeat && msg.commit == 0)
}

fn on_ready(
    raft_group: &mut RawNode<MemStorage>,
    // kv_pairs: &mut HashMap<u16, String>,
    logger: &slog::Logger,
    proposals: &Mutex<VecDeque<Proposal>>,
) {
    if !raft_group.has_ready() {
        return;
    }
    let store = raft_group.raft.raft_log.store.clone();

    // Get the `Ready` with `RawNode::ready` interface.
    let mut ready = raft_group.ready();

    let handle_messages = |msgs: Vec<Message>| {
        for msg in msgs {
            let to_port = node_id_to_port(msg.to);
            // info!(logger, "send message to port: {}", to_port);
            let env = Arc::new(EnvBuilder::new().build());
            let ch = ChannelBuilder::new(env).connect(&format!("127.0.0.1:{}", to_port));
            let client = TikvClient::new(ch);

            let mut req = RaftMessage::default();
            req.set_message(msg);
            let _reply = client.raft_unary(&req).expect("rpc");
        }
    };

    if !ready.messages().is_empty() {
        // Send out the messages come from the node.
        handle_messages(ready.take_messages());
    }

    if *ready.snapshot() != Snapshot::default() {
        let s = ready.snapshot().clone();
        if let Err(e) = store.wl().apply_snapshot(s) {
            error!(
                logger,
                "apply snapshot fail: {:?}, need to retry or panic", e
            );
            return;
        }
    }

    let mut handle_committed_entries =
        |rn: &mut RawNode<MemStorage>, committed_entries: Vec<Entry>| {
            for entry in committed_entries {
                if entry.data.is_empty() {
                    // From new elected leaders.
                    continue;
                }
                if let EntryType::EntryConfChange = entry.get_entry_type() {
                    // For conf change messages, make them effective.
                    let mut cc = ConfChange::default();
                    cc.merge_from_bytes(&entry.data).unwrap();
                    let cs = rn.apply_conf_change(&cc).unwrap();
                    store.wl().set_conf_state(cs);
                } else {
                    // For normal proposals, extract the key-value pair and then
                    // insert them into the kv engine.
                    let _data = std::str::from_utf8(&entry.data).unwrap();
                    let _reg = Regex::new("put ([0-9]+) (.+)").unwrap();
                    // if let Some(caps) = reg.captures(data) {
                    //     kv_pairs.insert(caps[1].parse().unwrap(), caps[2].to_string());
                    // }
                }
                if rn.raft.state == StateRole::Leader {
                    // The leader should response to the clients, tell them if their proposals
                    // succeeded or not.
                    let proposal = proposals.lock().unwrap().pop_front().unwrap();
                    proposal.propose_success.send(true).unwrap();
                }
            }
        };
    // Apply all committed entries.
    handle_committed_entries(raft_group, ready.take_committed_entries());

    if let Err(e) = store.wl().append(ready.entries()) {
        error!(
            logger,
            "persist raft log fail: {:?}, need to retry or panic", e
        );
        return;
    }

    if let Some(hs) = ready.hs() {
        // Raft HardState changed, and we need to persist it.
        store.wl().set_hardstate(hs.clone());
    }

    if !ready.persisted_messages().is_empty() {
        // Send out the persisted messages come from the node.
        handle_messages(ready.take_persisted_messages());
    }

    // Call `RawNode::advance` interface to update position flags in the raft.
    let mut light_rd = raft_group.advance(ready);
    // Update commit index.
    if let Some(commit) = light_rd.commit_index() {
        store.wl().mut_hard_state().set_commit(commit);
    }
    // Send out the messages.
    handle_messages(light_rd.take_messages());
    // Apply all committed entries.
    handle_committed_entries(raft_group, light_rd.take_committed_entries());
    // Advance the apply index.
    raft_group.advance_apply();
}

struct Proposal {
    normal: Option<(u16, String)>, // key is an u16 integer, and value is a string.
    conf_change: Option<ConfChange>, // conf change.
    transfer_leader: Option<u64>,
    // If it's proposed, it will be set to the index of the entry.
    proposed: u64,
    propose_success: SyncSender<bool>,
}

impl Proposal {
    fn conf_change(cc: &ConfChange) -> (Self, Receiver<bool>) {
        let (tx, rx) = mpsc::sync_channel(1);
        let proposal = Proposal {
            normal: None,
            conf_change: Some(cc.clone()),
            transfer_leader: None,
            proposed: 0,
            propose_success: tx,
        };
        (proposal, rx)
    }

    fn normal(key: u16, value: String) -> (Self, Receiver<bool>) {
        let (tx, rx) = mpsc::sync_channel(1);
        let proposal = Proposal {
            normal: Some((key, value)),
            conf_change: None,
            transfer_leader: None,
            proposed: 0,
            propose_success: tx,
        };
        (proposal, rx)
    }
}

fn propose(raft_group: &mut RawNode<MemStorage>, proposal: &mut Proposal, logger: &slog::Logger) {
    let last_index1 = raft_group.raft.raft_log.last_index() + 1;
    if let Some((ref key, ref value)) = proposal.normal {
        let data = format!("put {} {}", key, value).into_bytes();
        let _ = raft_group.propose(vec![], data);
    } else if let Some(ref cc) = proposal.conf_change {
        info!(logger, "before propose conf change");
        let _ = raft_group.propose_conf_change(vec![], cc.clone());
    } else if let Some(_transferee) = proposal.transfer_leader {
        // TODO: implement transfer leader.
        unimplemented!();
    }

    let last_index2 = raft_group.raft.raft_log.last_index() + 1;
    if last_index2 == last_index1 {
        // Propose failed, don't forget to respond to the client.
        proposal.propose_success.send(false).unwrap();
    } else {
        proposal.proposed = last_index1;
    }
}

// Proposes some conf change for peers [2, 5].
fn add_all_followers(proposals: &Mutex<VecDeque<Proposal>>, logger: &slog::Logger) {
    for i in 2..4u64 {
        let mut conf_change = ConfChange {
            node_id: i,
            ..Default::default()
        };
        conf_change.set_change_type(ConfChangeType::AddNode);
        loop {
            info!(logger, "before send proposal");
            let (proposal, rx) = Proposal::conf_change(&conf_change);
            proposals.lock().unwrap().push_back(proposal);
            if rx.recv().unwrap() {
                break;
            }
            thread::sleep(Duration::from_millis(100));
            info!(logger, "after send proposal");
        }
    }
}
