use log::error;
use serde_json::Deserializer;
use std::{
    io::{BufReader, BufWriter},
    net::TcpListener,
};

use crate::{
    common::{GetResponse, RemoveResponse, Request, SetResponse},
    KvsEngine, Result,
};

#[derive(Debug)]
pub struct KvsServer<E: KvsEngine> {
    addr: String,
    engine: E,
}

impl<E: KvsEngine> KvsServer<E> {
    pub fn new(addr: String, engine: E) -> Self {
        KvsServer { addr, engine }
    }

    pub fn run(&mut self) -> Result<()> {
        let listener = TcpListener::bind(&self.addr)?;
        for stream in listener.incoming() {
            match stream {
                Ok(stream) => {
                    let reader = BufReader::new(&stream);
                    let req_reader = Deserializer::from_reader(reader).into_iter::<Request>();
                    for req in req_reader {
                        let writer = BufWriter::new(&stream);
                        match req {
                            Ok(req) => match req {
                                Request::Get { key } => {
                                    let value = self.engine.get(key);
                                    match value {
                                        Ok(value) => {
                                            let r = GetResponse::Ok(value);
                                            serde_json::to_writer(writer, &r)?;
                                        }
                                        Err(e) => {
                                            let r = GetResponse::Err(format!("{}", e));
                                            serde_json::to_writer(writer, &r)?;
                                        }
                                    }
                                }
                                Request::Set { key, value } => {
                                    let s = self.engine.set(key, value);
                                    match s {
                                        Ok(()) => {
                                            let r = SetResponse::Ok(());
                                            serde_json::to_writer(writer, &r)?;
                                        }
                                        Err(e) => {
                                            let r = SetResponse::Err(format!("{}", e));
                                            serde_json::to_writer(writer, &r)?;
                                        }
                                    }
                                }
                                Request::Remove { key } => {
                                    let s = self.engine.remove(key);
                                    match s {
                                        Ok(()) => {
                                            let r = RemoveResponse::Ok(());
                                            serde_json::to_writer(writer, &r)?;
                                        }
                                        Err(e) => {
                                            let r = RemoveResponse::Err(format!("{}", e));
                                            serde_json::to_writer(writer, &r)?;
                                        }
                                    }
                                }
                            },
                            Err(e) => error!("received invalid request: {}", e),
                        }
                    }
                }
                Err(e) => error!("Connection failed: {}", e),
            }
        }
        Ok(())
    }
}
