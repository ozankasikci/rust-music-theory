extern crate fantasy_in_rust as fir;
use fir::chord::{ChordQuality::*, ChordNumber::*, *};
use fir::note::{PitchClass::*, *};

fn assert_notes(pitches: &Vec<PitchClass>, notes: Vec<Note>) {
    for (i, pitch) in pitches.iter().enumerate() {
        assert_eq!(*pitch, notes[i].pitch_class);
    }
}

#[cfg(test)]
mod chord_tests {
    use super::*;

    #[test]
    fn test_all_chords_in_c() {
        let chord_tuples = [
            (Chord::new(C, Major, Triad), vec![C, E, G]),
            (Chord::new(C, Minor, Triad), vec![C, Ds, G]),
            (Chord::new(C, Augmented, Triad), vec![C, E, Gs]),
            (Chord::new(C, Diminished, Triad), vec![C, Ds, Fs]),
            (Chord::new(C, Major, Seventh), vec![C, E, G, B]),
            (Chord::new(C, Minor,Seventh), vec![C, Ds, G, As]),
            (Chord::new(C, Augmented,Seventh), vec![C, E, Gs, As]),
            (Chord::new(C, AugmentedMajor,Seventh), vec![C, E, Gs, B]),
            (Chord::new(C, Diminished, Seventh), vec![C, Ds, Fs, A]),
            (Chord::new(C, HalfDiminished, Seventh), vec![C, Ds, Fs, As]),
            (Chord::new(C, MinorMajor, Seventh), vec![C, Ds, G, B]),
            (Chord::new(C, Dominant, Seventh), vec![C, E, G, As]),
        ];

        for chord_tuple in chord_tuples.iter() {
            let (chord, pitches) = chord_tuple;
            assert_notes(pitches, chord.notes());
        }
    }
}
