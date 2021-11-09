use chrono::{DateTime, TimeZone, Utc};
use serde::{Deserialize, Serialize};
use std::ffi::OsStr;
use std::fs;
use std::fs::File;
use std::io::BufReader;
use std::path::{Path, PathBuf};

use anyhow::Result;
use thiserror::Error;

pub(crate) struct RaceService {
    races_dir: PathBuf,
    archived_dir: PathBuf,
}

impl RaceService {

    pub(crate) fn new<P: Into<PathBuf>, Q: Into<PathBuf>>(races_dir: P, archived_dir: Q) -> Self {
        let races_dir: PathBuf = races_dir.into();
        let archived_dir: PathBuf = archived_dir.into();
        if !races_dir.exists() {
            if let Err(e) = fs::create_dir_all(&races_dir) {
                panic!("Error creating dir {:?} : {}", races_dir, e);
            }
        } else if !races_dir.is_dir() {
            panic!("{:?} is not a directory", races_dir);
        }
        if !archived_dir.exists() {
            if let Err(e) = fs::create_dir_all(&archived_dir) {
                panic!("Error creating dir {:?} : {}", archived_dir, e);
            }
        } else if !archived_dir.is_dir() {
            panic!("{:?} is not a directory", archived_dir);
        }
        RaceService { races_dir, archived_dir }
    }

    pub(crate) async fn list(&self, archived: Option<bool>) -> Result<Vec<Race>> {
        let mut res = Vec::new();

        let (dir, archived) = if let Some(true) = archived {
            (&self.archived_dir, true)
        } else {
            (&self.races_dir, false)
        };

        let paths = fs::read_dir(dir)?;

        for entry in paths {
            if let Ok(entry) = entry {
                if let Ok(metadata) = entry.metadata() {
                    if metadata.is_file() {
                        println!("entry {:?}", entry.path());
                        if let Some(ext) = entry.path().extension() {
                            if ext == OsStr::new("yaml") {
                                let file = File::open(entry.path()).unwrap();
                                let reader = BufReader::new(file);

                                // Read the JSON contents of the file as an instance of `AppInfo`.
                                match serde_yaml::from_reader(reader) {
                                    Ok(race) => {
                                        let mut race: Race = race;
                                        race.id = Some(entry.path().file_prefix().unwrap().to_string_lossy().to_string());
                                        race.archived = archived;
                                        res.push(race);
                                    },
                                    Err(e) => {
                                        println!("Error reading file {:?} : {:?}", entry, e);
                                    }
                                }
                            }
                        }
                    }
                } else {
                    println!("Couldn't get metadata for {:?}", entry.path());
                }
            }
        }

        res.sort_by(|a, b| {
            let a = a.start_time.unwrap_or(Utc{}.ymd(1970, 1, 1).and_hms_nano(0, 0, 0, 0));
            let b = b.start_time.unwrap_or(Utc{}.ymd(1970, 1, 1).and_hms_nano(0, 0, 0, 0));
            b.cmp(&a)
        });
        Ok(res)
    }

    pub(crate) async fn get(&self, race_id: String) -> Result<Option<Race>> {

        let mut path = self.races_dir.join(format!("{}.yaml", race_id));
        let mut archived = false;
        if !path.exists() {
            path = self.archived_dir.join(format!("{}.yaml", race_id));
            archived = true;
            if !path.exists() {
                return Ok(None)
            }
        }

        let reader = BufReader::new(File::open(&path)?);

        // Read the JSON contents of the file as an instance of `AppInfo`.
        let race: Option<Race> = serde_yaml::from_reader(reader)?;
        let race = race.map(|mut r: Race| {
            r.id = Some(race_id);
            r.archived = archived;
            r
        });
        Ok(race)
    }

    fn get_id(&self, race: &Race) -> Result<String> {
        match &race.id {
            Some(id) => {
                Ok(id.clone())
            }
            None => {
                Err(RaceError::IdIsMandatory().into())
            }
        }
    }

    pub(crate) async fn create(&self, race: &Race) -> Result<()> {
        let id = self.get_id(race)?;
        let path = self.races_dir.join(format!("{}.yaml", id));
        if path.exists() {
            Err(RaceError::AlreadyExists(id).into())
        } else {
            match self.save_race(&path, race) {
                Ok(()) => Ok(()),
                Err(e) => {
                    println!("Error saving race {:?} : {}", path, e);
                    Err(e.into())
                }
            }
        }
    }

