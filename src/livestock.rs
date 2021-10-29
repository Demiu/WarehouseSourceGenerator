use chrono::{Duration, Local, NaiveDateTime};
use rand::{distributions::Uniform, prelude::Distribution, Rng};
use serde::Serialize;

use crate::{
    config,
    herd::Herd,
    pasture::{self, Pasture},
    species::Species,
};

#[derive(Serialize)]
pub struct Livestock<'a> {
    pub id: usize,
    birth: NaiveDateTime,
    disposal: Option<NaiveDateTime>,
    disposal_purpose: Option<&'a str>,
    species_id: usize,
    herd_id: usize,
}

impl<'a> Livestock<'a> {
    pub const fn new(
        id: usize,
        birth: NaiveDateTime,
        disposal: Option<NaiveDateTime>,
        disposal_purpose: Option<&'a str>,
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

