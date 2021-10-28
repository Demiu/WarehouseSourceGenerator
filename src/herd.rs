use crate::{
    pasture::{self, Pasture},
    species::Species,
};
use rand::{distributions::Slice, prelude::Distribution};
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

pub fn expand_herd_vec(herds: &mut Vec<Herd>, species: &Vec<Species>, pastures: &Vec<Pasture>) {
    let mut rng = rand::thread_rng();
    let species_distribution = Slice::new(species).unwrap();

    for pasture in pastures.iter() {
        herds.push(Herd::new(
            herds.len() as u32,
            pasture,
            species_distribution.sample(&mut rng),
        ));
    }
}