    pub(crate) async fn update(&self, race_id: String, race: &Race) -> Result<()> {
        let mut path = self.races_dir.join(format!("{}.yaml", race_id));
        if !path.exists() {
            return Err(RaceError::NotFound(race_id).into())
        } else {

            if let Some(id) = &race.id {
                if id != &race_id {
                    // the id change. must remove old file and create new one.
                    match fs::remove_file(&path) {
                        Ok(_) => {},
                        Err(e) => {
                            println!("Error removing file {:?} : {}", path, e);
                            return Err(e.into());
                        }
                    }
                    path = self.races_dir.join(format!("{}.yaml", id))
                }
            }

            match self.save_race(&path, race) {
                Ok(()) => Ok(()),
                Err(e) => {
                    println!("Error saving race {:?} : {}", path, e);
                    Err(e.into())
                }
            }
        }
    }

    pub(crate) async fn delete(&self, race_id: String) -> Result<()> {
        let mut path = self.races_dir.join(format!("{}.yaml", race_id));
        if !path.exists() {
            path = self.archived_dir.join(format!("{}.yaml", race_id));
            if !path.exists() {
                return Err(RaceError::NotFound(race_id).into())
            }
        }

        match fs::remove_file(&path) {
            Ok(_) => Ok(()),
            Err(e) => {
                println!("Error removing file {:?} : {}", path, e);
                Err(e.into())
            }
        }
    }

    pub(crate) async fn archive(&self, race_id: String) -> Result<()> {
        let path = self.races_dir.join(format!("{}.yaml", race_id));
        if !path.exists() {
            Err(RaceError::NotFound(race_id).into())
        } else {
            let archived = self.archived_dir.join(format!("{}.yaml", race_id));
            Self::rename(&path, &archived)
        }
    }

    pub(crate) async fn restore(&self, race_id: String) -> Result<()> {
        let archived = self.archived_dir.join(format!("{}.yaml", race_id));
        if !archived.exists() {
            Err(RaceError::NotFound(race_id).into())
        } else {
            let path = self.races_dir.join(format!("{}.yaml", race_id));
            if path.exists() {
                Err(RaceError::AlreadyExists(race_id).into())
            } else {
                Self::rename(&archived, &path)
            }
        }
    }

    fn rename(from: &Path, to: &Path) -> Result<()> {
        match fs::rename(from, to) {
            Ok(_) => Ok(()),
            Err(e) => {
                println!("Error moving file {:?} to {:?} : {}", from, to, e);
                Err(e.into())
            }
        }
    }

    fn save_race(&self, path: &Path, race: &Race) -> Result<()> {

        let f = std::fs::OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(path)?;
        serde_yaml::to_writer(f, race)?;

        Ok(())
    }
}

#[derive(Error, Debug)]
pub enum RaceError {
    #[error("Race {0} already exists.")]
    AlreadyExists(String),
    #[error("Race {0} does not exist.")]
    NotFound(String),
    #[error("Id is mandatory")]
    IdIsMandatory(),
}


#[derive(Deserialize, Serialize, Debug)]
pub(crate) struct Race {
    #[serde(skip_serializing)]
    pub(crate) id: Option<String>,
    pub(crate) race_id: Option<String>,
    #[serde(default, skip_serializing)]
    pub(crate) archived: bool,
    pub(crate) name: String,
    #[serde(rename = "shortName")]
    pub(crate) short_name: Option<String>,
    pub(crate) boat: String,
    pub(crate) start_time: Option<DateTime<Utc>>,
    pub(crate) end_time: Option<DateTime<Utc>>,
    pub(crate) start: LatLon,
    pub(crate) waypoints: Vec<Waypoint>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) ice_limits: Option<Limits>,
}

#[derive(Deserialize, Serialize, Debug)]
pub(crate) struct LatLon {
    pub(crate) lat: f64,
    pub(crate) lon: f64,
}

#[derive(Deserialize, Serialize, Debug)]
pub(crate) struct Limits {
    pub(crate) north: Vec<LatLon>,
    pub(crate) south: Vec<LatLon>,
    #[serde(rename = "maxLat")]
    pub(crate) max_lat: f64,
    #[serde(rename = "minLat")]
    pub(crate) min_lat: f64,
}

#[derive(Deserialize, Serialize, Debug)]
pub(crate) struct Waypoint {
    pub(crate) name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) radius: Option<u8>,
    pub(crate) latlons: Vec<LatLon>,
    #[serde(rename = "toAvoid", skip_serializing_if = "Option::is_none")]
    pub(crate) to_avoid: Option<Vec<Vec<Vec<f64>>>>
}



