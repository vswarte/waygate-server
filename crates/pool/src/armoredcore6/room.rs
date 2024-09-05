use std::sync::{atomic::{AtomicU32, Ordering}, RwLock};

use waygate_message::armoredcore6::{MatchFormat, MemberRotation, RequestCreateRoomParams, RequestGetRoomListParams, ResponseGetRoomListParamsEntry, RoomDetails};

use crate::{MatchResult, PoolQuery};

pub fn normalize_room_keyword<S: AsRef<str>>(input: S) -> String {
    let bytes = input.as_ref().as_bytes();
    match bytes.iter().position(|&b| b == 0x0) {
        Some(end) => String::from_utf8_lossy(&bytes[..end]).to_string(),
        None => input.as_ref().to_string(),
    }
}

#[derive(Debug, Default)]
pub struct RoomPoolEntry {
    pub room_details: RwLock<RoomDetails>,
    pub players_in_room: AtomicU32,
    pub room_data: Vec<u8>,
}

impl From<Box<RequestCreateRoomParams>> for RoomPoolEntry {
    fn from(value: Box<RequestCreateRoomParams>) -> Self {
        // Normalize input because the client just sends in the keyword with a
        // fixed size lenght of 33?
        let mut room_details = value.room_details;
        room_details.keywords = normalize_room_keyword(room_details.keywords.as_str());

        Self {
            players_in_room: AtomicU32::new(1),
            room_details: RwLock::new(room_details),
            room_data: value.room_data,
        }
    }
}

impl Into<ResponseGetRoomListParamsEntry> for &MatchResult<RoomPoolEntry> {
    fn into(self) -> ResponseGetRoomListParamsEntry {
        let room_details = self.1.room_details.read()
            .unwrap()
            .clone();

        ResponseGetRoomListParamsEntry {
            players_in_room: self.1.players_in_room.load(Ordering::Relaxed),
            room_details,
            unk10: 0,
            room_data: self.1.room_data.clone(),
        }
    }
}

#[derive(Debug)]
pub struct RoomPoolQuery {
    room_details: RoomDetails,
}

impl PoolQuery<RoomPoolEntry> for RoomPoolQuery {
    fn matches(&self, entry: &RoomPoolEntry) -> bool {
        let entry_details = entry.room_details.read().unwrap().clone();

        if self.room_details.player_capacity != 0
            && self.room_details.player_capacity != entry_details.player_capacity {
            println!("Player cap is wrong");
            return false
        }

        if self.room_details.match_format != MatchFormat::All
            && self.room_details.match_format != entry_details.match_format {
            println!("Match format is wrong");
            return false
        }

        if self.room_details.time_limit != 0
            && self.room_details.time_limit != entry_details.time_limit {
            println!("Time limit is wrong");
            return false
        }

        if self.room_details.map != 0
            && self.room_details.map != 10000
            && self.room_details.map != entry_details.map {
            println!("Map is wrong");
            return false
        }

        if self.room_details.member_rotation != MemberRotation::All
            && self.room_details.member_rotation != entry_details.member_rotation {
            println!("Member rotation is wrong");
            return false
        }

        if entry.players_in_room.load(Ordering::Relaxed) >= entry_details.player_capacity {
            return false;
        }

        if self.room_details.keywords.as_str() != entry_details.keywords.as_str() {
            println!("Pasword is wrong");
            return false
        }

        true
    }
}

impl From<Box<RequestGetRoomListParams>> for RoomPoolQuery {
    fn from(value: Box<RequestGetRoomListParams>) -> Self {
        RoomPoolQuery {
            room_details: value.room_details,
        }
    }
}

#[cfg(test)]
mod test {
}
