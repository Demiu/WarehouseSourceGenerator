use serde::Serialize;

#[derive(Serialize)]
pub enum SpeciesKind {
    Animal,
    Plant,
}

#[derive(Serialize)]
pub struct Species<'a> {
    pub id: u32,
    name: &'a str,
    kind: SpeciesKind,
}

impl<'a> Species<'a> {
    pub const fn new(id: u32, name: &'a str, kind: SpeciesKind) -> Self {
        Species { id, name, kind }
    }
}
