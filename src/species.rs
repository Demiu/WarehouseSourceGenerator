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
    pub id: u32,
    name: &'a str,
    kind: SpeciesKind,

    #[serde(skip_serializing)]
    area_requirements: Option<SpeciesAreaRequirements>,
}

impl<'a> Species<'a> {
    pub const fn new(
        id: u32,
        name: &'a str,
        kind: SpeciesKind,
        area_requirements: Option<SpeciesAreaRequirements>,
    ) -> Self {
        Species {
            id,
            name,
            kind,
            area_requirements,
        }
    }
}
