use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct RequestCreateSessionParams {
    pub unk1: u32,
    pub unk2: u32,
    pub unk3: u32,
    pub game_version: u32,
    pub unk4: u32,
    pub unk5: u32,
    pub steam_ticket: Vec<u8>,
    pub unk6: u32,
    pub unk7: u32,
    pub unk8: u32,
    pub unk9: u32,
    pub unk10: u32,
    pub unk11: u32,
    pub unk12: u32,
    pub unk13: u32,
    pub unk14: u16,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ResponseCreateSessionParams {
    pub player_id: i32,
    pub steam_id: String,
    pub ip_address: String,
    pub unk: u32,
    pub session_id: i32,
    pub valid_from: u64,
    pub valid_until: u64,
    pub cookie: String,
    pub unk_string: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RequestRestoreSessionParams {
    pub game_version: u32,
    /*
    pub unk1: u32,
    pub unk2: u32,
    pub steam_ticket: Vec<u8>,
    pub unk: u32,
    pub session_id: i32,
    pub valid_from: u64,
    pub valid_until: u64,
    pub cookie: String,
    pub unk_string: String,
    */
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ResponseRestoreSessionParams {
    pub unk: u32,
    pub session_id: i32,
    pub valid_from: u64,
    pub valid_until: u64,
    pub cookie: String,
    pub unk_string: String,
}

#[cfg(test)]
mod tests {
    use crate::session::*;
    use crate::Serializer;

    #[test]
    fn response_create_session_serialize() {
        let mut buf: Vec<u8> = Vec::new();

        let params = ResponseCreateSessionParams {
            player_id: 288724,
            steam_id: String::from("01100001023a7e18"),
            ip_address: String::from("99.99.999.999"),
            unk: 2131458904,
            session_id: 291891302,
            valid_from: 1695716746,
            valid_until: 1695720346,
            cookie: String::from(
                "a6ab6316a2e7683db73c6fe281b280e251f1756458bad659428799adcfbffc4e",
            ),
            unk_string: String::from(""),
        };

        params.serialize(&mut Serializer::new(&mut buf)).unwrap();

        assert_eq!(
            buf,
            include_bytes!("../../test/params/ResponseCreateSession.bin")
        );
    }
}
