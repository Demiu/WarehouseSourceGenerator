use std::time::Duration;

use chrono::NaiveDateTime;
use rand::{distributions::Uniform, prelude::Distribution, seq::index, Rng};
use serde::Serialize;

use crate::{herd::Herd, pasture::Pasture, species::Species};

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
    latest_birth: NaiveDateTime,
) {
    let mut rng = rand::thread_rng();

    let birth_span = latest_birth
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
            .pasture_kind_to_req_area[pasture.kind];
        let count = (pasture.area
            / (area_requirement
                * chrono::Duration::from_std(species.lifespan)
                    .unwrap()
                    .num_days() as f32)) as usize;
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
    max_date: NaiveDateTime,
) {
    let mut rng = rand::thread_rng();

    let kill_count = (livestock.len() as f32 * kill_pct) as usize;
    let to_kill = index::sample(&mut rng, livestock.len(), kill_count);
    for id in to_kill {
        let animal = &mut livestock[id];
        if let Some(_) = animal.disposal {
            continue;
        }
        let species = &species[animal.species_id];
        let lifespan =
            chrono::Duration::from_std(rng.gen_range(Duration::new(0, 0)..species.lifespan))
                .unwrap();
        let mut disposal_time = animal.birth + lifespan;
        if disposal_time > max_date {
            disposal_time = max_date;
        }
        animal.disposal = Some(disposal_time);
        animal.disposal_purpose = Some(DisposalPurpose::Health);
    }
}

pub fn butcher_livestock_vec(
    livestock: &mut Vec<Livestock>,
    species: &Vec<Species>,
    cutoff_time: NaiveDateTime,
) {
    for animal in livestock {
        if let Some(_) = animal.disposal_purpose {
            continue;
        }
        let species = &species[animal.species_id];
        let disposal = animal.birth + chrono::Duration::from_std(species.lifespan).unwrap();
        if disposal < cutoff_time {
            animal.disposal = Some(disposal);
            animal.disposal_purpose = Some(DisposalPurpose::Butcher);
        }
    }
}
