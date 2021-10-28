mod employee;
mod feeding_report;
mod headcount_report;
mod health_report;
mod herd;
mod livestock;
mod pasture;
mod snapshot;
mod species;
mod warehouse;

use std::{fs::OpenOptions, path::Path, vec};

use chrono::{NaiveDate, NaiveDateTime, NaiveTime};
use employee::*;
use enum_map::enum_map;
use feeding_report::*;
use herd::*;
use livestock::*;
use pasture::*;
use rand::{prelude::SliceRandom, Rng};
use serde::ser::SerializeStruct;
use snapshot::*;
use species::*;

use crate::warehouse::Warehouse;

mod config {
    use std::time::Duration;

    pub const RESULT_DIR: &str = "out";

    pub const FEEDING_REPORT_COUNT: i32 = 1_000;
    pub const FEEDING_REPORT_INTERVAL: Duration = Duration::from_secs(1 * 24 * 60 * 60);
}

fn main() {
    let names = vec![
        "Ben", "Bob", "Gus", "Jim", "Joe", "Sam", "Tim", "Tom", "Ada", "Ann", "Deb",
    ];
    let surnames = vec![
        "Ali", "Ash", "Cho", "Ito", "Kim", "Lis", "Rey", "Sun", "Way", "Xie", "Zhu",
    ];
    let pasture_size_ranges = enum_map! {
        PastureKind::Open => PastureAreaMinMax { min: 10_000., max: 1_000_000. },
        PastureKind::Covered => PastureAreaMinMax { min: 1_000., max: 90_000. },
        PastureKind::Individual => PastureAreaMinMax { min: 100., max: 8_100. },
    };

    let mut ss = Snapshot::new();
    ss.species = vec![
        Species::new(0, "Angus Cow", SpeciesKind::Animal),
        Species::new(1, "Holstein Cow", SpeciesKind::Animal),
        Species::new(2, "Chicken", SpeciesKind::Animal),
        Species::new(3, "Sheep", SpeciesKind::Animal),
        Species::new(4, "Wheat", SpeciesKind::Plant),
        Species::new(5, "Corn", SpeciesKind::Plant),
        Species::new(6, "Soybeans", SpeciesKind::Plant),
    ];
    expand_pasture_vec(&mut ss.pastures, 1000, pasture_size_ranges);
    expand_herd_vec(&mut ss.herds, &ss.species, &ss.pastures);
    ss.feeding_reports = FeedingReport::generate_random(&ss.pastures);

    let birth_min = NaiveDateTime::new(
        NaiveDate::from_ymd(2000, 1, 1),
        NaiveTime::from_hms(00, 00, 00),
    );
    ss.expand_livestock_random(0, 10_000, birth_min);
    ss.expand_livestock_random(1, 1000, birth_min);
    ss.expand_livestock_random(2, 5000, birth_min);
    ss.expand_livestock_random(3, 500, birth_min);

    ss.expand_employees_random(50, &names, &surnames);

    ss.expand_health_reports_random(1000, 0.15, 0.07, 0.03);

    ss.expand_warehouses_random(16);

    ss.saveToDir(config::RESULT_DIR);
}
