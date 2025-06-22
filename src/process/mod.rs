use anyhow::Result;
use midly::{MidiMessage, Smf};

use crate::process::director_data::*;

mod director_data;

pub fn process(smf: Smf) -> Result<DirectorData> {
    for (i, track) in smf.tracks.iter().enumerate() {
        println!("track {} has {} events", i, track.len());
        println!("  events:");
        for event in track {
            if let midly::TrackEventKind::Midi { channel, message } = event.kind {
                println!(
                    "  {} {}",
                    channel,
                    match message {
                        MidiMessage::NoteOn { key, vel } => format!("ON {key} ({vel})"),
                        MidiMessage::NoteOff { key, vel } => format!("OFF {key} ({vel})"),
                        _ => "Other".to_string(),
                    }
                );
            }
        }
    }

    let output = DirectorData {
        actuators: vec![Actuator {
            name: "Target".to_string(),
            notes: vec![
                Note {
                    delta: 0.5,
                    track: 1,
                    note: 50,
                },
                Note {
                    delta: 1.5,
                    track: 1,
                    note: 51,
                },
            ],
        }],
    };

    Ok(output)
}
