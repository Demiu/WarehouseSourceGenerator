use crate::{pasture::Pasture, species::Species};
use serde::Serialize;

#[derive(Serialize)]
pub struct Herd {
    pub id: u32,
    pub pasture_id: u32,
    pub species_id: u32,

    #[serde(skip_serializing)]
    pub size: u32,
}

impl Herd {
    pub const fn new(id: u32, pasture: &Pasture, species: &Species) -> Self {
        Herd {
            id,
            pasture_id: pasture.id,
            species_id: species.id,
            size: 0,
        }
    }
}
