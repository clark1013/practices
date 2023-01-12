use log::error;
use serde_json::Deserializer;
use std::{
    io::{BufReader, BufWriter, Read, Write},
    net::TcpListener,
};

use crate::{common::Request, KvsEngine, Result};

#[derive(Debug)]
pub struct KvsServer<E: KvsEngine> {
    addr: String,
    engine: E,
}

impl<E: KvsEngine> KvsServer<E> {
    pub fn new(addr: String, engine: E) -> Self {
        KvsServer { addr, engine }
    }

    pub fn run(&self) -> Result<()> {
        let listener = TcpListener::bind(&self.addr)?;
        for stream in listener.incoming() {
            match stream {
                Ok(stream) => {
                    let writer = BufWriter::new(&stream);
                    let r = Request::Get {
                        key: "123".to_string(),
                    };
                    serde_json::to_writer(writer, &r)?;
                    let reader = BufReader::new(&stream);
                    let req_reader = Deserializer::from_reader(reader).into_iter::<Request>();
                    for req in req_reader {
                        println!("{:?}", req);
                    }
                }
                Err(e) => error!("Connection failed: {}", e),
            }
        }
        Ok(())
    }
}
