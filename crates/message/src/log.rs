use serde::{Deserialize, Serialize};

use crate::Location;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RequestUseItemLogParams {
    pub used_items: Vec<RequestUseItemLogParamsEntry>,
    pub location: Location,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RequestUseItemLogParamsEntry {
    pub item_id: u32,
    pub times_used: u32,
    pub unk3: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RequestGetItemLogParams {
    pub acquired_items: Vec<RequestGetItemLogParamsEntry>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RequestGetItemLogParamsEntry {
    pub location: Location,
    pub item_category: u32,
    pub item_id: u32,
    pub quantity: u32,
    pub unk1: u32,
    pub unk2: u32,
    pub unk3: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RequestKillEnemyLogParams {
    pub killed_enemies: Vec<RequestKillEnemyLogParamsEntry>,
    pub location: Location,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RequestKillEnemyLogParamsEntry {
    pub npc_param: u32,
    pub killed_count: u32,
}
#[cfg(test)]
mod test {
    use waygate_wire::deserialize;
    use crate::RequestUseItemLogParams;

    #[test]
    fn deserialize_use_item_log() {
        let deserialized: RequestUseItemLogParams = deserialize(
            include_bytes!("../test/data/RequestUseItemLog.bin"),
        ).unwrap();

        assert_eq!(deserialized.used_items.len(), 1);
        assert_eq!(deserialized.used_items[0].item_id, 101);
        assert_eq!(deserialized.used_items[0].times_used, 1);
        assert_eq!(deserialized.used_items[0].unk3, 1);
        assert_eq!(deserialized.location.map, 60423600);
        assert_eq!(deserialized.location.x, -45.939575);
        assert_eq!(deserialized.location.y, 92.36392);
        assert_eq!(deserialized.location.z, 79.65545);
    }
}
