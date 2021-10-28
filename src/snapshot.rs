use std::{
    fs::OpenOptions,
    ops::Deref,
    path::{Path, PathBuf},
};

use serde::Serialize;

use crate::{
    config,
    feeding_report::FeedingReport,
    herd::{self, Herd},
    livestock::{self, Livestock},
    pasture::Pasture,
    species::{self, Species},
};

pub struct Snapshot<'a, 'b> {
    pub pastures: Vec<Pasture>,
    pub species: Vec<Species<'a>>,
    pub herds: Vec<Herd>,
    pub feeding_reports: Vec<FeedingReport>,
    pub livestock: Vec<Livestock<'b>>,
}

impl<'a, 'b> Snapshot<'a, 'b> {
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
