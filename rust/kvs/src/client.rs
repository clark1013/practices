use super::error::Result;
use std::{
    io::{BufReader, BufWriter, Write},
    net::TcpStream,
};

use clap::Error;
use log::error;
use serde::Deserialize;
use serde_json::{de::IoRead, Deserializer};

use crate::{
    common::{GetResponse, RemoveResponse, Request, SetResponse},
    error::KvsError,
};

pub struct Client {
    writer: BufWriter<TcpStream>,
    reader: Deserializer<IoRead<BufReader<TcpStream>>>,
}

impl Client {
    pub fn new(addr: String) -> Self {
        // let tcp_reader = TcpStream::connect(&addr).unwrap();
        // let tcp_writer = tcp_reader.try_clone().unwrap();
        let tcp_writer = TcpStream::connect(&addr).unwrap();
        let tcp_reader = tcp_writer.try_clone().unwrap();
        Client {
            writer: BufWriter::new(tcp_writer),
            reader: Deserializer::from_reader(BufReader::new(tcp_reader)),
        }
    }

    pub fn get(&mut self, key: String) -> Result<()> {
        let req = Request::Get { key };
        serde_json::to_writer(&mut self.writer, &req)?;
        self.writer.flush()?;
        let resp = GetResponse::deserialize(&mut self.reader)?;
        match resp {
            GetResponse::Ok(value) => match value {
                Some(v) => println!("{}", v),
                None => println!("Key not found"),
            },
            GetResponse::Err(e) => {
                error!("{}", e);
                return Err(KvsError::KeyNotFound);
            }
        }
        Ok(())
    }

    pub fn set(&mut self, key: String, value: String) -> Result<()> {
        let req = Request::Set { key, value };
        serde_json::to_writer(&mut self.writer, &req)?;
        self.writer.flush()?;
        let resp = SetResponse::deserialize(&mut self.reader)?;
        match resp {
            SetResponse::Ok(_value) => (),
            SetResponse::Err(e) => {
                error!("{}", e);
                return Err(KvsError::KeyNotFound);
            }
        }
        Ok(())
    }

    pub fn remove(&mut self, key: String) -> Result<()> {
        let req = Request::Remove { key };
        serde_json::to_writer(&mut self.writer, &req)?;
        self.writer.flush()?;
        let resp = RemoveResponse::deserialize(&mut self.reader)?;
        match resp {
            RemoveResponse::Ok(_value) => (),
            RemoveResponse::Err(e) => {
                error!("{}", e);
                return Err(KvsError::KeyNotFound);
            }
        }
        Ok(())
    }
}
