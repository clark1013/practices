use super::message::Message;
use super::node::Node;
use crate::error::Result;
use futures::prelude::*;
use log::{debug, error};
use std::{collections::HashMap, time::Duration};
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::mpsc;
use tokio_serde::formats::SymmetricalBincode;
use tokio_util::codec::{FramedRead, FramedWrite, LengthDelimitedCodec};

const TICK: Duration = Duration::from_millis(100);

pub struct RaftServer {
    node: Node,
    peers: HashMap<String, String>,
    node_rx: mpsc::Receiver<Message>,
}

impl RaftServer {
    pub async fn new(id: &str, peers: HashMap<String, String>) -> Result<Self> {
        let (node_tx, node_rx) = mpsc::channel(32);
        let node = Node::new(
            id,
            peers.iter().map(|(k, _v)| k.to_string()).collect(),
            node_tx,
        )
        .await?;
        Ok(RaftServer {
            node,
            peers,
            node_rx,
        })
    }

    pub async fn serve(mut self, listener: TcpListener) -> Result<()> {
        let mut ticker = tokio::time::interval(TICK);
        let (tcp_recv_tx, mut tcp_recv_rx) = mpsc::channel::<Message>(32);
        loop {
            tokio::select! {
                // send message to peers
                Some(message) = self.node_rx.recv() => {
                    let addr = self.peers.get(&message.to);
                    match addr {
                        Some(addr) => {
                            tokio::spawn(Self::tcp_send(addr.clone(), message));
                        }
                        None => error!("invalid message.to={}", &message.to),
                    }

                },
                // receive message from peers
                Some(message) = tcp_recv_rx.recv() => {
                    debug!("received message from {}: {:?}", message.from, message);
                }
                _ = ticker.tick() =>  {
                    self.node = self.node.tick().await?;
                },
                Ok((socket, _)) = listener.accept()=> {
                    tokio::spawn(Self::tcp_recv(socket, tcp_recv_tx.clone()));
                },
            }
        }
    }

    pub async fn tcp_send(addr: String, message: Message) -> Result<()> {
        let sokect = TcpStream::connect(addr).await?;
        let length_delimited = FramedWrite::new(sokect, LengthDelimitedCodec::new());
        let mut serialized =
            tokio_serde::SymmetricallyFramed::new(length_delimited, SymmetricalBincode::default());
        serialized.send(message).await?;
        Ok(())
    }

    pub async fn tcp_recv(socket: TcpStream, tcp_recv_rx: mpsc::Sender<Message>) -> Result<()> {
        let length_delimited = FramedRead::new(socket, LengthDelimitedCodec::new());
        let mut deserialized = tokio_serde::SymmetricallyFramed::new(
            length_delimited,
            SymmetricalBincode::<Message>::default(),
        );
        while let Some(message) = deserialized.try_next().await.unwrap() {
            tcp_recv_rx.send(message).await.unwrap();
        }
        Ok(())
    }
}
