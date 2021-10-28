mod feeding_report;
mod herd;
mod livestock;
mod pasture;
mod snapshot;
mod species;

use std::{fs::OpenOptions, path::Path};

use chrono::{NaiveDate, NaiveDateTime, NaiveTime};
use feeding_report::*;
use herd::*;
use livestock::*;
use pasture::*;
use snapshot::*;
use species::*;

mod config {
    use std::time::Duration;

    pub const RESULT_DIR: &str = "out";

    pub const FEEDING_REPORT_COUNT: i32 = 1_000;
    pub const FEEDING_REPORT_INTERVAL: Duration = Duration::from_secs(1 * 24 * 60 * 60);
}

fn main() {
    let mut snapshot = Snapshot::new();
    snapshot.pastures = vec![
        Pasture::new(0, 1440000., PastureKind::Open),
        Pasture::new(1, 40000., PastureKind::Covered),
        Pasture::new(2, 1000., PastureKind::Individual),
        Pasture::new(3, 10000., PastureKind::Open),
    ];
    snapshot.species = vec![
        Species::new(0, "Angus Cow", SpeciesKind::Animal),
        Species::new(1, "Holstein Cow", SpeciesKind::Animal),
        Species::new(2, "Chicken", SpeciesKind::Animal),
        Species::new(3, "Sheep", SpeciesKind::Animal),
        Species::new(4, "Wheat", SpeciesKind::Plant),
        Species::new(5, "Corn", SpeciesKind::Plant),
        Species::new(6, "Soybeans", SpeciesKind::Plant),
    ];
    snapshot.herds = vec![
        Herd::new(0, &snapshot.pastures[0], &snapshot.species[0]),
        Herd::new(1, &snapshot.pastures[1], &snapshot.species[1]),
        Herd::new(2, &snapshot.pastures[2], &snapshot.species[2]),
        Herd::new(3, &snapshot.pastures[3], &snapshot.species[3]),
    ];
    snapshot.feeding_reports = FeedingReport::generate_random(&snapshot.pastures);

    let birth_min = NaiveDateTime::new(
        NaiveDate::from_ymd(2000, 1, 1),
        NaiveTime::from_hms(00, 00, 00),
    );
    snapshot.expand_livestock_random(0, 10_000, birth_min);
    snapshot.saveToDir(config::RESULT_DIR);
}
