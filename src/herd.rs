use crate::{pasture::Pasture, species::Species};

pub struct Herd {
    pub id: u32,
    pub pasture_id: u32,
    pub species_id: u32,
}

impl Herd {
    pub const fn new(id: u32, pasture: &Pasture, species: &Species) -> Self {
        Herd {
            id,
            pasture_id: pasture.id,
            species_id: species.id,
        }
    }
}
