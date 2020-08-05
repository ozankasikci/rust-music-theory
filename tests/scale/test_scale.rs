extern crate rust_music_theory as theory;
use theory::note::{PitchSymbol::*, *};
use theory::scale::{Mode::*, ScaleType::*, *};

fn assert_notes(pitches: &[PitchClass], notes: Vec<Note>) {
    for (i, pitch) in pitches.iter().enumerate() {
        assert_eq!(*pitch, notes[i].pitch_class);
    }
}

#[cfg(test)]
mod scale_tests {
    use super::*;

    #[test]
    fn test_all_scales_in_c() {
        let scale_tuples = [
            (
                Scale::new(Diatonic, pclass(C,0), 4, Some(Ionian)).unwrap(),
                vec![pclass(C,0), pclass(D,0), pclass(E,0), pclass(F,0), pclass(G,0), pclass(A,0), pclass(B,0), pclass(C,0)],
            ),
            (
                Scale::new(Diatonic, pclass(C,0), 4, Some(Locrian)).unwrap(),
                vec![pclass(C,0), pclass(C,1), pclass(D,1), pclass(F,0), pclass(F,1), pclass(G,1), pclass(A,1), pclass(C,0)],
            ),
            (
                Scale::new(Diatonic, pclass(B,0), 4, Some(Locrian)).unwrap(),
                vec![pclass(B,0), pclass(C,0), pclass(D,0), pclass(E,0), pclass(F,0), pclass(G,0), pclass(A,0), pclass(B,0)],
            ),
            (
                Scale::new(Diatonic, pclass(C,0), 4, Some(Dorian)).unwrap(),
                vec![pclass(C,0), pclass(D,0), pclass(D,1), pclass(F,0), pclass(G,0), pclass(A,0), pclass(A,1), pclass(C,0)],
            ),
            (
                Scale::new(Diatonic, pclass(A,0), 4, Some(Aeolian)).unwrap(),
                vec![pclass(A,0), pclass(B,0), pclass(C,0), pclass(D,0), pclass(E,0), pclass(F,0), pclass(G,0), pclass(A,0)],
            ),
            (
                Scale::new(Diatonic, pclass(C,0), 4, Some(Lydian)).unwrap(),
                vec![pclass(C,0), pclass(D,0), pclass(E,0), pclass(F,1), pclass(G,0), pclass(A,0), pclass(B,0), pclass(C,0)],
            ),
            (
                Scale::new(Diatonic, pclass(C,0), 4, Some(Mixolydian)).unwrap(),
                vec![pclass(C,0), pclass(D,0), pclass(E,0), pclass(F,0), pclass(G,0), pclass(A,0), pclass(A,1), pclass(C,0)],
            ),
            (
                Scale::new(Diatonic, pclass(C,0), 4, Some(Phrygian)).unwrap(),
                vec![pclass(C,0), pclass(C,1), pclass(D,1), pclass(F,0), pclass(G,0), pclass(G,1), pclass(A,1), pclass(C,0)],
            ),
            (
                Scale::new(ScaleType::HarmonicMinor, pclass(C,0), 4, None).unwrap(),
                vec![pclass(C,0), pclass(D,0), pclass(D,1), pclass(F,0), pclass(G,0), pclass(G,1), pclass(B,0), pclass(C,0)],
            ),
            (
                Scale::new(ScaleType::MelodicMinor, pclass(C,0), 4, None).unwrap(),
                vec![pclass(C,0), pclass(D,0), pclass(D,1), pclass(F,0), pclass(G,0), pclass(A,0), pclass(B,0), pclass(C,0)],
            ),
        ];

        for scale_tuple in scale_tuples.iter() {
            let (scale, pitches) = scale_tuple;
            assert_notes(pitches, scale.notes());

            if scale.scale_type == Diatonic {
                if let Some(mode) = scale.mode {
                    assert!(mode.is_diatonic());
                }
            }
        }
    }

    #[test]
    fn test_octave_increment() {
        let scale = Scale::new(
            ScaleType::Diatonic,
            pclass(PitchSymbol::G,0),
            5,
            Some(Mode::Mixolydian)
        ).unwrap();

        for (i, note) in scale.notes().iter().enumerate() {
            assert_eq!(note.octave, if i <= 2 { 5 } else { 6 });
        }
    }
}
