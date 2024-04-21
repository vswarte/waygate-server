use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RequestCreateMatchingTicketParams {
    pub unk1: Vec<u32>,
    pub unk2: u32,
}
