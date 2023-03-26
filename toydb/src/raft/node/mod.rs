mod candidate;
mod follower;
mod leader;

use candidate::Candidate;
use follower::Follower;
use leader::Leader;

use crate::error::Result;

pub enum Node {
    Leader(NodeRole<Leader>),
    Candidate(NodeRole<Candidate>),
    Follower(NodeRole<Follower>),
}

impl Node {
    pub async fn new(id: &str, peers: Vec<String>) -> Result<Self> {
        let node_role = NodeRole {
            id: id.to_owned(),
            peers,
            current_term: 0,
            role: Follower::new(),
        };
        Ok(node_role.into())
    }

    pub fn id(&self) -> String {
        match self {
            Node::Leader(nr) => nr.id.clone(),
            Node::Candidate(nr) => nr.id.clone(),
            Node::Follower(nr) => nr.id.clone(),
        }
    }
}

impl From<NodeRole<Follower>> for Node {
    fn from(node_role: NodeRole<Follower>) -> Self {
        Node::Follower(node_role)
    }
}

pub struct NodeRole<R> {
    id: String,
    peers: Vec<String>,
    current_term: u64,
    role: R,
}

impl<R> NodeRole<R> {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::error::Result;
    #[tokio::test]
    async fn new() -> Result<()> {
        let node = Node::new("a", vec!["b".to_owned(), "c".to_owned()]).await?;
        match node {
            Node::Follower(node_role) => {
                assert_eq!(node_role.id, "a");
            }
            _ => panic!("Expected node start as follower"),
        }
        Ok(())
    }
}
