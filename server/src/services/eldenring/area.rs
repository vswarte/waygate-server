use message::eldenring::{PlayRegionArea, PuddleArea};

#[derive(Clone, Debug, PartialEq)]
pub enum MatchingArea {
    PlayRegion(PlayRegionArea),
    Puddle(PuddleArea),
}

impl Default for MatchingArea {
    fn default() -> Self {
        Self::PlayRegion(PlayRegionArea {
            area: -1,
            play_region: -1,
        })
    }
}

impl MatchingArea {
    pub fn matches(&self, other: &Self) -> bool {
        match (self, other) {
            (MatchingArea::PlayRegion(a), MatchingArea::PlayRegion(b)) => a == b,
            (MatchingArea::Puddle(a), MatchingArea::Puddle(b)) => a.puddle_id == b.puddle_id,
            _ => false,
        }
    }
}

impl From<PlayRegionArea> for MatchingArea {
    fn from(value: PlayRegionArea) -> Self {
        Self::PlayRegion(value)
    }
}

impl From<PuddleArea> for MatchingArea {
    fn from(value: PuddleArea) -> Self {
        Self::Puddle(value)
    }
}

impl From<MatchingArea> for PlayRegionArea {
    fn from(val: MatchingArea) -> Self {
        match val {
            MatchingArea::PlayRegion(play_region) => play_region,
            MatchingArea::Puddle { .. } => {
                unimplemented!("Tried converting puddle area to play region?")
            }
        }
    }
}

impl From<MatchingArea> for PuddleArea {
    fn from(val: MatchingArea) -> Self {
        match val {
            MatchingArea::Puddle(puddle_area) => puddle_area,
            MatchingArea::PlayRegion { .. } => {
                unimplemented!("Tried converting puddle area to play region?")
            }
        }
    }
}
