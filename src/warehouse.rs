use crate::employee::Employee;
use serde::Serialize;

#[derive(Serialize)]
pub struct Warehouse {
    id: u32,
    manager_id: u32,
    area: f32,
    volume: f32,
}

impl Warehouse {
    pub const fn new(id: u32, manager: &Employee, area: f32, volume: f32) -> Self {
        Self {
            id,
            manager_id: manager.id,
            area,
            volume,
        }
    }
}
