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

pub fn generate_feeding_report_vec(
    pastures: &Vec<Pasture>,
    count_per_pasture: usize,
    report_interval: Duration,
    last_report_date: Option<NaiveDateTime>,
) -> Vec<FeedingReport> {
    let last_report_date = match last_report_date {
        None => Local::now().naive_local(),
        Some(date) => date,
    };
    let first_report_date = last_report_date - report_interval * count_per_pasture as i32;

    let mut rng = rand::thread_rng();

    let mut ret = vec![];
    for pasture in pastures {
        let mut date = first_report_date;
        ret.push(FeedingReport::new(
            ret.len(),
            date.date(),
            pasture,
            100.0,
            rng.gen(),
        ));
        for _ in 0..(count_per_pasture - 1) {
            date += report_interval;
            let prev_report = ret.last().unwrap();
            let start_fill = rng.gen_range(0.0..prev_report.end_fill_pct);
            let end_fill = rng.gen_range(start_fill..=100.0);
            ret.push(FeedingReport::new(
                ret.len(),
                date.date(),
                pasture,
                start_fill,
                end_fill,
            ));
        }
    }

    return ret;
}
