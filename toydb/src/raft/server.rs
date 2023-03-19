use std::collections::HashMap;

use crate::error::Result;

pub struct RaftServer {
    peers: HashMap<String, String>,
}

impl RaftServer {
    pub fn new(id: &str, peers: HashMap<String, String>) -> Result<Self> {
        return Ok(RaftServer { peers });
    }
}
