use serde::{de, Deserialize, Serialize};

use super::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct RequestCreateBloodstainParams {
    pub area: PlayRegionArea,
    pub advertisement_data: Vec<u8>,
    pub replay_data: Vec<u8>,
    pub group_passwords: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BloodstainAdvertisementData {
    pub magic: [u8; 8],
    pub version: u32,
    pub unkc: u32,
    pub unk10: u32,
    pub unk14: u32,
    pub unk18: u32,
    pub location: Location,
    pub yaw: f32,
    pub ghost_spawn_position: Position,
    pub unk3c: u32,
    pub unk40: u16,
    pub unk42: u16,
    pub unk44: u16,
    pub unk46: u8,
    pub unk47: u8,
    pub unk48: u64,
    pub play_region: u32,
    pub character_name: [u16; 18],
    pub unk70: u32,
    pub unk74: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ResponseCreateBloodstainParams {
    pub identifier: ObjectIdentifier,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RequestGetBloodstainListParams {
    pub search_areas: Vec<PlayRegionArea>,
    pub group_passwords: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ResponseGetBloodstainListParamsEntry {
    pub area: PlayRegionArea,
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
    pub area: PlayRegionArea,
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
mod test {
    use super::{RequestCreateBloodstainParams, RequestGetBloodstainListParams};
    use wire::deserialize;

    #[test]
    fn deserialize_create_bloodstain() {
        let deserialized: RequestCreateBloodstainParams = deserialize(include_bytes!(
            "../../test/data/RequestCreateBloodstain.bin"
        ))
        .unwrap();

        assert_eq!(deserialized.area.play_region, 1400001);
        assert_eq!(deserialized.area.area, 1400001);
        assert_eq!(deserialized.advertisement_data.len(), 128);
        assert_eq!(deserialized.replay_data.len(), 1165);
        assert_eq!(deserialized.group_passwords.len(), 1);
        assert_eq!(deserialized.group_passwords[0], "schlong");

        let adv_data: super::BloodstainAdvertisementData =
            deserialize(&deserialized.advertisement_data).unwrap();
        assert_eq!(&adv_data.magic, &[7, 0, 0, 0, 120, 0, 0, 0]);
        assert_eq!(adv_data.version, 7);
        assert_eq!(adv_data.play_region, 1400001);
        assert_eq!(adv_data.location.position.x, 150.72684);
        assert_eq!(adv_data.location.position.y, 112.589775);
        assert_eq!(adv_data.location.position.z, -180.64731);
    }

    #[test]
    fn deserialize_get_bloodstain_list() {
        let deserialized: RequestGetBloodstainListParams = deserialize(include_bytes!(
            "../../test/data/RequestGetBloodstainList.bin"
        ))
        .unwrap();

        assert_eq!(deserialized.search_areas.len(), 10);
        assert_eq!(deserialized.search_areas[0].play_region, 3000001);
        assert_eq!(deserialized.search_areas[0].area, 3000001);
        assert_eq!(deserialized.search_areas[1].play_region, 6043330);
        assert_eq!(deserialized.search_areas[1].area, 6102002);
        assert_eq!(deserialized.search_areas[2].play_region, 6042330);
        assert_eq!(deserialized.search_areas[2].area, 6102002);
        assert_eq!(deserialized.search_areas[3].play_region, 3000000);
        assert_eq!(deserialized.search_areas[3].area, 3000000);
        assert_eq!(deserialized.search_areas[4].play_region, 6043330);
        assert_eq!(deserialized.search_areas[4].area, 6102000);
        assert_eq!(deserialized.search_areas[5].play_region, 6043340);
        assert_eq!(deserialized.search_areas[5].area, 6102002);
        assert_eq!(deserialized.search_areas[6].play_region, 6043340);
        assert_eq!(deserialized.search_areas[6].area, 6102000);
        assert_eq!(deserialized.search_areas[7].play_region, 6043320);
        assert_eq!(deserialized.search_areas[7].area, 6102002);
        assert_eq!(deserialized.search_areas[8].play_region, 6042340);
        assert_eq!(deserialized.search_areas[8].area, 6102002);
        assert_eq!(deserialized.search_areas[9].play_region, 6042320);
        assert_eq!(deserialized.search_areas[9].area, 6102002);
        assert_eq!(deserialized.group_passwords.len(), 4);
        assert_eq!(deserialized.group_passwords[0], "group1");
        assert_eq!(deserialized.group_passwords[1], "group2");
        assert_eq!(deserialized.group_passwords[2], "group3");
        assert_eq!(deserialized.group_passwords[3], "group4");
    }
}
