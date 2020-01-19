#![allow(dead_code)]
use fantasy_in_rust::chord::{Chord, ChordQuality};
use fantasy_in_rust::note::{Note, PitchClass};
use fantasy_in_rust::scale::{Mode, Scale, ScaleType};

fn main() {
    let scale = Scale::new(ScaleType::Diatonic, PitchClass::Fs, 4, Some(Mode::Locrian)).unwrap();

    let chord = Chord::new(PitchClass::C, ChordQuality::DiminishedSeventh).notes();
    println!("{:#?}", chord);
    let chord = Chord::new(PitchClass::C, ChordQuality::HalfDiminishedSeventh).notes();
    println!("{:#?}", chord);
    let chord = Chord::new(PitchClass::C, ChordQuality::MinorMajorSeventh).notes();
    println!("{:#?}", chord);
    let chord = Chord::new(PitchClass::G, ChordQuality::DominantSeventh).notes();
    println!("{:#?}", chord);
}
