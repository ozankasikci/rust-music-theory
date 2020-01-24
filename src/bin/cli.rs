use fantasy_in_rust::note::{PitchClass, Notes};
use fantasy_in_rust::chord::Chord;
use fantasy_in_rust::scale::{Mode, Scale, ScaleType};
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let input = args[1].clone();

    /*
    let (pitch, regex_match) = PitchClass::from_regex(&input).unwrap();
    let (mode, regex_match) = Mode::from_regex(&input[regex_match.end()..]).unwrap();
    eprintln!("mode = {:?}", mode);
    let scale_type = ScaleType::from_mode(&mode);
    eprintln!("scale_type = {:?}", scale_type);
    let octave = 4;

    let scale = Scale::new(scale_type, pitch, octave, Some(mode)).unwrap();
    scale.print_notes();
    */

    let chord = Chord::from_regex(&input).unwrap();
    chord.print_notes()
}
