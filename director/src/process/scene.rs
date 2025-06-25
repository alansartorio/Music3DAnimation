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
        machines: vec![
            Machine {
                name: "Marimba".to_string(),
                actuators: vec![Actuator {
                    name: "Marimba".to_string(),
                    controlled_notes: ControlledNotes::TrackByName("Mallets".to_string()),
                    costs: ActuatorCosts {
                        switch_note: 0.5,
                        play_note: 0.1,
                    },
                }],
            },
            Machine {
                name: "Lead".to_string(),
                actuators: vec![Actuator {
                    name: "Lead".to_string(),
                    controlled_notes: ControlledNotes::TrackByName("Analog Brass 2".to_string()),
                    costs: ActuatorCosts {
                        switch_note: 0.5,
                        play_note: 0.1,
                    },
                }],
            },
            Machine {
                name: "Chords".to_string(),
                actuators: (0..3)
                    .map(|n| Actuator {
                        name: format!("Chords Voice {n}"),
                        controlled_notes: ControlledNotes::TrackByName("Synth Brazz 1".to_string()),
                        costs: ActuatorCosts {
                            switch_note: 0.5,
                            play_note: 0.1,
                        },
                    })
                    .collect(),
            },
            Machine {
                name: "Basskick".to_string(),
                actuators: vec![Actuator {
                    name: "Basskick".to_string(),
                    controlled_notes: ControlledNotes::TrackByName("Kicklong.ds".to_string()),
                    costs: ActuatorCosts {
                        switch_note: 0.5,
                        play_note: 0.1,
                    },
                }],
            },
            Machine {
                name: "Drummer".to_string(),
                actuators: vec![
                    Actuator {
                        name: "HiHatOpen".to_string(),
                        controlled_notes: ControlledNotes::TrackByName("Hat_o.ds".to_string()),
                        costs: ActuatorCosts {
                            switch_note: 0.5,
                            play_note: 0.1,
                        },
                    },
                    Actuator {
                        name: "HiHatClosed".to_string(),
                        controlled_notes: ControlledNotes::TrackByName("Hat_c.ds".to_string()),
                        costs: ActuatorCosts {
                            switch_note: 0.5,
                            play_note: 0.1,
                        },
                    },
                    Actuator {
                        name: "Snare".to_string(),
                        controlled_notes: ControlledNotes::TrackByName("Snare.ds".to_string()),
                        costs: ActuatorCosts {
                            switch_note: 0.5,
                            play_note: 0.1,
                        },
                    },
                ],
            },
        ],
    }
}
