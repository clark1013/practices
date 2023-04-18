mod candidate;
mod follower;
mod leader;

use super::message::Message;
use crate::{error::Result, storage};
use candidate::Candidate;
use follower::Follower;
use leader::Leader;
use tokio::sync::mpsc;

/// The interval between leader heartbeats, in ticks.
const HEARTBEAT_INTERVAL: u64 = 1;

/// The minimum election timeout, in ticks.
const ELECTION_TIMEOUT_MIN: u64 = 8 * HEARTBEAT_INTERVAL;

/// The maximum election timeout, in ticks.
const ELECTION_TIMEOUT_MAX: u64 = 15 * HEARTBEAT_INTERVAL;

pub enum Node {
    Leader(NodeState<Leader>),
    Candidate(NodeState<Candidate>),
    Follower(NodeState<Follower>),
}

impl Node {
    pub async fn new(
        id: &str,
        peers: Vec<String>,
        node_tx: mpsc::Sender<Message>,
        raft_store: Box<dyn storage::log::LogStore>,
    ) -> Result<Self> {
        let node_state = NodeState {
            id: id.to_owned(),
            peers,
            current_term: 0,
            voted_for: None,
            role: Follower::new(),
            node_tx,
            log_store: raft_store,
        };
        Ok(node_state.into())
    }

    pub fn id(&self) -> String {
        match self {
            Node::Leader(ns) => ns.id.clone(),
            Node::Candidate(ns) => ns.id.clone(),
            Node::Follower(ns) => ns.id.clone(),
        }
    }

    pub async fn tick(self) -> Result<Self> {
        match self {
            Node::Leader(ns) => ns.tick().await,
            Node::Candidate(ns) => ns.tick().await,
            Node::Follower(ns) => ns.tick().await,
        }
    }

    pub async fn handle_message(self, message: Message) -> Result<Self> {
        match self {
            Node::Leader(ns) => ns.handle_message(message).await,
            Node::Candidate(ns) => ns.handle_message(message).await,
            Node::Follower(ns) => ns.handle_message(message).await,
        }
    }
}

impl From<NodeState<Follower>> for Node {
    fn from(node_state: NodeState<Follower>) -> Self {
        Node::Follower(node_state)
    }
}

impl From<NodeState<Leader>> for Node {
    fn from(node_state: NodeState<Leader>) -> Self {
        Node::Leader(node_state)
    }
}

impl From<NodeState<Candidate>> for Node {
    fn from(node_state: NodeState<Candidate>) -> Self {
        Node::Candidate(node_state)
    }
}

pub struct NodeState<R> {
    id: String,
    peers: Vec<String>,
    current_term: u64,
    voted_for: Option<String>,
    role: R,
    // log_store saves persistent log entries
    log_store: Box<dyn storage::log::LogStore>,
    // the channel is used for send message to other nodes
    node_tx: mpsc::Sender<Message>,
}

impl<R> NodeState<R> {
    pub fn become_role<T>(self, role: T) -> Result<NodeState<T>> {
        Ok(NodeState {
            id: self.id,
            peers: self.peers,
            current_term: self.current_term,
            voted_for: self.voted_for,
            role,
            node_tx: self.node_tx,
            log_store: self.log_store,
        })
    }

    fn quorum(&self) -> u64 {
        (self.peers.len() as u64 + 1) / 2 + 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{error::Result, storage::log::MemoryLogStore};
    #[tokio::test]
    async fn new() -> Result<()> {
        let (node_tx, _) = mpsc::channel(1);
        let node = Node::new(
            "a",
            vec!["b".to_owned(), "c".to_owned()],
            node_tx,
            Box::new(MemoryLogStore::new()),
        )
        .await?;
        match node {
            Node::Follower(node_state) => {
                assert_eq!(node_state.id, "a");
            }
            _ => panic!("Expected node start as follower"),
        }
        Ok(())
    }
}
