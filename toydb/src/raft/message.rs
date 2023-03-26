use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Message {
    pub from: String,
    pub to: String,
    pub event: Event,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Event {
    RequestVoteReq { term: u64, candidate_id: String },
    RequestVoteResp { term: u64, vote_granted: bool },
}
