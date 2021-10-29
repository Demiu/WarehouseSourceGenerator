use std::{
    fs::OpenOptions,
    ops::{Deref, Index},
    path::Path,
    slice::SliceIndex,
};

use chrono::{Duration, NaiveDateTime};
use enum_map::EnumMap;
use serde::Serialize;

use crate::{
    employee::{self, expand_employee_vec, Employee},
    feeding_report::{expand_feeding_report_vec, FeedingReport},
    headcount_report::{expand_headcount_report_vec, HeadcountReport},
    health_report::{expand_health_report_vec_for_headcount_vec, HealthReport},
    herd::{expand_herd_vec, Herd},
    livestock::{butcher_livestock_vec, expand_livestock, kill_off_livestock_vec, Livestock},
    pasture::{expand_pasture_vec, Pasture, PastureAreaMinMax, PastureKind},
    species::Species,
    warehouse::{expand_warehouse_vec, Warehouse},
};

pub struct Snapshot {
    pub pastures: Vec<Pasture>,
    pub species: Vec<Species<'static>>,
    pub herds: Vec<Herd>,
    pub feeding_reports: Vec<FeedingReport>,
    pub livestock: Vec<Livestock>,
    pub employees: Vec<Employee<'static, 'static>>,
    pub health_reports: Vec<HealthReport>,
    pub warehouses: Vec<Warehouse>,
    pub headcount_reports: Vec<HeadcountReport>,
}

impl Snapshot {
    pub const fn new() -> Self {
        Snapshot {
            pastures: vec![],
            species: vec![],
            herds: vec![],
            feeding_reports: vec![],
            livestock: vec![],
            employees: vec![],
            health_reports: vec![],
            warehouses: vec![],
            headcount_reports: vec![],
        }
    }

    pub fn expand<T>(
        &mut self,
        from_when: NaiveDateTime,
        to_when: NaiveDateTime,
        reports_interval: Duration,
        new_pastures: usize,
        pasture_ranges: EnumMap<PastureKind, PastureAreaMinMax>,
        species_for_herds: T,
        hired_employees_count: usize,
        employee_names: &[&'static str],
        employee_surnames: &[&'static str],
        employee_salary_min: f32,
        employee_salary_max: f32,
        new_warehouse_count: usize,
        kill_off_pct: f32,
        headcount_min_count: u32,
        headcount_max_count: u32,
        ill_max_pct: f32,
        severly_ill_max_pct: f32,
        terminal_max_pct: f32,
    ) where
        T: SliceIndex<[Species<'static>], Output = [Species<'static>]>,
    {
        let old_pasture_count = self.pastures.len(); // we're only generating herds for new pastures
        let old_headcount_report_count = self.headcount_reports.len(); // only generate health reports for new headcounts
        expand_pasture_vec(&mut self.pastures, new_pastures, &pasture_ranges);
        expand_herd_vec(
            &mut self.herds,
            &self.pastures[old_pasture_count..],
            &self.species[species_for_herds],
        );
        expand_feeding_report_vec(
            &mut self.feeding_reports,
            &self.pastures,
            from_when,
            to_when,
            reports_interval,
        );
        expand_employee_vec(
            &mut self.employees,
            hired_employees_count,
            employee_names,
            employee_surnames,
            employee_salary_min,
            employee_salary_max,
        );
        expand_warehouse_vec(&mut self.warehouses, new_warehouse_count, &self.employees);
        expand_livestock(
            &mut self.livestock,
            &self.herds,
            &self.species,
            &self.pastures,
            from_when,
            to_when,
        );
        kill_off_livestock_vec(&mut self.livestock, kill_off_pct, &self.species, to_when);
        butcher_livestock_vec(&mut self.livestock, &self.species, to_when);
        expand_headcount_report_vec(
            &mut self.headcount_reports,
            &self.herds,
            &self.employees,
            headcount_min_count,
            headcount_max_count,
            from_when,
            to_when,
            reports_interval,
        );
        expand_health_report_vec_for_headcount_vec(
            &mut self.health_reports,
            &self.headcount_reports[old_headcount_report_count..],
            &self.employees,
            &self.herds,
            ill_max_pct,
            severly_ill_max_pct,
            terminal_max_pct,
        );
    }

    pub fn save_to_dir(&self, dir: &str) {
        let dir = Path::new(dir);
        save_to_file(dir.join("pasture").with_extension("csv"), &self.pastures);
        save_to_file(dir.join("species").with_extension("csv"), &self.species);
        save_to_file(dir.join("herd").with_extension("csv"), &self.herds);
        save_to_file(
            dir.join("feeding_report").with_extension("csv"),
            &self.feeding_reports,
        );
        save_to_file(dir.join("livestock").with_extension("csv"), &self.livestock);
        save_to_file(dir.join("employee").with_extension("csv"), &self.employees);
        save_to_file(
            dir.join("health_report").with_extension("csv"),
            &self.health_reports,
        );
        save_to_file(
            dir.join("warehouse").with_extension("csv"),
            &self.warehouses,
        );
        save_to_file(
            dir.join("headcount_report").with_extension("csv"),
            &self.headcount_reports,
        );
    }
}

fn save_to_file<P, T>(path: P, data: &Vec<T>)
where
    P: Deref<Target = Path> + AsRef<Path>,
    T: Serialize,
{
    std::fs::create_dir_all(path.parent().unwrap()).unwrap();
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .truncate(true)
        .open(path)
        .unwrap();
    let mut writer = csv::WriterBuilder::new()
        .has_headers(false)
        .from_writer(file);
    for elem in data.iter() {
        writer.serialize(elem).unwrap();
    }
}
