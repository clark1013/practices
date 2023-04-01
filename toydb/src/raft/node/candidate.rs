use log::{error, info};
use rand::Rng;

use crate::{
    error::Result,
    raft::message::{Event, Message},
};

use super::{
    follower::Follower, leader::Leader, Node, NodeState, ELECTION_TIMEOUT_MAX, ELECTION_TIMEOUT_MIN,
};

pub struct Candidate {
    wait_election_ticks: u64,
    wait_election_timout_ticks: u64,
    votes: u64,
}

impl Candidate {
    pub fn new() -> Self {
        Candidate {
            wait_election_ticks: 0,
            wait_election_timout_ticks: rand::thread_rng()
                .gen_range(ELECTION_TIMEOUT_MIN..ELECTION_TIMEOUT_MAX),
            votes: 0,
        }
    }
}

impl NodeState<Candidate> {
    fn become_leader(self) -> Result<NodeState<Leader>> {
        info!("candidate -> leader, term: {}", self.current_term);
        self.become_role(Leader::new())
    }

    fn become_follwer(self) -> Result<NodeState<Follower>> {
        info!("candidate -> follower, term: {}", self.current_term);
        self.become_role(Follower::new())
    }

    pub async fn start_election(&mut self) -> Result<()> {
        self.current_term += 1;
        self.voted_for = Some(self.id.clone());
        self.role.votes += 1;
        for peer in &self.peers {
            self.node_tx
                .send(Message {
                    from: self.id.clone(),
                    to: peer.clone(),
                    event: Event::RequestVoteReq {
                        term: self.current_term,
                        candidate_id: self.id.clone(),
                    },
                })
                .await
                .unwrap();
        }
        Ok(())
    }

    pub async fn tick(mut self) -> Result<Node> {
        self.role.wait_election_ticks += 1;
        if self.role.wait_election_ticks >= self.role.wait_election_timout_ticks {
            info!(
                "wait election timeout, start a new round, term: {}",
                self.current_term
            );
            self.role = Candidate::new();
            self.start_election().await?;
        }
        Ok(self.into())
    }

    pub async fn handle_message(mut self, message: Message) -> Result<Node> {
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
            Event::RequestVoteResp { term, vote_granted } => {
                if term < self.current_term {
                    return Ok(self.into());
                }
                if vote_granted {
                    self.role.votes += 1;
                }
                if self.role.votes >= self.quorum() {
                    let leader = self.become_leader().unwrap();
                    return Ok(leader.into());
                }
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
                info!("candidate received AppendEntriesResp");
            }
        }
        Ok(self.into())
    }
}
