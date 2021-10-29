use chrono::NaiveDateTime;
use serde::Serialize;

use crate::{employee::Employee, herd::Herd};

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
