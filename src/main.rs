use fantasy_in_rust::note::{Note, PitchClass};
use fantasy_in_rust::scale::{Scale};

fn main() {
    let not = Note{pitch_class: PitchClass::As, octave: 1};
    println!("{:?}", not);
}