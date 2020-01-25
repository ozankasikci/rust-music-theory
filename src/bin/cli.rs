use rust_music_theory::chord::Chord;
use rust_music_theory::note::Notes;
use std::env;

use clap::{App, Arg};

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

fn main() {
    let matches = App::new("RustMusicTheory")
        .version("0.1")
        .author("Ozan Kaşıkçı")
        .about("A music theory guide")
        .subcommand(
            App::new("scale")
                .subcommand(
                    App::new("list")
                )
        )
        .get_matches();


    match matches.subcommand() {
        ("scale", Some(scale_matches)) => {
            match scale_matches.subcommand() {
                ("list", _) => {
                    println!("Available Scales:");
                    for scale in &AVAILABLE_SCALES {
                       println!(" - {}", scale);
                    }
                }
                _ => unreachable!()
            }
        }
        _ => unreachable!()
    }


}
