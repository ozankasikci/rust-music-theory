use rust_music_theory::{
    note::{Notes, PitchClass},
    scale::{Direction, Mode, Scale, ScaleType},
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
    "Harmonic Minor",
    "Melodic Minor",
];

#[derive(StructOpt, Debug)]
#[structopt(about = "Provides information for the specified scale")]
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
        println!("Available Scales:");
        for scale in &AVAILABLE_SCALES {
            println!(" - {}", scale);
        }
    }
}

#[derive(StructOpt, Debug)]
pub struct NotesCommand {
    tonic: PitchClass,
    mode: Mode,
    #[structopt(long, short, conflicts_with = "descending")]
    _ascending: bool,
    #[structopt(long, short)]
    descending: bool,
}

impl NotesCommand {
    pub fn execute(self) {
        let scale = Scale::new(
            ScaleType::from(self.mode),
            self.tonic,
            4,
            Some(self.mode),
            if self.descending {
                Direction::Ascending
            } else {
                Direction::Descending
            },
        )
        .unwrap();
        scale.print_notes();
    }
}
