use chrono::{Duration, Local, NaiveDateTime};
use rand::{distributions::Uniform, prelude::Distribution, Rng};
use serde::Serialize;

use crate::{config, herd::Herd, species::Species};

#[derive(Serialize)]
pub struct Livestock<'a> {
    pub id: u32,
    birth: NaiveDateTime,
    disposal: Option<NaiveDateTime>,
    disposal_purpose: Option<&'a str>,
    species_id: u32,
    herd_id: u32,
}

impl<'a> Livestock<'a> {
    pub const fn new(
        id: u32,
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

    pub fn expand_herd_random(
        to_expand: &mut Vec<Self>,
        herd: &Herd,
        count: usize,
        birth_min: NaiveDateTime,
    ) {
        let mut rng = rand::thread_rng();

        let birth_span = Local::now()
            .naive_local()
            .signed_duration_since(birth_min)
            .to_std()
            .unwrap();
        let distribution = Uniform::new(std::time::Duration::new(0, 0), birth_span);

        let indicices = to_expand.len()..(count + to_expand.len());
        for id in indicices {
            let birth_offset = chrono::Duration::from_std(distribution.sample(&mut rng)).unwrap();
            to_expand.push(Livestock::new(
                id as u32,
                birth_min + birth_offset,
                None,
                None,
                herd,
            ));
        }
    }
}
