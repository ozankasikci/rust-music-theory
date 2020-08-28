use rust_music_theory::{
    note::Notes,
    scale::Scale,
};
use structopt::StructOpt;

const AVAILABLE_SCALES: [&str; 9] = [
    "Major|Ionian",
    "Minor|Aeolian",
    "Dorian",
    "Phrygian",
    "Lydian",
    "Mixolydian",
    "Locrian",
    "HarmonicMinor",
    "MelodicMinor",
];

#[derive(StructOpt, Debug)]
#[structopt(about = "Provides information for the specified scale")]
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
#[structopt(alias = "l", about = "Prints out the available scales")]
pub struct ListCommand {}

impl ListCommand {
    pub fn execute(self) {
        println!("Available Scales:");
        for scale in &AVAILABLE_SCALES {
            println!(" - {}", scale);
        }
    }
}

#[derive(StructOpt, Debug)]
#[structopt(alias = "n", about = "Prints out the notes of the <scale>", help = "Examples:\n- C melodic minor\n- D# dorian")]
pub struct NotesCommand {
    #[structopt(name = "scale", required = true)]
    scale_strings: Vec<String>,
}

impl NotesCommand {
    pub fn execute(self) {
        let scale_string = self.scale_strings.join(" ");
        let scale = Scale::from_regex(&scale_string); //TODO: reintegrate direction (directly into from_str/regex)
        if let Ok(scale) = scale {
            scale.print_notes();
        } else {
            use structopt::clap::*;
            Error::with_description("Couldn't parse scale", ErrorKind::ValueValidation).exit(); //TODO: Better Errors
        }
    }
}
