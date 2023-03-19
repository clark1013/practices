use std::collections::HashMap;

use crate::{
    error::Result,
    raft::{self, RaftServer},
};

pub struct Server {
    raft: raft::RaftServer,
}

impl Server {
    pub async fn new(id: &str, peers: HashMap<String, String>) -> Result<Self> {
        return Ok(Server {
            raft: RaftServer::new(id, peers),
        });
    }
}
