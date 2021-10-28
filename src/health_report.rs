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
    pub fn new(
        id: u32,
        doctor: &Employee,
        herd: &Herd,
        timestamp: NaiveDateTime,
        ill_pct: f32,
        severly_ill_pct: f32,
        terminal_pct: f32,
    ) -> Self {
        let ill_count = (herd.size as f32 * ill_pct) as u32;
        let severly_ill_count = (herd.size as f32 * severly_ill_pct) as u32;
        let terminal_count = (herd.size as f32 * terminal_pct) as u32;
        let healthy_count = herd.size - ill_count - severly_ill_count - terminal_count;
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
