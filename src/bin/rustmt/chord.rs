use rust_music_theory::{
    chord::{Chord, Number, Quality},
    note::{Notes, PitchClass},
};
use structopt::StructOpt;

const AVAILABLE_CHORDS: [&str; 22] = [
    "Major Triad",
    "Minor Triad",
    "Suspended2 Triad",
    "Suspended4 Triad",
    "Augmented Triad",
    "Diminished Triad",
    "Major Seventh",
    "Minor Seventh",
    "Augmented Seventh",
    "Augmented Major Seventh",
    "Diminished Seventh",
    "Half Diminished Seventh",
    "Minor Major Seventh",
    "Dominant Seventh",
    "Dominant Ninth",
    "Major Ninth",
    "Dominant Eleventh",
    "Major Eleventh",
    "Minor Eleventh",
    "Dominant Thirteenth",
    "Major Thirteenth",
    "Minor Thirteenth",
];

#[derive(StructOpt, Debug)]
#[structopt(about = "Provides information for the specified chord")]
pub enum Command {
    #[structopt(alias = "l")]
    List(ListCommand),
    #[structopt(alias = "n")]
    Notes(NotesCommand),
}

impl Command {
    pub fn execute(self) {
        match self {
            Command::List(list_command) => list_command.execute(),
            Command::Notes(note_command) => note_command.execute(),
        }
    }
}

#[derive(StructOpt, Debug)]
pub struct ListCommand {}

impl ListCommand {
    pub fn execute(self) {
        println!("Available chords:");
        for chord in &AVAILABLE_CHORDS {
            println!(" - {}", chord);
        }
    }
}

#[derive(StructOpt, Debug)]
pub struct NotesCommand {
    pitch_class: PitchClass,
    quality: Quality,
    number: Number,
}

impl NotesCommand {
    pub fn execute(self) {
        let chord = Chord::new(self.pitch_class, self.quality, self.number);
        chord.print_notes();
    }
}
