extern crate rust_music_theory as theory;
use theory::chord::{Number::*, Quality::*, *};
use theory::note::{PitchSymbol::*, *};

fn assert_notes(pitches: &[PitchClass], notes: Vec<Note>) {
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
            (Chord::new(pclass(C,0), Major, Triad), vec![pclass(C,0), pclass(E,0), pclass(G,0)]),
            (Chord::new(pclass(C,0), Minor, Triad), vec![pclass(C,0), pclass(D,1), pclass(G,0)]),
            (Chord::new(pclass(C,0), Augmented, Triad), vec![pclass(C,0), pclass(E,0), pclass(G,1)]),
            (Chord::new(pclass(C,0), Diminished, Triad), vec![pclass(C,0), pclass(D,1), pclass(F,1)]),
            (Chord::new(pclass(C,0), Major, Seventh), vec![pclass(C,0), pclass(E,0), pclass(G,0), pclass(B,0)]),
            (Chord::new(pclass(C,0), Minor, Seventh), vec![pclass(C,0), pclass(D,1), pclass(G,0), pclass(A,1)]),
            (Chord::new(pclass(C,0), Augmented, Seventh), vec![pclass(C,0), pclass(E,0), pclass(G,1), pclass(A,1)]),
            (Chord::new(pclass(C,0), Augmented, MajorSeventh), vec![pclass(C,0), pclass(E,0), pclass(G,1), pclass(B,0)]),
            (Chord::new(pclass(C,0), Diminished, Seventh), vec![pclass(C,0), pclass(D,1), pclass(F,1), pclass(A,0)]),
            (Chord::new(pclass(C,0), HalfDiminished, Seventh), vec![pclass(C,0), pclass(D,1), pclass(F,1), pclass(A,1)]),
            (Chord::new(pclass(C,0), Minor, MajorSeventh), vec![pclass(C,0), pclass(D,1), pclass(G,0), pclass(B,0)]),
            (Chord::new(pclass(C,0), Dominant, Seventh), vec![pclass(C,0), pclass(E,0), pclass(G,0), pclass(A,1)]),
        ];

        for chord_tuple in chord_tuples.iter() {
            let (chord, pitches) = chord_tuple;
            assert_notes(pitches, chord.notes());
        }
    }
}
