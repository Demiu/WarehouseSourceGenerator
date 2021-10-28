use std::{
    fs::OpenOptions,
    ops::Deref,
    path::{Path, PathBuf},
};

use chrono::{Local, NaiveDateTime};
use rand::{distributions::Uniform, prelude::Distribution};
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
    pub const fn new() -> Self {
        Snapshot {
            pastures: vec![],
            species: vec![],
            herds: vec![],
            feeding_reports: vec![],
            livestock: vec![],
        }
    }

    pub fn expand_livestock_random(
        &mut self,
        herd_id: usize,
        count: usize,
        birth_min: NaiveDateTime,
    ) {
        let herd = &self.herds[herd_id];
        let mut rng = rand::thread_rng();

        let birth_span = Local::now()
            .naive_local()
            .signed_duration_since(birth_min)
            .to_std()
            .unwrap();
        let distribution = Uniform::new(std::time::Duration::new(0, 0), birth_span);

        let indicices = self.livestock.len()..(count + self.livestock.len());
        for id in indicices {
            let birth_offset = chrono::Duration::from_std(distribution.sample(&mut rng)).unwrap();
            self.livestock.push(Livestock::new(
                id as u32,
                birth_min + birth_offset,
                None,
                None,
                herd,
            ));
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
