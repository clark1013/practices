use std::{collections::HashMap, time::Duration};

use tokio::net::TcpListener;

use crate::{error::Result, raft::node::NodeRole};

use super::node::Node;

const TICK: Duration = Duration::from_secs(1);

pub struct RaftServer {
    node: Node,
    peers: HashMap<String, String>,
}

impl RaftServer {
    pub async fn new(id: &str, peers: HashMap<String, String>) -> Result<Self> {
        let node = Node::new(id, peers.iter().map(|(k, _v)| k.to_string()).collect()).await?;
        Ok(RaftServer { node, peers })
    }

    pub async fn serve(self, listener: TcpListener) -> Result<()> {
        let mut ticker = tokio::time::interval(TICK);
        loop {
            tokio::select! {
                _ = ticker.tick() =>  println!("TICK {:?}", self.peers),
                Ok((stream, _)) = listener.accept() => println!("{:?}", stream),
            }
        }
    }
}
