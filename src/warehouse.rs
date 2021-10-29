use crate::employee::Employee;
use rand::{prelude::SliceRandom, Rng};
use serde::Serialize;

#[derive(Serialize)]
pub struct Warehouse {
    id: usize,
    manager_id: usize,
    area: f32,
    volume: f32,
}

impl Warehouse {
    pub const fn new(id: usize, manager: &Employee, area: f32, volume: f32) -> Self {
        Self {
            id,
            manager_id: manager.id,
            area,
            volume,
        }
    }
}

pub fn expand_warehouse_vec(
    warehouses: &mut Vec<Warehouse>,
    count: usize,
    employees: &Vec<Employee>,
) {
    let mut rng = rand::thread_rng();

    for manager in employees.choose_multiple(&mut rng, count) {
        warehouses.push(Warehouse::new(
            warehouses.len(),
            manager,
            rng.gen_range(0.0..40000.0),
            rng.gen_range(0.0..90000.0),
        ));
    }
}

pub fn shuffle_managers_warehouse_vec(warehouses: &mut [Warehouse], employees: &Vec<Employee>) {
    let mut rng = rand::thread_rng();

    for (i, manager) in employees
        .choose_multiple(&mut rng, warehouses.len())
        .enumerate()
    {
        warehouses[i].manager_id = manager.id;
    }
}
