use std::collections::HashMap;
use tokio::net::TcpListener;

use crate::{
    error::{Error, Result},
    raft::{self, RaftServer},
    storage,
};

pub struct Server {
    raft: raft::RaftServer,
    raft_listener: Option<TcpListener>,
}

impl Server {
    pub async fn new(
        id: &str,
        peers: HashMap<String, String>,
        raft_store: Box<dyn storage::log::LogStore>,
    ) -> Result<Self> {
        Ok(Server {
            raft: RaftServer::new(id, peers, raft_store).await?,
            raft_listener: None,
        })
    }

    pub async fn listen(mut self, raft_addr: &str) -> Result<Self> {
        let raft_listener = TcpListener::bind(raft_addr).await?;
        self.raft_listener = Some(raft_listener);
        Ok(self)
    }

    pub async fn serve(self) -> Result<()> {
        let raft_listener = self
            .raft_listener
            .ok_or_else(|| Error::Internal("Must listen before serve".to_string()))?;
        self.raft.serve(raft_listener).await?;
        Ok(())
    }
}
