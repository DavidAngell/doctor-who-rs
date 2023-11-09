#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate reqwest;

use std::io::Read;
use anyhow::Result;

#[derive(Debug, Deserialize)]
pub struct DoctorWhoData {
    pub actors: Vec<Actor>,
    pub companions: Vec<Companion>,
    pub directors: Vec<Director>,
    pub doctors: Vec<Doctor>,
    pub episodes: Vec<Episode>,
    pub seasons: Vec<Season>,
    pub serials: Vec<Serial>,
    pub writers: Vec<Writer>,
    pub serials_companions: Vec<SerialCompanion>,
    pub serials_directors: Vec<SerialDirector>,
    pub serials_doctors: Vec<SerialDoctor>,
    pub serials_writers: Vec<SerialWriter>,
}

impl DoctorWhoData {
    pub fn new() -> Result<Self> {
        let mut file = std::fs::File::open("src/doctor_who.json")?;
        let mut data = String::new();
        file.read_to_string(&mut data).unwrap();
        
        let json = serde_json::from_str(&data)?;
        Ok(json)
    }
}

#[derive(Debug, Deserialize)]
pub struct Actor {
    id: i32,
    name: String,
    gender: String,
}

#[derive(Debug, Deserialize)]
pub struct Companion {
    id: i32,
    name: String,
    actor: i32,
}

#[derive(Debug, Deserialize)]
pub struct Director {
    id: i32,
    name: String,
}

#[derive(Debug, Deserialize)]
pub struct Doctor {
    id: i32,
    incarnation: String,
    primary_actor: i32,
}

#[derive(Debug, Deserialize)]
pub struct Episode {
    id: i32,
    title: String,
    serial_id: i32,
    story: Option<String>,
    episode_order: String,
    original_air_date: String,
    runtime: String,
    uk_viewers_mm: f32,
    appreciation_index: i32,
    missing: i32,
    recreated: i32,
}

#[derive(Debug, Deserialize)]
pub struct Season {
    id: i32,
    name: String,
}

#[derive(Debug, Deserialize)]
pub struct Serial {
    id: i32,
    season_id: i32,
    story: String,
    serial: i32,
    title: String,
    production_code: String,
}

#[derive(Debug, Deserialize)]
pub struct Writer {
    id: i32,
    name: String,
}

#[derive(Debug, Deserialize)]
pub struct SerialCompanion {
    serial_id: i32,
    companion_id: i32,
}

#[derive(Debug, Deserialize)]
pub struct SerialDirector {
    serial_id: i32,
    director_id: i32,
}

#[derive(Debug, Deserialize)]
pub struct SerialDoctor {
    serial_id: i32,
    doctor_id: i32,
}

#[derive(Debug, Deserialize)]
pub struct SerialWriter {
    serial_id: i32,
    writer_id: i32,
}