use chrono::{Duration, NaiveDateTime};
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

pub fn expand_headcount_report_vec(
    headcount_reports: &mut Vec<HeadcountReport>,
    herds: &Vec<Herd>,
    employees: &Vec<Employee>,
    min_count: u32,
    max_count: u32,
    first_report_dt: NaiveDateTime,
    last_report_dt: NaiveDateTime,
    report_interval: Duration,
) {
    let count_per_herd = last_report_dt
        .signed_duration_since(first_report_dt)
        .num_seconds()
        / report_interval.num_seconds();

    let mut rng = rand::thread_rng();
    let employee_distribution = Slice::new(employees).unwrap();
    let quantity_distribution = Uniform::new(min_count, max_count);

    for herd in herds {
        let mut timestamp = first_report_dt;
        for _ in 0..count_per_herd {
            headcount_reports.push(HeadcountReport::new(
                headcount_reports.len(),
                employee_distribution.sample(&mut rng),
                herd,
                timestamp,
                quantity_distribution.sample(&mut rng),
            ));
            timestamp += report_interval;
        }
    }
}
