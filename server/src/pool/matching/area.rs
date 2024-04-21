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

impl Into<OnlineArea> for &MatchingArea {
    fn into(self) -> OnlineArea {
        OnlineArea {
            area: self.area as i32,
            play_region: self.play_region as i32,
        }
    }
}
