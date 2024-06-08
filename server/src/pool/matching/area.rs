use fnrpc::shared::{PlayRegionArea, PuddleArea};

#[derive(Clone, Debug, PartialEq)]
pub enum MatchingArea {
    PlayRegion {
        area: i32,
        play_region: i32,
    },
    Puddle {
        match_area: i32,
        area: i32,
    },
}

impl Default for MatchingArea {
    fn default() -> Self {
        Self::PlayRegion { area: -1, play_region: -1 }
    }
}

impl MatchingArea {
    pub fn matches(&self, other: &Self) -> bool {
        match self {
            MatchingArea::PlayRegion { area: _, play_region: a } => match other {
                MatchingArea::PlayRegion { area: _, play_region: b } => a == b,
                MatchingArea::Puddle { match_area, area } => false,
            },
            MatchingArea::Puddle { match_area: a, area: _ } => match other {
                MatchingArea::PlayRegion { area, play_region } => false,
                MatchingArea::Puddle { match_area: b, area: _ } => a == b,
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
        Self::Puddle {
            match_area: value.match_area,
            area: value.area,
        }
    }
}

impl From<&MatchingArea> for PlayRegionArea {
    fn from(val: &MatchingArea) -> Self {
        match val {
            MatchingArea::PlayRegion { area, play_region } => PlayRegionArea {
                area: area.to_owned(),
                play_region: play_region.to_owned(),
            },
            MatchingArea::Puddle { match_area: _, area: _} => unimplemented!("Tried converting puddle area to play region?"),
        }
    }
}

impl From<&MatchingArea> for PuddleArea {
    fn from(val: &MatchingArea) -> Self {
        match val {
            MatchingArea::Puddle { match_area, area } => PuddleArea {
                match_area: match_area.to_owned(),
                area: area.to_owned(),
            },
            MatchingArea::PlayRegion { area, play_region } => unimplemented!("Tried converting puddle area to play region?"),
        }
    }
}
