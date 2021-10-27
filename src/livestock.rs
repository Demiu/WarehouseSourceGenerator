use chrono::NaiveDateTime;

use crate::{herd::Herd, species::Species};

pub struct Livestock<'a> {
    pub id: u32,
    birth: NaiveDateTime,
    disposal: Option<NaiveDateTime>,
    disposal_purpose: &'a str,
    species_id: u32,
    herd_id: u32,
}

impl<'a> Livestock<'a> {
    pub const fn new(
        id: u32,
        birth: NaiveDateTime,
        disposal: Option<NaiveDateTime>,
        disposal_purpose: &'a str,
        herd: Herd,
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
