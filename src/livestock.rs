use std::time::Duration;

use chrono::{Local, NaiveDate, NaiveDateTime};
use rand::{
    distributions::Uniform,
    prelude::{Distribution, SliceRandom},
    seq::index,
    Rng,
};
use serde::Serialize;

use crate::{
    config,
    herd::Herd,
    pasture::{self, Pasture},
    species::Species,
};

#[derive(Serialize)]
pub enum DisposalPurpose {
    Butcher,
    Health,
}

#[derive(Serialize)]
pub struct Livestock {
    pub id: usize,
    birth: NaiveDateTime,
    disposal: Option<NaiveDateTime>,
    disposal_purpose: Option<DisposalPurpose>,
    species_id: usize,
    herd_id: usize,
}

impl Livestock {
    pub const fn new(
        id: usize,
        birth: NaiveDateTime,
        disposal: Option<NaiveDateTime>,
        disposal_purpose: Option<DisposalPurpose>,
        herd: &Herd,
    ) -> Self {
        Livestock {
            id,
            birth,
            disposal,
            disposal_purpose,
            species_id: herd.species_id,
            herd_id: herd.id,
        }
    }
}

pub fn expand_livestock(
    livestock: &mut Vec<Livestock>,
    herds: &Vec<Herd>,
    species: &Vec<Species>,
    pastures: &Vec<Pasture>,
    earliest_birth: NaiveDateTime,
) {
    let mut rng = rand::thread_rng();

    let birth_span = Local::now()
        .naive_local()
        .signed_duration_since(earliest_birth)
        .to_std()
        .unwrap();
    let birth_offset_distribution = Uniform::new(std::time::Duration::new(0, 0), birth_span);

    for herd in herds {
        let species = &species[herd.species_id];
        let pasture = &pastures[herd.pasture_id];
        let area_requirement = species
            .area_requirements
            .as_ref()
            .unwrap()
            .pastureKindToReqArea[pasture.kind];
        let count = (pasture.area / area_requirement) as usize;
        for _ in 0..count {
            let birth_offset =
                chrono::Duration::from_std(birth_offset_distribution.sample(&mut rng)).unwrap();
            livestock.push(Livestock::new(
                livestock.len(),
                earliest_birth + birth_offset,
                None,
                None,
                herd,
            ));
        }
    }
}

pub fn kill_off_livestock_vec(
    livestock: &mut Vec<Livestock>,
    kill_pct: f32,
    species: &Vec<Species>,
) {
    let mut rng = rand::thread_rng();

    let kill_count = (livestock.len() as f32 * kill_pct) as usize;
    let to_kill = index::sample(&mut rng, livestock.len(), kill_count);
    for id in to_kill {
        let animal = &mut livestock[id];
        let species = &species[animal.species_id];
        let lifespan =
            chrono::Duration::from_std(rng.gen_range(Duration::new(0, 0)..species.lifespan))
                .unwrap();
        animal.disposal = Some(animal.birth + lifespan);
        animal.disposal_purpose = Some(DisposalPurpose::Health);
    }
}
