use crate::{
    error::Result,
    raft::{
        message::{Event, Message},
        node::follower::Follower,
    },
};
use log::{error, info};

use super::{Node, NodeState, HEARTBEAT_INTERVAL};

pub struct Leader {
    heartbeat_ticks: u64,
}

impl Leader {
    pub fn new() -> Self {
        Leader { heartbeat_ticks: 0 }
    }
}

impl NodeState<Leader> {
    fn become_follwer(self) -> Result<NodeState<Follower>> {
        info!("candidate -> follower, term: {}", self.current_term);
        self.become_role(Follower::new())
    }

    pub async fn tick(mut self) -> Result<Node> {
        self.role.heartbeat_ticks += 1;
        if self.role.heartbeat_ticks >= HEARTBEAT_INTERVAL {
            for peer in &self.peers {
                self.node_tx
                    .send(Message {
                        from: self.id.clone(),
                        to: peer.clone(),
                        event: Event::AppendEntriesReq {
                            term: self.current_term,
                            leader_id: self.id.clone(),
                        },
                    })
                    .await
                    .unwrap();
            }
            self.role.heartbeat_ticks = 0;
        }
        Ok(self.into())
    }

    pub async fn handle_message(self, message: Message) -> Result<Node> {
        if message.to != self.id {
            error!("message.to is invalid, to={}", message.to);
            return Ok(self.into());
        }
        match message.event {
            Event::RequestVoteReq {
                term,
                ref candidate_id,
            } => {
                if term > self.current_term {
                    let mut follower = self.become_follwer().unwrap();
                    follower.voted_for = None;
                    follower.current_term = term;
                    return follower.handle_message(message).await;
                }
            }
            Event::RequestVoteResp { .. } => {
                info!("leader received RequestVoteResp");
            }
            Event::AppendEntriesReq {
                term,
                ref leader_id,
            } => {
                if term > self.current_term {
                    let mut follower = self.become_follwer().unwrap();
                    follower.voted_for = Some(leader_id.clone());
                    follower.current_term = term;
                    return follower.handle_message(message).await;
                }
            }
            Event::AppendEntriesResp { .. } => {
                info!("leader received AppendEntriesResp");
            }
        }
        Ok(self.into())
    }
}
