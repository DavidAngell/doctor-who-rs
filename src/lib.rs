#[macro_use]
extern crate serde_derive;
extern crate serde_json;

use std::io::Read;
use anyhow::Result;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Actor {
    pub id: i32,
    pub name: String,
    pub gender: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Companion {
    pub id: i32,
    pub name: String,
    pub actor: i32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Director {
    pub id: i32,
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Doctor {
    pub id: i32,
    pub incarnation: String,
    pub primary_actor: i32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Episode {
    pub id: i32,
    pub title: String,
    pub serial_id: i32,
    pub story: Option<String>,
    pub episode_order: String,
    pub original_air_date: String,
    pub runtime: String,
    pub uk_viewers_mm: f32,
    pub appreciation_index: i32,
    pub missing: i32,
    pub recreated: i32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Season {
    pub id: i32,
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Serial {
    pub id: i32,
    pub season_id: i32,
    pub story: String,
    pub serial: i32,
    pub title: String,
    pub production_code: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Writer {
    pub id: i32,
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SerialCompanion {
    pub serial_id: i32,
    pub companion_id: i32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SerialDirector {
    pub serial_id: i32,
    pub director_id: i32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SerialDoctor {
    pub serial_id: i32,
    pub doctor_id: i32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SerialWriter {
    pub serial_id: i32,
    pub writer_id: i32,
}

#[derive(Debug, Serialize, Deserialize)]
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
    pub fn get_actors(&self) -> Vec<&Actor> {
        self.actors.iter().collect()
    }

    pub fn get_actor(&self, id: i32) -> Option<&Actor> {
        self.actors.iter().find(|&a| a.id == id)
    }

    pub fn get_actor_doctors(&self, actor: Actor) -> Vec<&Doctor> {
        self.doctors.iter().filter(|&d| d.primary_actor == actor.id).collect()
    }

    // Companion methods
    pub fn get_companions(&self) -> Vec<&Companion> {
        self.companions.iter().collect()
    }

    pub fn get_companion(&self, id: i32) -> Option<&Companion> {
        self.companions.iter().find(|&c| c.id == id)
    }

    pub fn get_companion_actors(&self, companion: &Companion) -> Vec<&Actor> {
        self.actors.iter()
            .filter(|&a| a.id == companion.actor)
            .collect()
    }

    pub fn get_companion_serials(&self, companion: &Companion) -> Vec<&Serial> {
        let serial_ids: Vec<i32> = self.serials_companions.iter()
            .filter(|&sc| sc.companion_id == companion.id)
            .map(|sc| sc.serial_id)
            .collect();

        self.serials.iter()
            .filter(|&s| serial_ids.contains(&s.id))
            .collect()
    }

    // Director methods
    pub fn get_directors(&self) -> Vec<&Director> {
        self.directors.iter().collect()
    }

    pub fn get_director(&self, id: i32) -> Option<&Director> {
        self.directors.iter().find(|&d| d.id == id)
    }

    pub fn get_director_serials(&self, director: &Director) -> Vec<&Serial> {
        let serial_ids: Vec<i32> = self.serials_directors.iter()
            .filter(|&sd| sd.director_id == director.id)
            .map(|sd| sd.serial_id)
            .collect();

        self.serials.iter()
            .filter(|&s| serial_ids.contains(&s.id))
            .collect()
    }

    // Doctor methods
    pub fn get_doctors(&self) -> Vec<&Doctor> {
        self.doctors.iter().collect()
    }

    pub fn get_doctor(&self, id: i32) -> Option<&Doctor> {
        self.doctors.iter().find(|&d| d.id == id)
    }

    pub fn get_doctor_actors(&self, doctor: Doctor) -> Vec<&Actor> {
        self.actors.iter()
            .filter(|&a| a.id == doctor.primary_actor)
            .collect()
    }

    pub fn get_doctor_serials(&self, doctor: &Doctor) -> Vec<&Serial> {
        let serial_ids: Vec<i32> = self.serials_doctors.iter()
            .filter(|&sd| sd.doctor_id == doctor.id)
            .map(|sd| sd.serial_id)
            .collect();

        self.serials.iter()
            .filter(|&s| serial_ids.contains(&s.id))
            .collect()
    }

    // Episode methods
    pub fn get_episodes(&self) -> Vec<&Episode> {
        self.episodes.iter().collect()
    }

    pub fn get_episode(&self, id: i32) -> Option<&Episode> {
        self.episodes.iter().find(|&e| e.id == id)
    }

    // Season methods
    pub fn get_seasons(&self) -> Vec<&Season> {
        self.seasons.iter().collect()
    }

    pub fn get_season(&self, id: i32) -> Option<&Season> {
        self.seasons.iter().find(|&s| s.id == id)
    }

    pub fn get_season_serial(&self, season: Season) -> Option<&Serial> {
        self.serials.iter().find(|&s| s.season_id == season.id)
    }

    // Serial methods
    pub fn get_serials(&self) -> Vec<&Serial> {
        self.serials.iter().collect()
    }

    pub fn get_serial(&self, id: i32) -> Option<&Serial> {
        self.serials.iter().find(|&s| s.id == id)
    }

    pub fn get_serial_episodes(&self, serial: &Serial) -> Vec<&Episode> {
        self.episodes.iter()
            .filter(|&e| e.serial_id == serial.id)
            .collect()
    }

    pub fn get_serial_directors(&self, serial: &Serial) -> Vec<&Director> {
        let director_ids: Vec<i32> = self.serials_directors.iter()
            .filter(|&sd| sd.serial_id == serial.id)
            .map(|sd| sd.director_id)
            .collect();

        self.directors.iter()
            .filter(|&d| director_ids.contains(&d.id))
            .collect()
    }

    pub fn get_serial_writers(&self, serial: &Serial) -> Vec<&Writer> {
        let writer_ids: Vec<i32> = self.serials_writers.iter()
            .filter(|&sw| sw.serial_id == serial.id)
            .map(|sw| sw.writer_id)
            .collect();

        self.writers.iter()
            .filter(|&w| writer_ids.contains(&w.id))
            .collect()
    }

    pub fn get_serial_doctors(&self, serial: &Serial) -> Vec<&Doctor> {
        let doctor_ids: Vec<i32> = self.serials_doctors.iter()
            .filter(|&sd| sd.serial_id == serial.id)
            .map(|sd| sd.doctor_id)
            .collect();

        self.doctors.iter()
            .filter(|&d| doctor_ids.contains(&d.id))
            .collect()
    }

    pub fn get_serial_companions(&self, serial: &Serial) -> Vec<&Companion> {
        let companion_ids: Vec<i32> = self.serials_companions.iter()
            .filter(|&sc| sc.serial_id == serial.id)
            .map(|sc| sc.companion_id)
            .collect();

        self.companions.iter()
            .filter(|&c| companion_ids.contains(&c.id))
            .collect()
    }

    // Writer methods
    pub fn get_writers(&self) -> Vec<&Writer> {
        self.writers.iter().collect()
    }

    pub fn get_writer(&self, id: i32) -> Option<&Writer> {
        self.writers.iter().find(|&w| w.id == id)
    }

    pub fn get_writer_serials(&self, writer: &Writer) -> Vec<&Serial> {
        let serial_ids: Vec<i32> = self.serials_writers.iter()
            .filter(|&sw| sw.writer_id == writer.id)
            .map(|sw| sw.serial_id)
            .collect();

        self.serials.iter()
            .filter(|&s| serial_ids.contains(&s.id))
            .collect()
    }

    pub fn get_writer_episodes(&self, writer: &Writer) -> Vec<&Episode> {
        let serial_ids: Vec<i32> = self.serials_writers.iter()
            .filter(|&sw| sw.writer_id == writer.id)
            .map(|sw| sw.serial_id)
            .collect();

        let episode_ids: Vec<i32> = self.episodes.iter()
            .filter(|&e| serial_ids.contains(&e.serial_id))
            .map(|e| e.id)
            .collect();

        self.episodes.iter()
            .filter(|&e| episode_ids.contains(&e.id))
            .collect()
    }
}