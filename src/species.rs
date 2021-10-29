use std::time::Duration;

use enum_map::EnumMap;
use serde::Serialize;

use crate::pasture::PastureKind;

#[derive(Serialize)]
pub enum SpeciesKind {
    Animal,
    Plant,
}

pub struct SpeciesAreaRequirements {
    pub pastureKindToReqArea: EnumMap<PastureKind, f32>,
}

#[derive(Serialize)]
pub struct Species<'a> {
    pub id: usize,
    name: &'a str,
    kind: SpeciesKind,

    #[serde(skip_serializing)]
    pub lifespan: Duration,
    #[serde(skip_serializing)]
    pub area_requirements: Option<SpeciesAreaRequirements>,
}

impl<'a> Species<'a> {
    pub const fn new(
        id: usize,
        name: &'a str,
        kind: SpeciesKind,
        lifespan_days: u64,
        area_requirements: Option<SpeciesAreaRequirements>,
    ) -> Self {
        Species {
            id,
            name,
            kind,
            lifespan: Duration::from_secs(lifespan_days * 24 * 60 * 60),
            area_requirements,
        }
    }
}
