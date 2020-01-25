use clap::{App, Arg};
use rust_music_theory::scale::Scale;
use rust_music_theory::note::Notes;

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
                .arg(
                    Arg::with_name("args")
                        .help("scale args")
                        .required(true)
                        .multiple(true)
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
                _ => {}
            }

            let scale_args = scale_matches.values_of("args")
                .unwrap()
                .collect::<Vec<_>>()
                .join(" ");

            let scale = Scale::from_regex(&scale_args).unwrap();
            scale.print_notes();
        }
        
        _ => unreachable!()
    }


}
