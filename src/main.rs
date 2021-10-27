mod feeding_report;
mod herd;
mod livestock;
mod pasture;
mod species;

use feeding_report::*;
use herd::*;
use livestock::*;
use pasture::*;
use species::*;

mod config {
    use std::time::Duration;

    pub const FEEDING_REPORT_COUNT: i32 = 1_000_000;
    pub const FEEDING_REPORT_INTERVAL: Duration = Duration::from_secs(1 * 24 * 60);
}

fn main() {
    let pastures = vec![
        Pasture::new(0, 1440000., PastureKind::Open),
        Pasture::new(1, 40000., PastureKind::Covered),
        Pasture::new(2, 1000., PastureKind::Individual),
        Pasture::new(3, 10000., PastureKind::Open),
    ];
    let species = vec![
        Species::new(0, "Angus Cow", SpeciesKind::Animal),
        Species::new(1, "Holstein Cow", SpeciesKind::Animal),
        Species::new(2, "Chicken", SpeciesKind::Animal),
        Species::new(3, "Sheep", SpeciesKind::Animal),
        Species::new(4, "Wheat", SpeciesKind::Plant),
        Species::new(5, "Corn", SpeciesKind::Plant),
        Species::new(6, "Soybeans", SpeciesKind::Plant),
    ];
    let herds = vec![
        Herd::new(0, &pastures[0], &species[0]),
        Herd::new(1, &pastures[1], &species[1]),
        Herd::new(2, &pastures[2], &species[2]),
        Herd::new(3, &pastures[3], &species[3]),
    ];
    let feeding_reports = FeedingReport::generate_random(&pastures);
}
