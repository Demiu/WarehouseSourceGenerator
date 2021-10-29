use chrono::{prelude::*, Duration};
use rand::Rng;
use serde::Serialize;

use crate::pasture::Pasture;

#[derive(Serialize)]
pub struct FeedingReport {
    pub id: usize,
    pub date: NaiveDate,
    pub pasture_id: usize,
    pub start_fill_pct: f32,
    pub end_fill_pct: f32,
}

impl FeedingReport {
    pub const fn new(
        id: usize,
        date: NaiveDate,
        pasture: &Pasture,
        start_fill_pct: f32,
        end_fill_pct: f32,
    ) -> Self {
        FeedingReport {
            id,
            date,
            pasture_id: pasture.id,
            start_fill_pct,
            end_fill_pct,
        }
    }
}

pub fn expand_feeding_report_vec(
    feeding_reports: &mut Vec<FeedingReport>,
    pastures: &Vec<Pasture>,
    first_report_dt: NaiveDateTime,
    last_report_dt: NaiveDateTime,
    report_interval: Duration,
) {
    let count_per_pasture = last_report_dt
        .signed_duration_since(first_report_dt)
        .num_seconds()
        / report_interval.num_seconds();

    let mut rng = rand::thread_rng();

    for pasture in pastures {
        let mut date = first_report_dt;
        feeding_reports.push(FeedingReport::new(
            feeding_reports.len(),
            date.date(),
            pasture,
            0.0,
            rng.gen_range(0.0..100.0),
        ));
        for _ in 0..(count_per_pasture - 1) {
            date += report_interval;
            let prev_report = feeding_reports.last().unwrap();
            let start_fill = rng.gen_range(0.0..prev_report.end_fill_pct);
            let end_fill = rng.gen_range(start_fill..=100.0);
            feeding_reports.push(FeedingReport::new(
                feeding_reports.len(),
                date.date(),
                pasture,
                start_fill,
                end_fill,
            ));
        }
    }
}
