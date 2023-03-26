use futures::prelude::*;
use std::{collections::HashMap, time::Duration};

use tokio::net::{TcpListener, TcpStream};
use tokio_serde::formats::SymmetricalBincode;
use tokio_util::codec::{FramedRead, FramedWrite, LengthDelimitedCodec};

use crate::error::Result;

use super::message::Message;
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
                _ = ticker.tick() =>  {
                    for v in self.peers.values() {
                        let addr = v.clone();
                        tokio::spawn(Self::tcp_send(self.node.id(), addr));
                    }
                },
                Ok((socket, _)) = listener.accept()=> {
                    tokio::spawn(Self::tcp_recv(socket));
                },
            }
        }
    }

    pub async fn tcp_send(id: String, addr: String) -> Result<()> {
        let sokect = TcpStream::connect(addr).await?;
        let length_delimited = FramedWrite::new(sokect, LengthDelimitedCodec::new());
        let mut serialized =
            tokio_serde::SymmetricallyFramed::new(length_delimited, SymmetricalBincode::default());
        let message = Message { from: id };
        serialized.send(message).await?;
        Ok(())
    }

    pub async fn tcp_recv(socket: TcpStream) -> Result<()> {
        let length_delimited = FramedRead::new(socket, LengthDelimitedCodec::new());
        let mut deserialized = tokio_serde::SymmetricallyFramed::new(
            length_delimited,
            SymmetricalBincode::<Message>::default(),
        );
        while let Some(msg) = deserialized.try_next().await.unwrap() {
            println!("GOT {:?}", msg)
        }
        Ok(())
    }
}
