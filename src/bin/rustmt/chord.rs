use rust_music_theory::{
    chord::Chord,
    note::Notes,
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
    List(ListCommand),
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
#[structopt(alias = "l", about = "Prints out the available chords")]
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
#[structopt(alias = "n", about = "Prints out the notes of the <chord>", help = "Examples:\n- C minor\n- Ab augmented major seventh\n- F# dominant seventh / C#\n- C/1")]
pub struct NotesCommand {
    #[structopt(name = "chord", required = true)]
    chord_strings: Vec<String>,
}

impl NotesCommand {
    pub fn execute(self) {
        let chord_string = self.chord_strings.join(" ");
        let chord = Chord::from_regex(&chord_string);
        if let Ok(chord) = chord {
            chord.print_notes();
        } else {
            use structopt::clap::*;
            Error::with_description("Couldn't parse chord", ErrorKind::ValueValidation).exit(); //TODO: Better Errors
        }
    }
}
