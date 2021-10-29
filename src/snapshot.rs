use std::{
    fs::OpenOptions,
    ops::Deref,
    path::Path,
};

use chrono::{Local, NaiveDateTime};
use rand::{
    distributions::{Slice, Uniform},
    prelude::Distribution,
};
use serde::Serialize;

use crate::{
    config,
    employee::Employee,
    feeding_report::FeedingReport,
    health_report::HealthReport,
    herd::Herd,
    livestock::Livestock,
    pasture::Pasture,
    species::Species,
    warehouse::Warehouse,
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
                let ill_count = (herd.size as f32 * ill_distribution.sample(&mut rng)) as u32;
                let severly_ill_count =
                    (herd.size as f32 * severly_ill_distribution.sample(&mut rng)) as u32;
                let terminal_count =
                    (herd.size as f32 * terminal_distribution.sample(&mut rng)) as u32;
                let healthy_count = herd.size - ill_count - severly_ill_count - terminal_count;
                self.health_reports.push(HealthReport::new(
                    id,
                    doctor,
                    herd,
                    date,
                    healthy_count,
                    ill_count,
                    severly_ill_count,
                    terminal_count,
                ));
                date += interval;
            }
        }
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
