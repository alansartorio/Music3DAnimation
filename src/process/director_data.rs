use serde::Serialize;

#[derive(Serialize)]
pub struct Note {
    pub delta: f64,
    pub track: u64,
    pub note: u64,
}

#[derive(Serialize)]
pub struct Actuator {
    pub name: String,
    pub notes: Vec<Note>,
}

#[derive(Serialize)]
pub struct DirectorData {
    pub actuators: Vec<Actuator>,
}

