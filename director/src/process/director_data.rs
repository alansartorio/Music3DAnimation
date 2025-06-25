use serde::Serialize;

#[derive(Debug, Serialize, PartialEq)]
pub struct Note {
    pub delta: f64,
    pub track: u32,
    pub channel: u8,
    pub note: u8,
}

#[derive(Debug, Serialize)]
pub struct Actuator {
    pub name: String,
    pub notes: Vec<Note>,
}

#[derive(Debug, Serialize)]
pub struct DirectorData {
    pub actuators: Vec<Actuator>,
}

