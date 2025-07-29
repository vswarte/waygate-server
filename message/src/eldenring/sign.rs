use serde::{Deserialize, Serialize};

use super::shared::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct RequestCreateSignParams {
    pub area: PlayRegionArea,
    pub matching_parameters: MatchingParameters,
    pub unk0: u32, // Could be sign type although this is also embedded in the sign data itself
    pub data: Vec<u8>,
    pub group_passwords: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ResponseCreateSignParams {
    pub identifier: ObjectIdentifier,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RequestGetSignListParams {
    pub known_signs: Vec<ObjectIdentifier>,
    pub search_areas: Vec<PlayRegionArea>,
    pub matching_parameters: MatchingParameters,
    // TODO: rest
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ResponseGetSignListParamsEntry {
    pub player_id: i32,
    pub identifier: ObjectIdentifier,
    pub area: PlayRegionArea,
    pub data: Vec<u8>,
    pub external_id: String,
    pub unk_string: String,
    pub group_passwords: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ResponseGetSignListParams {
    pub known_signs: Vec<ObjectIdentifier>,
    pub entries: Vec<ResponseGetSignListParamsEntry>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RequestSummonSignParams {
    pub player_id: i32,
    pub identifier: ObjectIdentifier,
    pub data: Vec<u8>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ResponseSummonSignParams {}

#[derive(Serialize, Deserialize, Debug)]
pub struct RequestRejectSignParams {
    pub sign_identifier: ObjectIdentifier,
    pub summoning_player_id: i32,
    pub reason: i32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ResponseRejectSignParams {}

#[derive(Serialize, Deserialize, Debug)]
pub struct RequestRemoveSignParams {
    pub sign_identifier: ObjectIdentifier,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ResponseRemoveSignParams {}

#[derive(Serialize, Deserialize, Debug)]
pub struct RequestUpdateSignParams {
    pub identifier: ObjectIdentifier,
    pub unk0: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ResponseUpdateSignParams {
    pub identifier: ObjectIdentifier,
    pub unk0: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RequestGetMatchAreaSignListParams {
    pub known_signs: Vec<ObjectIdentifier>,
    pub puddles: Vec<PuddleArea>,
    pub matching_parameters: MatchingParameters,
    pub unk5: u8,
    pub unk6: u8,
    pub unk7: u8,
    pub group_passwords: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ResponseGetMatchAreaSignListParamsEntry {
    pub player_id: i32,
    pub identifier: ObjectIdentifier,
    pub puddle: PuddleArea,
    pub unk1: i32,
    pub data: Vec<u8>,
    pub external_id: String,
    pub unk2: u32,
    pub group_passwords: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ResponseGetMatchAreaSignListParams {
    pub known_signs: Vec<ObjectIdentifier>,
    pub entries: Vec<ResponseGetMatchAreaSignListParamsEntry>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RequestCreateMatchAreaSignParams {
    pub puddle_id: u32,
    // This request uses a fixed-size array for flags, but all other requests use a Vec<u8>
    pub puddle_flags: [u8; 8],
    pub matching_parameters: MatchingParameters,
    // MultiplayProperties + 0x10
    pub unk2: i32,
    pub data: Vec<u8>,
    pub group_passwords: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ResponseCreateMatchAreaSignParams {
    pub identifier: ObjectIdentifier,
}

#[cfg(test)]
mod test {
    use crate::eldenring::SellRegion;

    use super::{ObjectIdentifier, RequestGetSignListParams, RequestUpdateSignParams};
    use wire::deserialize;

    #[test]
    fn deserialize_update_sign() {
        let deserialized: RequestUpdateSignParams =
            deserialize(include_bytes!("../../test/data/RequestUpdateSign.bin")).unwrap();

        assert_eq!(deserialized.identifier, ObjectIdentifier(0),);
        assert_eq!(deserialized.unk0, 0);
    }

    #[test]
    fn deserialize_get_sign_list() {
        let deserialized: RequestGetSignListParams =
            deserialize(include_bytes!("../../test/data/RequestGetSignList.bin")).unwrap();

        assert_eq!(deserialized.known_signs.len(), 0);
        assert_eq!(deserialized.search_areas.len(), 2);
        assert_eq!(deserialized.search_areas[0].play_region, 1100000);
        assert_eq!(deserialized.search_areas[0].area, 1100000);
        assert_eq!(deserialized.search_areas[1].play_region, 1100001);
        assert_eq!(deserialized.search_areas[1].area, 1100001);
        assert_eq!(
            deserialized.matching_parameters.regulation_version,
            11001000
        );
        assert_eq!(
            deserialized.matching_parameters.sell_region,
            SellRegion::Global
        );
        assert!(
            !deserialized
                .matching_parameters
                .cross_region_matchmaking_disabled
        );
        assert_eq!(deserialized.matching_parameters.unk1, 1);
        assert_eq!(deserialized.matching_parameters.platform, 0);
        assert_eq!(deserialized.matching_parameters.character_level, 99);
        assert_eq!(deserialized.matching_parameters.unk2, 0);
        assert_eq!(deserialized.matching_parameters.game_clear_count, 0);
        assert_eq!(deserialized.matching_parameters.password, "");
        assert_eq!(deserialized.matching_parameters.vow_type, 0);
        assert_eq!(deserialized.matching_parameters.max_reinforce, 22);
        assert_eq!(deserialized.matching_parameters.max_spirit_ash_reinforce, 5);
    }
}
