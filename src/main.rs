#![allow(dead_code)]
use fantasy_in_rust::note::{Note, PitchClass};
use fantasy_in_rust::scale::{Scale, ScaleType, Mode};

fn main() {
    let scale = Scale::new(ScaleType::Diatonic, PitchClass::A, 4, Mode::Ionian);
    println!("{:#?}",scale.notes())
}
