extern crate fantasy_in_rust as fir;
use fir::note::{*, PitchClass::*};
use fir::chord::{*, ChordQuality::*};

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
            (Chord::new(C, MajorTriad), vec![C, E, G]),
            (Chord::new(C, MinorTriad), vec![C, Ds, G]),
            (Chord::new(C, AugmentedTriad), vec![C, E, Gs]),
            (Chord::new(C, DiminishedTriad), vec![C, Ds, Fs]),
            (Chord::new(C, MajorSeventh), vec![C, E, G, B]),
            (Chord::new(C, MinorSeventh), vec![C, Ds, G, As]),
            (Chord::new(C, AugmentedSeventh), vec![C, E, Gs, As]),
            (Chord::new(C, AugmentedMajorSeventh), vec![C, E, Gs, B]),
            (Chord::new(C, DiminishedSeventh), vec![C, Ds, Fs, A]),
            (Chord::new(C, HalfDiminishedSeventh), vec![C, Ds, Fs, As]),
            (Chord::new(C, MinorMajorSeventh), vec![C, Ds, G, B]),
            (Chord::new(C, DominantSeventh), vec![C, E, G, As]),
        ];

        for chord_tuple in chord_tuples.iter() {
            let (chord, pitches) = chord_tuple;
            assert_notes(pitches, chord.notes());
        }
    }
}