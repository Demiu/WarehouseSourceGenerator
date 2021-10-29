use core::panic;

use enum_map::{Enum, EnumMap};
use rand::{self, Rng};
use serde::Serialize;

#[derive(Clone, Copy, Enum, Serialize)]
pub enum PastureKind {
    Open,
    Covered,
    Individual,
}

pub struct PastureAreaMinMax {
    pub min: f32,
    pub max: f32,
}

#[derive(Serialize)]
pub struct Pasture {
    pub id: usize,
    pub area: f32,
    pub kind: PastureKind,
}

impl Pasture {
    pub const fn new(id: usize, area: f32, kind: PastureKind) -> Self {
        Pasture { id, area, kind }
    }
}

pub fn expand_pasture_vec(
    pastures: &mut Vec<Pasture>,
    count: usize,
    size_ranges: &EnumMap<PastureKind, PastureAreaMinMax>,
) {
    let mut rng = rand::thread_rng();

    for _ in 0..count {
        let kind = match rng.gen_range(0..3) {
            0 => PastureKind::Open,
            1 => PastureKind::Covered,
            2 => PastureKind::Individual,
            _ => panic!(),
        };
        pastures.push(Pasture::new(
            pastures.len(),
            rng.gen_range(size_ranges[kind].min..=size_ranges[kind].max),
            kind,
        ));
    }
}
