use rand::Rng;

use crate::{error::Result, raft::message::Message};
use log::info;

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
        let mut node_state = self.become_role(Candidate::new())?;
        node_state.start_election().await?;
        Ok(node_state)
    }

    pub async fn tick(mut self) -> Result<Node> {
        self.role.no_viable_leader_ticks += 1;
        if self.role.no_viable_leader_ticks >= self.role.election_timeout_ticks {
            Ok(self.become_cadicate().await?.into())
        } else {
            Ok(self.into())
        }
    }

    pub async fn handle_message(self, message: Message) {}
}
