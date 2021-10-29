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

use crate::employee::*;
use crate::feeding_report::*;
use crate::headcount_report::*;
use crate::health_report::*;
use crate::herd::*;
use crate::livestock::*;
use crate::pasture::*;
use crate::snapshot::*;
use crate::species::*;
use crate::warehouse::*;
use chrono::{Duration, Local, NaiveDate, NaiveDateTime, NaiveTime};
use enum_map::enum_map;

fn main() {
    let names = [
        "Ben", "Bob", "Gus", "Jim", "Joe", "Sam", "Tim", "Tom", "Ada", "Ann", "Deb",
    ];
    let surnames = [
        "Ali", "Ash", "Cho", "Ito", "Kim", "Lis", "Rey", "Sun", "Way", "Xie", "Zhu",
    ];
    let pasture_size_ranges = enum_map! {
        PastureKind::Open => PastureAreaMinMax { min: 1_000., max: 10_000. },
        PastureKind::Covered => PastureAreaMinMax { min: 10., max: 900. },
        PastureKind::Individual => PastureAreaMinMax { min: 1., max: 10. },
    };
    let livestock_birth_min = NaiveDateTime::new(
        NaiveDate::from_ymd(2019, 1, 1),
        NaiveTime::from_hms(00, 00, 00),
    );

    let report_interval = Duration::days(1);
    let snapshot2_when = Local::now().naive_local();
    let snapshot1_when = snapshot2_when - Duration::days(6 * 30 + 3);
    let initial_when = snapshot1_when - report_interval * 1000;

    let mut ss = Snapshot::new();
    ss.species = vec![
        Species::new(
            0,
            "Angus Cow",
            SpeciesKind::Animal,
            639,
            Some(SpeciesAreaRequirements {
                pasture_kind_to_req_area: enum_map! {
                    PastureKind::Open => 10.,
                    PastureKind::Covered => 8.,
                    PastureKind::Individual => 2.,
                },
            }),
        ),
        Species::new(
            1,
            "Holstein Cow",
            SpeciesKind::Animal,
            639,
            Some(SpeciesAreaRequirements {
                pasture_kind_to_req_area: enum_map! {
                    PastureKind::Open => 10.,
                    PastureKind::Covered => 8.,
                    PastureKind::Individual => 2.,
                },
            }),
        ),
        Species::new(
            2,
            "Chicken",
            SpeciesKind::Animal,
            42,
            Some(SpeciesAreaRequirements {
                pasture_kind_to_req_area: enum_map! {
                    PastureKind::Open => 1.,
                    PastureKind::Covered => 1.,
                    PastureKind::Individual => 0.25,
                },
            }),
        ),
        Species::new(
            3,
            "Sheep",
            SpeciesKind::Animal,
            304,
            Some(SpeciesAreaRequirements {
                pasture_kind_to_req_area: enum_map! {
                    PastureKind::Open => 7.5,
                    PastureKind::Covered => 6.,
                    PastureKind::Individual => 3.,
                },
            }),
        ),
        Species::new(4, "Wheat", SpeciesKind::Plant, 210, None),
        Species::new(5, "Corn", SpeciesKind::Plant, 80, None),
        Species::new(6, "Soybeans", SpeciesKind::Plant, 55, None),
    ];
    ss.expand(
        initial_when,
        snapshot1_when,
        report_interval,
        1000,
        pasture_size_ranges,
        ..3,
        100,
        &names,
        &surnames,
        3000.0,
        12000.0,
        16,
        0.1,
        9000,
        12000,
        0.1,
        0.07,
        0.02,
    );
    ss.save_to_dir("out/snapshot1");

    ss.species.push(Species::new(
        ss.species.len(),
        "Pig",
        SpeciesKind::Animal,
        167,
        Some(SpeciesAreaRequirements {
            pasture_kind_to_req_area: enum_map! {
                PastureKind::Open => 9.,
                PastureKind::Covered => 7.,
                PastureKind::Individual => 4.,
            },
        }),
    ));
}
