use std::collections::HashSet;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct Note {
    pub track: u32,
    pub channel: u8,
    pub note: u8,
}

impl Note {
    fn range(
        track: u32,
        channel: u8,
        notes: impl Iterator<Item = u8>,
    ) -> impl Iterator<Item = Note> {
        notes.map(move |note| Note {
            track,
            channel,
            note,
        })
    }

}

#[derive(Debug)]
pub enum ControlledNotes {
    TrackByName(String),
    Notes(HashSet<Note>),
}

#[derive(Debug)]
pub struct ActuatorCosts {
    pub switch_note: f64,
    pub play_note: f64,
}

#[derive(Debug)]
pub struct Actuator {
    pub name: String,
    pub controlled_notes: ControlledNotes,
    pub costs: ActuatorCosts,
}

#[derive(Debug)]
pub struct Machine {
    pub name: String,
    pub actuators: Vec<Actuator>,
}

#[derive(Debug)]
pub struct Scene {
    pub machines: Vec<Machine>,
}

pub fn get_demo1_scene() -> Scene {
    Scene {
        machines: vec![Machine {
            name: "Drummer".to_string(),
            actuators: vec![Actuator {
                name: "Left Drumstick".to_string(),
                controlled_notes: ControlledNotes::TrackByName("Snare.ds".to_string()),
                costs: ActuatorCosts {
                    switch_note: 0.5,
                    play_note: 0.1,
                },
            }],
        }],
    }
}
