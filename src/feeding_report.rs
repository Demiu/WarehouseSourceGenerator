use chrono::{prelude::*, Duration};
use rand::{prelude::SliceRandom, Rng};

use crate::{config, pasture::Pasture};

#[derive(Debug)]
pub struct FeedingReport {
    pub id: u32,
    pub date: NaiveDate,
    pub pasture_id: u32,
    pub start_fill_pct: f32,
    pub end_fill_pct: f32,
}

impl FeedingReport {
    pub const fn new(
        id: u32,
        date: NaiveDate,
        pasture_id: u32,
        start_fill_pct: f32,
        end_fill_pct: f32,
    ) -> Self {
        FeedingReport {
            id,
            date,
            pasture_id,
            start_fill_pct,
            end_fill_pct,
        }
    }

    pub fn generate_random(pastures: &Vec<Pasture>) -> Vec<Self> {
        let count: i32 = config::FEEDING_REPORT_COUNT;
        let interval: Duration =
            chrono::Duration::from_std(config::FEEDING_REPORT_INTERVAL).unwrap();
        let last_report: NaiveDateTime = Local::now().naive_local();

        let mut rng = rand::thread_rng();
        let mut date = last_report - interval * count;

        let mut reports = Vec::with_capacity(count as usize);
        reports.push(FeedingReport::new(
            0,
            date.date(),
            pastures.choose(&mut rng).unwrap().id,
            rng.gen(),
            100.0,
        ));
        for i in 1..count as usize {
            date += interval;
            let previous_report = &reports[i - 1];
            let start_fill = rng.gen_range(0.0..previous_report.end_fill_pct);
            let end_fill = rng.gen_range(start_fill..100.0);
            let pasture_id = pastures.choose(&mut rng).unwrap().id;
            reports.push(FeedingReport::new(
                i as u32,
                date.date(),
                pasture_id,
                start_fill,
                end_fill,
            ));
        }

        return reports;
    }
}
