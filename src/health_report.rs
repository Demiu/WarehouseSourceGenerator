use chrono::NaiveDateTime;
use serde::Serialize;

use crate::{
    employee::{self, Employee},
    herd::{self, Herd},
    species,
};

#[derive(Serialize)]
pub struct HealthReport {
    pub id: u32,
    employee_id: u32,
    herd_id: u32,
    timestamp: NaiveDateTime,
    healthy_count: u32,
    ill_count: u32,
    severly_ill_count: u32,
    terminal_count: u32,
}

impl HealthReport {
    pub const fn new(
        id: u32,
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
