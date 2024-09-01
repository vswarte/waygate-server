use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ResponsePollMatchingTicketParams {
    pub unk0: u32,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RequestCreateMatchingTicketParams {
    pub unk1: Vec<u32>,
    pub unk2: u32,
}
