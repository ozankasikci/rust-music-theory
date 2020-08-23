extern crate rust_music_theory as theory;
use theory::note::{PitchSymbol::*, *};
use theory::scale::{Mode::*, ScaleType::*, *};

fn assert_notes(symbols: &[PitchSymbol], notes: Vec<Note>) {
    for (i, symbol) in symbols.iter().enumerate() {
        assert_eq!(Pitch::from(*symbol), notes[i].pitch);
    }
}

#[cfg(test)]
mod scale_tests {
    use super::*;

    #[test]
    fn test_all_scales_in_c() {
        let scale_tuples = [
            ((Diatonic, Some(Ionian)), vec![C, D, E, F, G, A, B, C]),
            ((Diatonic, Some(Dorian)), vec![C, D, Ds, F, G, A, As, C]),
            ((Diatonic, Some(Phrygian)), vec![C, Cs, Ds, F, G, Gs, As, C]),
            ((Diatonic, Some(Lydian)), vec![C, D, E, Fs, G, A, B, C]),
            ((Diatonic, Some(Mixolydian)), vec![C, D, E, F, G, A, As, C]),
            ((Diatonic, Some(Aeolian)), vec![C, D, Ds, F, G, Gs, As, C]),
            ((Diatonic, Some(Locrian)), vec![C, Cs, Ds, F, Fs, Gs, As, C]),
            (
                (ScaleType::HarmonicMinor, None),
                vec![C, D, Ds, F, G, Gs, B, C],
            ),
            (
                (ScaleType::MelodicMinor, None),
                vec![C, D, Ds, F, G, A, B, C],
            )
        ];

        for (scale_tuple, pitches) in scale_tuples.iter() {
            let (scale_type, mode) = scale_tuple;
            let scale_ascending =
                Scale::new(*scale_type, Pitch::from(C), 4, *mode, Direction::Ascending).unwrap();
            assert_notes(pitches, scale_ascending.notes());

            let scale_descending =
                Scale::new(*scale_type, Pitch::from(C), 4, *mode, Direction::Descending).unwrap();
            let mut pitches_descending = pitches.clone();
            pitches_descending.reverse();
            assert_notes(&pitches_descending, scale_descending.notes());

            if scale_ascending.scale_type == Diatonic {
                if let Some(mode) = scale_ascending.mode {
                    assert!(mode.is_diatonic());
                }
            }
        }
    }

    #[test]
    fn test_octave_increment() {
        let scale = Scale::new(
            ScaleType::Diatonic,
            Pitch::new(NoteLetter::G, 0),
            5,
            Some(Mode::Mixolydian),
            Direction::Ascending,
        )
        .unwrap();

        for (i, note) in scale.notes().iter().enumerate() {
            assert_eq!(note.octave, if i <= 2 { 5 } else { 6 });
        }
    }
}
