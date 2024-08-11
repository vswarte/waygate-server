use serde::{Serialize, Deserialize};

use super::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct RequestCreateBloodMessageParams {
    pub area: PlayRegionArea,
    pub character_id: i32,
    pub data: Vec<u8>,
    pub unk: i32,
    pub group_passwords: Vec<String>,
}

pub type ResponseCreateBloodMessageParams = ObjectIdentifier;

#[derive(Serialize, Deserialize, Debug)]
pub struct RequestGetBloodMessageListParams {
    pub search_areas: Vec<PlayRegionArea>,
    pub group_passwords: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ResponseGetBloodMessageListParamsEntry {
    pub player_id: i32,
    pub character_id: i32,
    pub identifier: ObjectIdentifier,
    pub rating_good: i32,
    pub rating_bad: i32,
    pub data: Vec<u8>,
    pub area: PlayRegionArea,
    pub group_passwords: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ResponseGetBloodMessageListParams {
    pub entries: Vec<ResponseGetBloodMessageListParamsEntry>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RequestEvaluateBloodMessageParams {
    pub identifier: ObjectIdentifier,
    pub rating: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RequestReentryBloodMessageParams {
    pub identifiers: Vec<ObjectIdentifier>,
    pub unk: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ResponseReentryBloodMessageParams {
    pub identifiers: Vec<ObjectIdentifier>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RequestRemoveBloodMessageParams {
    pub identifier: ObjectIdentifier,
}

#[cfg(test)]
mod test {
    use waygate_wire::deserialize;
    use crate::RequestGetBloodMessageListParams;
    use crate::RequestCreateBloodMessageParams;

    #[test]
    fn deserialize_create_bloodmessage() {
        let deserialized: RequestCreateBloodMessageParams = deserialize(
            include_bytes!("../test/data/RequestCreateBloodMessage.bin"),
        ).unwrap();

        assert_eq!(deserialized.area.play_region, 1400001);
        assert_eq!(deserialized.area.area, 1400001);
        assert_eq!(deserialized.character_id, 21);
        assert_eq!(deserialized.data.len(), 520);
        assert_eq!(deserialized.group_passwords.len(), 1);
        assert_eq!(deserialized.group_passwords[0], "schlong");
        assert_eq!(deserialized.unk, 0);
    }

    #[test]
    fn deserialize_get_bloodmessage_list() {
        let deserialized: RequestGetBloodMessageListParams = deserialize(
            include_bytes!("../test/data/RequestGetBloodMessageList.bin"),
        ).unwrap();

        assert_eq!(deserialized.search_areas.len(), 8);
        assert_eq!(deserialized.search_areas[0].play_region, 6043340);
        assert_eq!(deserialized.search_areas[0].area, 6102002);
        assert_eq!(deserialized.search_areas[1].play_region, 6042340);
        assert_eq!(deserialized.search_areas[1].area, 6102002);
        assert_eq!(deserialized.search_areas[2].play_region, 6043350);
        assert_eq!(deserialized.search_areas[2].area, 6102002);
        assert_eq!(deserialized.search_areas[3].play_region, 6042350);
        assert_eq!(deserialized.search_areas[3].area, 6102002);
        assert_eq!(deserialized.search_areas[4].play_region, 6043340);
        assert_eq!(deserialized.search_areas[4].area, 6102000);
        assert_eq!(deserialized.search_areas[5].play_region, 6042340);
        assert_eq!(deserialized.search_areas[5].area, 600000);
        assert_eq!(deserialized.search_areas[6].play_region, 6041340);
        assert_eq!(deserialized.search_areas[6].area, 6102002);
        assert_eq!(deserialized.search_areas[7].play_region, 6041340);
        assert_eq!(deserialized.search_areas[7].area, 600000);
        assert_eq!(deserialized.group_passwords.len(), 4);
        assert_eq!(deserialized.group_passwords[0], "group1");
        assert_eq!(deserialized.group_passwords[1], "group2");
        assert_eq!(deserialized.group_passwords[2], "group3");
        assert_eq!(deserialized.group_passwords[3], "group4");
    }
}
