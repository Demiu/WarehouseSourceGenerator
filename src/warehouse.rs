use crate::employee::Employee;
use rand::{prelude::*, Rng};
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

pub fn randomly_enlarge_warehouses(warehouses: &mut [Warehouse], maximum_extra_area: f32, maximum_extra_volume: f32) {
    let mut rng = rand::thread_rng();

    let to_edit = rng.gen_range(0..warehouses.len());
    for warehouse in warehouses.iter_mut().choose_multiple(&mut rng, to_edit)
    {
        warehouse.area += rng.gen_range(0.0..maximum_extra_area);
        warehouse.volume += rng.gen_range(0.0..maximum_extra_volume);
    }
}
