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
use crate::headcount_report::generate_headcount_report_vec;
use crate::herd::*;
use crate::livestock::*;
use crate::pasture::*;
use crate::snapshot::*;
use crate::species::*;
use crate::warehouse::*;
use chrono::{Duration, Local, NaiveDate, NaiveDateTime, NaiveTime};
use enum_map::enum_map;

mod config {
    use std::time::Duration;

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
        PastureKind::Open => PastureAreaMinMax { min: 1_000., max: 10_000. },
        PastureKind::Covered => PastureAreaMinMax { min: 10., max: 900. },
        PastureKind::Individual => PastureAreaMinMax { min: 1., max: 10. },
    };
    let livestock_birth_min = NaiveDateTime::new(
        NaiveDate::from_ymd(2019, 1, 1),
        NaiveTime::from_hms(00, 00, 00),
    );
    let snapshot1_when = Local::now().naive_local() - Duration::weeks(10);

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
    expand_pasture_vec(&mut ss.pastures, 1000, pasture_size_ranges);
    expand_herd_vec(&mut ss.herds, &ss.species[..4], &ss.pastures);
    ss.feeding_reports = generate_feeding_report_vec(&ss.pastures, 1000, None, Duration::days(1));
    expand_employee_vec(&mut ss.employees, 100, &names, &surnames, 3000.0, 12000.0);
    expand_warehouse_vec(&mut ss.warehouses, 16, &ss.employees);
    expand_livestock(
        &mut ss.livestock,
        &ss.herds,
        &ss.species,
        &ss.pastures,
        livestock_birth_min,
    );
    kill_off_livestock_vec(&mut ss.livestock, 0.1, &ss.species);
    butcher_livestock_vec(&mut ss.livestock, &ss.species, snapshot1_when);
    ss.headcount_reports = generate_headcount_report_vec(
        1000,
        &ss.herds,
        &ss.employees,
        900,
        1200,
        None,
        Duration::days(1),
    );

    ss.expand_health_reports_random(1000, 0.15, 0.07, 0.03);

    ss.save_to_dir("out");
}
