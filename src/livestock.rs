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
}
