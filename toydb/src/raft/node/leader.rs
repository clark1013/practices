use crate::{error::Result, raft::message::Message};

use super::{Node, NodeState};

pub struct Leader {}

impl NodeState<Leader> {
    pub async fn tick(self) -> Result<Node> {
        Ok(self.into())
    }

    pub async fn handle_message(self, message: Message) {}
}
