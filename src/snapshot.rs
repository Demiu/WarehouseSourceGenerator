use std::{fs::OpenOptions, ops::Deref, path::Path};

use serde::Serialize;

use crate::{
    employee::Employee, feeding_report::FeedingReport, headcount_report::HeadcountReport,
    health_report::HealthReport, herd::Herd, livestock::Livestock, pasture::Pasture,
    species::Species, warehouse::Warehouse,
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
