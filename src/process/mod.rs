#![allow(clippy::collapsible_match)]

use std::{
    cell::RefCell,
    collections::{BTreeSet, HashMap, hash_map::Entry},
    rc::Rc,
};

use anyhow::Result;
use midly::{MidiMessage, Smf};

use crate::process::{
    director_data::*,
    scene::{ControlledNotes, Machine, get_demo1_scene},
};

mod director_data;
mod scene;

#[derive(Debug, Default)]
struct TrackInfo {
    track_number: u32,
    track_name: Option<String>,
    device_name: Option<String>,
    instrument_name: Option<String>,
    program_name: Option<String>,
}

#[derive(Debug)]
struct CompleteNote {
    delta: f64,
    note: scene::Note,
    track_info: Rc<RefCell<TrackInfo>>,
}
impl CompleteNote {
    pub fn into_output_note(&self) -> director_data::Note {
        director_data::Note {
            delta: self.delta,
            track: self.note.track,
            channel: self.note.channel,
            note: self.note.note,
        }
    }
}

type MachineNotes = Vec<CompleteNote>;

fn note_matches_controlled_notes(
    controlled_notes: &ControlledNotes,
    complete_note: &CompleteNote,
) -> bool {
    match controlled_notes {
        ControlledNotes::Notes(notes) => notes.contains(&complete_note.note),
        ControlledNotes::TrackByName(name) => complete_note
            .track_info
            .borrow()
            .track_name
            .as_ref()
            .is_some_and(|track_name| track_name == name),
    }
}

fn find_default_us_per_beat(smf: &Smf) -> Option<midly::num::u24> {
    smf.tracks[0].iter().find_map(|event| {
        if let midly::TrackEventKind::Meta(midly::MetaMessage::Tempo(us_per_beat)) = event.kind {
            Some(us_per_beat)
        } else {
            None
        }
    })
}

fn get_machine_note_events(smf: &Smf, machine_notes: Vec<&ControlledNotes>) -> MachineNotes {
    let default_us_per_beat = find_default_us_per_beat(smf);
    let tick_length = match smf.header.timing {
        midly::Timing::Metrical(_ticks_per_beat) => None,
        midly::Timing::Timecode(fps, divisions) => {
            Some(1f64 / (fps.as_f32() as f64) / (divisions as f64))
        }
    };
    let notes: Vec<_> = smf
        .tracks
        .iter()
        .enumerate()
        .flat_map(|(track_number, track)| {
            let mut track_info = TrackInfo::default();
            track_info.track_number = track_number as u32;
            let track_info: Rc<RefCell<TrackInfo>> = Rc::new(track_info.into());
            let mut tick_length = tick_length.or_else(|| {
                if let midly::Timing::Metrical(ticks_per_beat) = smf.header.timing
                    && let Some(default_us_per_beat) = default_us_per_beat
                {
                    Some(
                        default_us_per_beat.as_int() as f64
                            / ticks_per_beat.as_int() as f64
                            / 1e6f64,
                    )
                } else {
                    None
                }
            });
            let mut notes = vec![];
            let mut time = 0f64;
            let mut previous_note_time = time;
            let mut previous_note = None;

            for event in track {
                if let Some(tick_length) = tick_length {
                    time += event.delta.as_int() as f64 * tick_length;
                }
                match event.kind {
                    midly::TrackEventKind::Meta(meta_message) => match meta_message {
                        midly::MetaMessage::Tempo(us_per_beat) => {
                            if let midly::Timing::Metrical(ticks_per_beat) = smf.header.timing {
                                tick_length.replace(
                                    us_per_beat.as_int() as f64
                                        / ticks_per_beat.as_int() as f64
                                        / 1e6f64,
                                );
                            }
                        }
                        midly::MetaMessage::TrackName(value) => {
                            track_info
                                .borrow_mut()
                                .track_name
                                .replace(str::from_utf8(value).unwrap().to_string());
                        }
                        midly::MetaMessage::DeviceName(value) => {
                            track_info
                                .borrow_mut()
                                .device_name
                                .replace(str::from_utf8(value).unwrap().to_string());
                        }
                        midly::MetaMessage::InstrumentName(value) => {
                            track_info
                                .borrow_mut()
                                .instrument_name
                                .replace(str::from_utf8(value).unwrap().to_string());
                        }
                        midly::MetaMessage::ProgramName(value) => {
                            track_info
                                .borrow_mut()
                                .program_name
                                .replace(str::from_utf8(value).unwrap().to_string());
                        }
                        _ => (),
                    },
                    midly::TrackEventKind::Midi { channel, message } => {
                        if let MidiMessage::NoteOn { key, vel: _vel } = message {
                            let complete_note = CompleteNote {
                                note: scene::Note {
                                    track: track_number as u32,
                                    channel: channel.as_int(),
                                    note: key.as_int(),
                                },
                                track_info: track_info.clone(),
                                delta: time - previous_note_time,
                            };
                            let is_repeated_note = complete_note.delta <= 0f64
                                && previous_note.as_ref().is_some_and(|previous_note| {
                                    complete_note.note.eq(previous_note)
                                });
                            if !is_repeated_note {
                                previous_note.replace(complete_note.note);
                                if machine_notes.iter().any(|controlled_notes| {
                                    note_matches_controlled_notes(controlled_notes, &complete_note)
                                }) {
                                    notes.push(complete_note);
                                    previous_note_time = time;
                                }
                            }
                        }
                    }
                    _ => {}
                }
            }

            notes
        })
        .collect();

    notes
}

