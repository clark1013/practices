mod candidate;
mod follower;
mod leader;

use candidate::Candidate;
use follower::Follower;
use leader::Leader;

pub enum Node {
    Leader(NodeRole<Leader>),
    Candidate(NodeRole<Candidate>),
    Follower(NodeRole<Follower>),
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
    use crate::error::Result;
    #[tokio::test]
    async fn new() -> Result<()> {
        assert!(1 == 1);
        Ok(())
    }
}
