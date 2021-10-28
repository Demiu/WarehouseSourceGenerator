use serde::Serialize;

#[derive(Serialize)]
pub enum PastureKind {
    Open,
    Covered,
    Individual,
}

#[derive(Serialize)]
pub struct Pasture {
    pub id: u32,
    area: f32,
    kind: PastureKind,
}

impl Pasture {
    pub const fn new(id: u32, area: f32, kind: PastureKind) -> Self {
        Pasture { id, area, kind }
    }
}
