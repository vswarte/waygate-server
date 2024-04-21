use serde::{Serialize, Deserialize};

use crate::params::shared::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct RequestCreateBloodstainParams {
    pub area: OnlineArea,
    pub advertisement_data: Vec<u8>,
    pub replay_data: Vec<u8>,
    pub group_passwords: Vec<String>,
}

pub type ResponseCreateBloodstainParams = ObjectIdentifier;

#[derive(Serialize, Deserialize, Debug)]
pub struct RequestGetBloodstainListParams {
    pub search_areas: Vec<OnlineArea>,
    pub group_passwords: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ResponseGetBloodstainListParamsEntry {
    pub area: OnlineArea,
    pub identifier: ObjectIdentifier,
    pub advertisement_data: Vec<u8>,
    pub group_passwords: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ResponseGetBloodstainListParams {
    pub entries: Vec<ResponseGetBloodstainListParamsEntry>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RequestGetDeadingGhostParams {
    pub map: i32,
    pub play_region: i32,
    pub identifier: ObjectIdentifier,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ResponseGetDeadingGhostParams {
    pub unk0: i32,
    pub unk4: i32,
    pub identifier: ObjectIdentifier,
    pub replay_data: Vec<u8>,
}

#[cfg(test)]
mod tests {
    use crate::bloodstain::*;
    use crate::Deserializer;

    #[test]
    fn request_create_bloodstain_params_deserialize() {
        let buf = include_bytes!("../../test/params/RequestCreateBloodstain.bin");

        let deserialized = RequestCreateBloodstainParams::deserialize(&mut Deserializer::new(buf))
            .unwrap();

        assert_eq!(6043340, deserialized.area.area);
        assert_eq!(6102002, deserialized.area.play_region);
        assert_ne!(0, deserialized.advertisement_data.len());
        assert_ne!(0, deserialized.replay_data.len());
        assert_eq!(String::from("group1"), deserialized.group_passwords[0]);
        assert_eq!(String::from("group2"), deserialized.group_passwords[1]);
        assert_eq!(String::from("group3"), deserialized.group_passwords[2]);
        assert_eq!(String::from("group4"), deserialized.group_passwords[3]);
    }

    #[test]
    fn request_get_bloodstain_list_params_deserialize() {
        let buf = include_bytes!("../../test/params/RequestGetBloodstainList.bin");

        let deserialized = RequestGetBloodstainListParams::deserialize(&mut Deserializer::new(buf))
            .unwrap();

        assert_eq!(3000001, deserialized.search_areas[0].area);
        assert_eq!(3000001, deserialized.search_areas[0].play_region);
        assert_eq!(6043330, deserialized.search_areas[1].area);
        assert_eq!(6102002, deserialized.search_areas[1].play_region);
        assert_eq!(6042330, deserialized.search_areas[2].area);
        assert_eq!(6102002, deserialized.search_areas[2].play_region);
        assert_eq!(3000000, deserialized.search_areas[3].area);
        assert_eq!(3000000, deserialized.search_areas[3].play_region);
        assert_eq!(6043330, deserialized.search_areas[4].area);
        assert_eq!(6102000, deserialized.search_areas[4].play_region);
        assert_eq!(6043340, deserialized.search_areas[5].area);
        assert_eq!(6102002, deserialized.search_areas[5].play_region);
        assert_eq!(6043340, deserialized.search_areas[6].area);
        assert_eq!(6102000, deserialized.search_areas[6].play_region);
        assert_eq!(6043320, deserialized.search_areas[7].area);
        assert_eq!(6102002, deserialized.search_areas[7].play_region);
        assert_eq!(6042340, deserialized.search_areas[8].area);
        assert_eq!(6102002, deserialized.search_areas[8].play_region);
        assert_eq!(6042320, deserialized.search_areas[9].area);
        assert_eq!(6102002, deserialized.search_areas[9].play_region);
        assert_eq!(String::from("group1"), deserialized.group_passwords[0]);
        assert_eq!(String::from("group2"), deserialized.group_passwords[1]);
        assert_eq!(String::from("group3"), deserialized.group_passwords[2]);
        assert_eq!(String::from("group4"), deserialized.group_passwords[3]);
    }
}
