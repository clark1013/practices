use log::info;
use rand::Rng;

use crate::{
    error::Result,
    raft::message::{Event, Message},
};

use super::{Node, NodeState, ELECTION_TIMEOUT_MAX, ELECTION_TIMEOUT_MIN};

pub struct Candidate {
    wait_election_ticks: u64,
    wait_election_timout_ticks: u64,
}

impl Candidate {
    pub fn new() -> Self {
        Candidate {
            wait_election_ticks: 0,
            wait_election_timout_ticks: rand::thread_rng()
                .gen_range(ELECTION_TIMEOUT_MIN..ELECTION_TIMEOUT_MAX),
        }
    }
}

impl NodeState<Candidate> {
    pub async fn start_election(&mut self) -> Result<()> {
        self.current_term += 1;
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

    pub async fn handle_message(self, message: Message) {}
}
