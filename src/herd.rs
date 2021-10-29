use crate::{pasture::Pasture, species::Species};
use rand::{distributions::Slice, prelude::Distribution};
use serde::Serialize;

#[derive(Serialize)]
pub struct Herd {
    pub id: usize,
    pub pasture_id: usize,
    pub species_id: usize,
}

impl Herd {
    pub const fn new(id: usize, pasture: &Pasture, species: &Species) -> Self {
        Herd {
            id,
            pasture_id: pasture.id,
            species_id: species.id,
        }
    }
}

pub fn expand_herd_vec(herds: &mut Vec<Herd>, pastures: &[Pasture], species: &[Species]) {
    let mut rng = rand::thread_rng();
    let species_distribution = Slice::new(species).unwrap();

    for pasture in pastures.iter() {
        herds.push(Herd::new(
            herds.len(),
            pasture,
            species_distribution.sample(&mut rng),
        ));
    }
}
