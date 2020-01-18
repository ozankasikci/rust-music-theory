#![allow(dead_code)]
use fantasy_in_rust::note::{Note, PitchClass};
use fantasy_in_rust::scale::{Scale, ScaleType};

fn main() {
    let scale = Scale::new(ScaleType::Diatonic, PitchClass::D, 4);
    scale.notes();
}