fn process_machine(notes: MachineNotes, machine: Machine) -> Vec<Actuator> {
    let mut next_index = 0usize;

    let mut track_last_note_time: HashMap<_, _> = notes
        .iter()
        .map(|note| note.track_info.borrow().track_number)
        .collect::<BTreeSet<_>>()
        .into_iter()
        .map(|track_number| (track_number, 0f64))
        .collect();

    let mut actuator_last_note_time: HashMap<_, _> = machine
        .actuators
        .iter()
        .map(|actuator| (actuator.name.as_str(), 0f64))
        .collect();

    let note_assignations: Vec<_> = notes
        .iter()
        .map(|note| {
            let track_time = track_last_note_time
                .get_mut(&note.track_info.borrow().track_number)
                .unwrap();
            *track_time += note.delta;

            let best_actuator = (0..machine.actuators.len())
                .map(|i| (i + next_index) % machine.actuators.len())
                .map(|i| &machine.actuators[i])
                .find(|actuator| note_matches_controlled_notes(&actuator.controlled_notes, note))
                .unwrap()
                .name
                .as_str();

            let last_note_time = actuator_last_note_time.get_mut(best_actuator).unwrap();
            let delta = *track_time - *last_note_time;
            *last_note_time = *track_time;

            next_index += 1;
            let mut note_copy = note.into_output_note();
            note_copy.delta = delta;

            (best_actuator, note_copy)
        })
        .collect();

    let mut actuators = HashMap::new();

    for (note_assignation, note) in note_assignations {
        match actuators.entry(note_assignation) {
            Entry::Vacant(entry) => {
                entry.insert(vec![note]);
            }
            Entry::Occupied(mut entry) => {
                entry.get_mut().push(note);
            }
        }
    }

    actuators
        .into_iter()
        .map(|(name, notes)| Actuator {
            name: name.to_string(),
            notes,
        })
        .collect()
}

fn debug_track_metadata(smf: &Smf) {
    for (i, track) in smf.tracks.iter().enumerate() {
        println!("track {} has {} events", i, track.len());
        let mut used_channel_notes: BTreeSet<(u8, u8)> = BTreeSet::new();
        for event in track {
            match event.kind {
                midly::TrackEventKind::Meta(meta_message) => match meta_message {
                    midly::MetaMessage::TrackName(value) => {
                        eprintln!("TrackName: {}", str::from_utf8(value).unwrap())
                    }
                    midly::MetaMessage::Text(value) => {
                        eprintln!("Text: {}", str::from_utf8(value).unwrap())
                    }
                    midly::MetaMessage::DeviceName(value) => {
                        eprintln!("DeviceName: {}", str::from_utf8(value).unwrap())
                    }
                    midly::MetaMessage::InstrumentName(value) => {
                        eprintln!("InstrumentName: {}", str::from_utf8(value).unwrap())
                    }
                    midly::MetaMessage::ProgramName(value) => {
                        eprintln!("ProgramName: {}", str::from_utf8(value).unwrap())
                    }
                    _ => {}
                },
                midly::TrackEventKind::Midi { channel, message } => {
                    if let MidiMessage::NoteOn { key, vel: _ } = message {
                        used_channel_notes.insert((channel.as_int(), key.as_int()));
                    }
                }
                _ => (),
            }
        }
        dbg!(used_channel_notes);
    }
}

pub fn process(smf: Smf) -> Result<DirectorData> {
    let scene = get_demo1_scene();

    debug_track_metadata(&smf);

    let mut actuators = vec![];

    for machine in scene.machines {
        let machine_notes: Vec<_> = machine
            .actuators
            .iter()
            .map(|actuator| &actuator.controlled_notes)
            .collect();

        let machine_note_events = get_machine_note_events(&smf, machine_notes);

        let machine_actuators = process_machine(machine_note_events, machine);
        actuators.extend(machine_actuators);
    }

    let output = DirectorData { actuators };

    Ok(output)
}
