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

    // Actor methods
    pub fn get_all_actors(&self) -> Vec<&Actor> {
        self.actors.iter().collect()
    }

    pub fn get_actor_by_id(&self, id: i32) -> Option<&Actor> {
        self.actors.iter().find(|&a| a.id == id)
    }

    pub fn get_all_doctors_by_actor(&self, actor: Actor) -> Vec<&Doctor> {
        self.doctors.iter().filter(|&d| d.primary_actor == actor.id).collect()
    }

    // Companion methods
    pub fn get_all_companions(&self) -> Vec<&Companion> {
        self.companions.iter().collect()
    }

    pub fn get_companion_by_id(&self, id: i32) -> Option<&Companion> {
        self.companions.iter().find(|&c| c.id == id)
    }

    pub fn get_serials_by_companion(&self, companion: Companion) -> Vec<&Serial> {
        let serial_ids: Vec<i32> = self.serials_companions.iter()
            .filter(|&sc| sc.companion_id == companion.id)
            .map(|sc| sc.serial_id)
            .collect();

        self.serials.iter()
            .filter(|&s| serial_ids.contains(&s.id))
            .collect()
    }

    // Director methods
    pub fn get_all_directors(&self) -> Vec<&Director> {
        self.directors.iter().collect()
    }

    pub fn get_director_by_id(&self, id: i32) -> Option<&Director> {
        self.directors.iter().find(|&d| d.id == id)
    }

    pub fn get_serials_by_director(&self, director: Director) -> Vec<&Serial> {
        let serial_ids: Vec<i32> = self.serials_directors.iter()
            .filter(|&sd| sd.director_id == director.id)
            .map(|sd| sd.serial_id)
            .collect();

        self.serials.iter()
            .filter(|&s| serial_ids.contains(&s.id))
            .collect()
    }

    // Doctor methods
    pub fn get_all_doctors(&self) -> Vec<&Doctor> {
        self.doctors.iter().collect()
    }

    pub fn get_doctor_by_id(&self, id: i32) -> Option<&Doctor> {
        self.doctors.iter().find(|&d| d.id == id)
    }

    pub fn get_actors_who_played_doctor(&self, doctor: Doctor) -> Vec<&Actor> {
        self.actors.iter()
            .filter(|&a| a.id == doctor.primary_actor)
            .collect()
    }

    pub fn get_serials_by_doctor(&self, doctor: Doctor) -> Vec<&Serial> {
        let serial_ids: Vec<i32> = self.serials_doctors.iter()
            .filter(|&sd| sd.doctor_id == doctor.id)
            .map(|sd| sd.serial_id)
            .collect();

        self.serials.iter()
            .filter(|&s| serial_ids.contains(&s.id))
            .collect()
    }

    // Episode methods
    pub fn get_all_episodes(&self) -> Vec<&Episode> {
        self.episodes.iter().collect()
    }

    pub fn get_episode_by_id(&self, id: i32) -> Option<&Episode> {
        self.episodes.iter().find(|&e| e.id == id)
    }

    // Season methods
    pub fn get_all_seasons(&self) -> Vec<&Season> {
        self.seasons.iter().collect()
    }

    pub fn get_season_by_id(&self, id: i32) -> Option<&Season> {
        self.seasons.iter().find(|&s| s.id == id)
    }

    pub fn get_serials_by_season(&self, season: Season) -> Vec<&Serial> {
        self.serials.iter()
            .filter(|&s| s.season_id == season.id)
            .collect()
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