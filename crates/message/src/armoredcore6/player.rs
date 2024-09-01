use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ResponseGetRatingStatus {
    pub unk6: u32,
}
