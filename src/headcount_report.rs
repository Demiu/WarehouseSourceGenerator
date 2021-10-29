use chrono::{Duration, Local, NaiveDateTime};
use rand::{
    distributions::{Slice, Uniform},
    prelude::Distribution,
};
use serde::Serialize;

use crate::{employee::Employee, herd::Herd};

#[derive(Serialize)]
pub struct HeadcountReport {
    id: usize,
    employee_id: usize,
    pub herd_id: usize,
    pub timestamp: NaiveDateTime,
    pub quantity: u32,
}

impl HeadcountReport {
    pub const fn new(
        id: usize,
        employee: &Employee,
        herd: &Herd,
        timestamp: NaiveDateTime,
        quantity: u32,
    ) -> Self {
        Self {
            id,
            employee_id: employee.id,
            herd_id: herd.id,
            timestamp,
            quantity,
        }
    }
}

pub fn generate_headcount_report_vec(
    count_per_herd: usize,
    herds: &Vec<Herd>,
    employees: &Vec<Employee>,
    min_count: u32,
    max_count: u32,
    last_report_dt: Option<NaiveDateTime>,
    report_interval: Duration,
) -> Vec<HeadcountReport> {
    let last_report_dt = match last_report_dt {
        None => Local::now().naive_local(),
        Some(date) => date,
    };
    let first_report_dt = last_report_dt - report_interval * count_per_herd as i32;

    let mut rng = rand::thread_rng();
    let employee_distribution = Slice::new(employees).unwrap();
    let quantity_distribution = Uniform::new(min_count, max_count);

    let mut ret = vec![];
    for herd in herds {
        let mut timestamp = first_report_dt;
        for _ in 0..count_per_herd {
            ret.push(HeadcountReport::new(
                ret.len(),
                employee_distribution.sample(&mut rng),
                herd,
                timestamp,
                quantity_distribution.sample(&mut rng),
            ));
            timestamp += report_interval;
        }
    }

    return ret;
}
