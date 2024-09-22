use waygate_message::{PlayRegionArea, PuddleArea};

#[derive(Clone, Debug, PartialEq)]
pub enum MatchingArea {
    PlayRegion { area: i32, play_region: i32 },
    Puddle(i32),
}

impl Default for MatchingArea {
    fn default() -> Self {
        Self::PlayRegion {
            area: -1,
            play_region: -1,
        }
    }
}

impl MatchingArea {
    pub fn matches(&self, other: &Self) -> bool {
        match self {
            MatchingArea::PlayRegion {
                area: _,
                play_region: a,
            } => match other {
                MatchingArea::PlayRegion {
                    area: _,
                    play_region: b,
                } => a == b,
                MatchingArea::Puddle(_) => false,
            },
            MatchingArea::Puddle(a) => match other {
                MatchingArea::PlayRegion {
                    area: _,
                    play_region: _,
                } => false,
                MatchingArea::Puddle(b) => a == b,
            },
        }
    }
}

impl From<&PlayRegionArea> for MatchingArea {
    fn from(value: &PlayRegionArea) -> Self {
        Self::PlayRegion {
            area: value.area,
            play_region: value.play_region,
        }
    }
}

impl From<&PuddleArea> for MatchingArea {
    fn from(value: &PuddleArea) -> Self {
        Self::Puddle(value.0)
    }
}

impl From<&MatchingArea> for PlayRegionArea {
    fn from(val: &MatchingArea) -> Self {
        match val {
            MatchingArea::PlayRegion { area, play_region } => PlayRegionArea {
                area: area.to_owned(),
                play_region: play_region.to_owned(),
            },
            MatchingArea::Puddle(_) => {
                unimplemented!("Tried converting puddle area to play region?")
            }
        }
    }
}

impl From<&MatchingArea> for PuddleArea {
    fn from(val: &MatchingArea) -> Self {
        match val {
            MatchingArea::Puddle(match_area) => PuddleArea(match_area.to_owned()),
            MatchingArea::PlayRegion {
                area: _,
                play_region: _,
            } => unimplemented!("Tried converting puddle area to play region?"),
        }
    }
}
