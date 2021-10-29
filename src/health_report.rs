use chrono::NaiveDateTime;
use rand::{
    distributions::{Slice, Uniform},
    prelude::Distribution,
};
use serde::Serialize;

use crate::{employee::Employee, headcount_report::HeadcountReport, herd::Herd};

#[derive(Serialize)]
pub struct HealthReport {
    pub id: usize,
    employee_id: usize,
    herd_id: usize,
    timestamp: NaiveDateTime,
    healthy_count: u32,
    ill_count: u32,
    severly_ill_count: u32,
    terminal_count: u32,
}

impl HealthReport {
    pub const fn new(
        id: usize,
        doctor: &Employee,
        herd: &Herd,
        timestamp: NaiveDateTime,
        healthy_count: u32,
        ill_count: u32,
        severly_ill_count: u32,
        terminal_count: u32,
    ) -> Self {
        Self {
            id,
            employee_id: doctor.id,
            herd_id: herd.id,
            timestamp,
            ill_count,
            severly_ill_count,
            terminal_count,
            healthy_count,
        }
    }
}

pub fn expand_health_report_vec_for_headcount_vec(
    health_reports: &mut Vec<HealthReport>,
    headcount_reports: &[HeadcountReport],
    employees: &Vec<Employee>,
    herds: &Vec<Herd>,
    ill_max_pct: f32,
    severly_ill_max_pct: f32,
    terminal_max_pct: f32,
) {
    let mut rng = rand::thread_rng();
    let employee_distribution = Slice::new(employees).unwrap();
    let ill_distribution = Uniform::new(0.0, ill_max_pct);
    let severly_ill_distribution = Uniform::new(0.0, severly_ill_max_pct);
    let terminal_distribution = Uniform::new(0.0, terminal_max_pct);

    for hc in headcount_reports {
        let doctor = employee_distribution.sample(&mut rng);
        let total_count = hc.quantity;
        let ill_count = (total_count as f32 * ill_distribution.sample(&mut rng)) as u32;
        let severly_ill_count =
            (total_count as f32 * severly_ill_distribution.sample(&mut rng)) as u32;
        let terminal_count = (total_count as f32 * terminal_distribution.sample(&mut rng)) as u32;
        let healthy_count = total_count - ill_count - severly_ill_count - terminal_count;
        health_reports.push(HealthReport::new(
            health_reports.len(),
            doctor,
            &herds[hc.herd_id],
            hc.timestamp,
            healthy_count,
            ill_count,
            severly_ill_count,
            terminal_count,
        ))
    }
}
