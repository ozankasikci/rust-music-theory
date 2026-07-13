use clap::{App, Arg, ArgMatches};
use rust_music_theory::chord::{Chord, SUPPORTED_CHORD_SYNTAX};
use rust_music_theory::note::Notes;
use rust_music_theory::scale::{Direction, Mode, Scale};

const STANDALONE_SCALES: [&str; 5] = [
    "Pentatonic Major",
    "Pentatonic Minor",
    "Blues",
    "Chromatic",
    "Whole Tone",
];

fn scale_command(scale_matches: &ArgMatches) -> Result<(), String> {
    use Direction::*;
    match scale_matches.subcommand() {
        ("list", _) => {
            println!("Available Scales:");
            for mode in Mode::heptatonic_modes() {
                println!(" - {}", mode.canonical_name());
            }
            for scale in &STANDALONE_SCALES {
                println!(" - {}", scale);
            }
            Ok(())
        }
        _ => {
            let scale_args = scale_matches
                .values_of("args")
                .map(|values| values.collect::<Vec<_>>().join(" "))
                .filter(|value| !value.trim().is_empty())
                .ok_or_else(|| "missing scale".to_string())?;

            let descending = scale_matches.is_present("descending");
            let direction = if descending { Descending } else { Ascending };

            let scale = Scale::from_regex_in_direction(&scale_args, direction)
                .map_err(|error| error.to_string())?;
            scale.print_notes();
            Ok(())
        }
    }
}

fn joined_args(matches: &ArgMatches) -> Result<String, String> {
    matches
        .values_of("args")
        .map(|values| values.collect::<Vec<_>>().join(" "))
        .filter(|value| !value.trim().is_empty())
        .ok_or_else(|| "missing chord symbol".to_string())
}

fn chord_command(chord_matches: &ArgMatches) -> Result<(), String> {
    match chord_matches.subcommand() {
        ("list", _) => {
            println!("Supported chord syntax:");
            for syntax in SUPPORTED_CHORD_SYNTAX {
                println!(" - {}", syntax);
            }
            Ok(())
        }
        ("normalize", Some(normalize_matches)) => {
            let symbol = joined_args(normalize_matches)?;
            let chord = Chord::parse(&symbol).map_err(|error| error.to_string())?;
            println!("{}", chord.canonical_symbol());
            Ok(())
        }
        _ => {
            let chord_args = joined_args(chord_matches)?;
            let chord = Chord::parse(&chord_args).map_err(|error| error.to_string())?;
            chord.print_notes();
            Ok(())
        }
    }
}

fn main() {
    let matches = App::new("RustMusicTheory")
        .version(env!("CARGO_PKG_VERSION"))
        .author("Ozan Kaşıkçı")
        .about("A music theory guide")
        .subcommand(
            App::new("scale")
                .about("Provides information for the specified scale")
                .subcommand(App::new("list").about("Prints out the available scales"))
                .arg(
                    Arg::with_name("args")
                        .help("scale args, examples:\nC melodic minor\nD# dorian")
                        .multiple(true),
                )
                .arg(
                    Arg::with_name("descending")
                        .help("list scale in descending order")
                        .short("d")
                        .long("descending"),
                ),
        )
        .subcommand(
            App::new("chord")
                .about("Provides information for the specified chord")
                .subcommand(App::new("list").about("Prints out the available chords"))
                .subcommand(
                    App::new("normalize")
                        .about("Prints the canonical ASCII form of a chord symbol")
                        .arg(
                            Arg::with_name("args")
                                .help("chord symbol, for example C7(b9,#11)")
                                .required(true)
                                .multiple(true),
                        ),
                )
                .arg(
                    Arg::with_name("args")
                        .help("chord args, examples:\nC minor\nAb augmented major seventh\nF# dominant seventh / C#\nC/1")
                        .multiple(true),
                ),
        )
        .get_matches();

    let result = match matches.subcommand() {
        ("scale", Some(scale_matches)) => scale_command(scale_matches),

        ("chord", Some(chord_matches)) => chord_command(chord_matches),

        _ => {
            println!("Please use the help command to see the available commands");
            Ok(())
        }
    };

    if let Err(message) = result {
        eprintln!("error: {}", message);
        std::process::exit(2);
    }
}
