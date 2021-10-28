use std::{
    fs::OpenOptions,
    ops::Deref,
    path::{Path, PathBuf},
};

use chrono::{Local, NaiveDateTime};
use rand::{
    distributions::{Slice, Uniform},
    prelude::Distribution,
    seq::index::IndexVecIntoIter,
};
use serde::Serialize;

use crate::{
    config,
    employee::{self, random_account_number, random_pesel, Employee},
    feeding_report::FeedingReport,
    health_report::HealthReport,
    herd::{self, Herd},
    livestock::{self, Livestock},
    pasture::Pasture,
    species::{self, Species},
};

pub struct Snapshot {
    pub pastures: Vec<Pasture>,
    pub species: Vec<Species<'static>>,
    pub herds: Vec<Herd>,
    pub feeding_reports: Vec<FeedingReport>,
    pub livestock: Vec<Livestock<'static>>,
    pub employees: Vec<Employee<'static, 'static>>,
    pub health_reports: Vec<HealthReport>,
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
        }
    }

    pub fn expand_livestock_random(
        &mut self,
        herd_id: usize,
        count: usize,
        birth_min: NaiveDateTime,
    ) {
        let herd = &mut self.herds[herd_id];
        let mut rng = rand::thread_rng();

        let birth_span = Local::now()
            .naive_local()
            .signed_duration_since(birth_min)
            .to_std()
            .unwrap();
        let distribution = Uniform::new(std::time::Duration::new(0, 0), birth_span);

        let indicices = self.livestock.len()..(self.livestock.len() + count);
        for id in indicices {
            let birth_offset = chrono::Duration::from_std(distribution.sample(&mut rng)).unwrap();
            self.livestock.push(Livestock::new(
                id as u32,
                birth_min + birth_offset,
                None,
                None,
                herd,
            ));
            herd.size += 1;
        }
    }

    pub fn expand_employees_random(
        &mut self,
        count: usize,
        names: &Vec<&'static str>,
        surnames: &Vec<&'static str>,
    ) {
        let mut rng = rand::thread_rng();
        let name_distribution = Slice::new(names).unwrap();
        let surname_distribution = Slice::new(surnames).unwrap();
        let salary_distribution = Uniform::new(3000.0, 12000.0);

        let indicies = self.employees.len()..(self.employees.len() + count);
        for id in indicies {
            self.employees.push(Employee::new(
                id as u32,
                name_distribution.sample(&mut rng),
                surname_distribution.sample(&mut rng),
                random_pesel(),
                random_account_number(),
                salary_distribution.sample(&mut rng),
            ));
        }
    }

    pub fn expand_health_reports_random(
        &mut self,
        count_per_herd: usize,
        ill_max_pct: f32,
        severly_ill_max_pct: f32,
        terminal_max_pct: f32,
    ) {
        let interval = chrono::Duration::from_std(config::FEEDING_REPORT_INTERVAL).unwrap();
        let first_report: NaiveDateTime =
            Local::now().naive_local() - interval * count_per_herd as i32;

        let mut rng = rand::thread_rng();
        let employee_distribution = Slice::new(&self.employees).unwrap();
        let ill_distribution = Uniform::new(0.0, ill_max_pct);
        let severly_ill_distribution = Uniform::new(0.0, severly_ill_max_pct);
        let terminal_distribution = Uniform::new(0.0, terminal_max_pct);

        for herd in self.herds.iter() {
            let mut date = first_report.clone();
            let indicies = self.health_reports.len()..(self.health_reports.len() + count_per_herd);
            for id in indicies {
                let doctor = employee_distribution.sample(&mut rng);
                self.health_reports.push(HealthReport::new(
                    id as u32,
                    doctor,
                    herd,
                    date,
                    ill_distribution.sample(&mut rng),
                    severly_ill_distribution.sample(&mut rng),
                    terminal_distribution.sample(&mut rng),
                ));
                date += interval;
            }
        }
    }

    pub fn saveToDir(&self, dir: &str) {
        let dir = Path::new(dir);
        saveToFile(dir.join("pasture").with_extension("csv"), &self.pastures);
        saveToFile(dir.join("species").with_extension("csv"), &self.species);
        saveToFile(dir.join("herd").with_extension("csv"), &self.herds);
        saveToFile(
            dir.join("feeding_report").with_extension("csv"),
            &self.feeding_reports,
        );
        saveToFile(dir.join("livestock").with_extension("csv"), &self.livestock);
        saveToFile(dir.join("employee").with_extension("csv"), &self.employees);
        saveToFile(
            dir.join("health_report").with_extension("csv"),
            &self.health_reports,
        );
    }
}

fn saveToFile<P, T>(path: P, data: &Vec<T>)
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
