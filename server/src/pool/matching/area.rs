use fnrpc::shared::PlayRegionArea;

#[derive(Clone, Debug, PartialEq)]
pub enum MatchingArea {
    PlayRegion {
        area: i32,
        play_region: i32,
    },
    Puddle(i32),
}

impl From<&PlayRegionArea> for MatchingArea {
    fn from(value: &PlayRegionArea) -> Self {
        Self::PlayRegion {
            area: value.area,
            play_region: value.play_region,
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
            MatchingArea::Puddle(_) => unimplemented!("Tried converting puddle area to play region?"),
        }
    }
}
