use chrono::{Duration, Local, NaiveDateTime};
use rand::{distributions::Uniform, prelude::Distribution, Rng};
use serde::Serialize;

use crate::{config, herd::Herd, species::Species};

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
