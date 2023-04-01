use rand::Rng;

use crate::{
    error::Result,
    raft::message::{Event, Message},
};
use log::{error, info};

use super::{candidate::Candidate, Node, NodeState, ELECTION_TIMEOUT_MAX, ELECTION_TIMEOUT_MIN};

pub struct Follower {
    no_viable_leader_ticks: u64,
    // If a follower receives no communication over a period of time called `election timeout`,
    // then it assumes there is no viable leader and begins an election to choose a new leader.
    election_timeout_ticks: u64,
}

impl Follower {
    pub fn new() -> Self {
        Follower {
            no_viable_leader_ticks: 0,
            election_timeout_ticks: rand::thread_rng()
                .gen_range(ELECTION_TIMEOUT_MIN..ELECTION_TIMEOUT_MAX),
        }
    }
}

impl NodeState<Follower> {
    async fn become_cadicate(self) -> Result<NodeState<Candidate>> {
        info!("follower -> candidate, term: {}", self.current_term);
        let mut candidate = self.become_role(Candidate::new())?;
        candidate.start_election().await?;
        Ok(candidate)
    }

    pub async fn tick(mut self) -> Result<Node> {
        self.role.no_viable_leader_ticks += 1;
        if self.role.no_viable_leader_ticks >= self.role.election_timeout_ticks {
            Ok(self.become_cadicate().await?.into())
        } else {
            Ok(self.into())
        }
    }

    pub async fn handle_message(mut self, message: Message) -> Result<Node> {
        if message.to != self.id {
            error!("message.to is invalid, to={}", message.to);
            return Ok(self.into());
        }
        match message.event {
            Event::RequestVoteReq { term, candidate_id } => {
                if term < self.current_term {
                    return Ok(self.into());
                }
                if term == self.current_term && self.voted_for.is_some() {
                    return Ok(self.into());
                }
                self.node_tx
                    .send(Message {
                        from: self.id.clone(),
                        to: candidate_id.clone(),
                        event: Event::RequestVoteResp {
                            term,
                            vote_granted: true,
                        },
                    })
                    .await
                    .unwrap();
                self.voted_for = Some(candidate_id);
                self.current_term = term;
            }
            Event::RequestVoteResp { .. } => {
                info!("follower received RequestVoteResp")
            }
            Event::AppendEntriesReq { term, leader_id } => {
                if term < self.current_term {
                    return Ok(self.into());
                }
                // TODO: should the follower change its voted_for?
                self.voted_for = Some(leader_id);
                self.current_term = term;
                self.role.no_viable_leader_ticks = 0;
            }
            Event::AppendEntriesResp { .. } => {
                info!("follower received AppendEntriesResp")
            }
        }
        Ok(self.into())
    }
}
