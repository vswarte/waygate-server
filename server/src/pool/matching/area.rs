use fnrpc::shared::OnlineArea;

#[derive(Clone, Debug, PartialEq)]
pub struct MatchingArea {
    map: u32,
    play_region: u32,
}

impl MatchingArea {
    pub const fn new(map: u32, play_region: u32) -> Self {
        Self { map, play_region }
    }
}

impl From<&OnlineArea> for MatchingArea {
    fn from(value: &OnlineArea) -> Self {
        Self {
            map: value.map as u32,
            play_region: value.play_region as u32,
        }
    }
}

impl Into<OnlineArea> for &MatchingArea {
    fn into(self) -> OnlineArea {
        OnlineArea {
            map: self.map as i32,
            play_region: self.play_region as i32,
        }
    }
}
