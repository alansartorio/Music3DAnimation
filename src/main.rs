use anyhow::Result;
use clap::Parser;
use midly::{MidiMessage, Smf};
use serde::Serialize;
use std::fs::{self, File};
use std::path::{Path, PathBuf};

#[derive(Serialize)]
struct Note {
    delta: f64,
    track: u64,
    note: u64,
}

#[derive(Serialize)]
struct Actuator {
    name: String,
    notes: Vec<Note>,
}

#[derive(Serialize)]
struct DirectorData {
    actuators: Vec<Actuator>,
}

pub fn load(path: impl AsRef<Path>) {
    // Load bytes into a buffer
    let bytes = fs::read(path).unwrap();

    // Parse bytes in a separate step
    let smf = Smf::parse(&bytes).unwrap();

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
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    midi: PathBuf,
    #[arg(short, long)]
    output: PathBuf,
}

fn main() -> Result<()> {
    let args: Args = Args::parse();

    load(args.midi);

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

    let file = File::create(args.output)?;
    serde_json::to_writer_pretty(file, &output)?;

    Ok(())
}
