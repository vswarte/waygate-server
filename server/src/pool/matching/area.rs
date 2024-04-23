use fnrpc::shared::OnlineArea;

#[derive(Clone, Debug, PartialEq)]
pub struct MatchingArea {
    area: u32,
    play_region: u32,
}

impl MatchingArea {
    pub const fn new(area: u32, play_region: u32) -> Self {
        Self { area, play_region }
    }
}

impl From<&OnlineArea> for MatchingArea {
    fn from(value: &OnlineArea) -> Self {
        Self {
            area: value.area as u32,
            play_region: value.play_region as u32,
        }
    }
}

impl From<&MatchingArea> for OnlineArea {
    fn from(val: &MatchingArea) -> Self {
        OnlineArea {
            area: val.area as i32,
            play_region: val.play_region as i32,
        }
    }
}
