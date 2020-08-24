extern crate rust_music_theory as theory;
use theory::chord::{Number::*, Quality::*, *};
use theory::note::{PitchSymbol::*, *};

fn assert_notes(symbols: &[PitchSymbol], notes: Vec<Note>) {
    for (i, symbol) in symbols.iter().enumerate() {
        assert_eq!(Pitch::from(*symbol), notes[i].pitch);
    }
}

#[cfg(test)]
mod chord_tests {
    use super::*;

    #[test]
    fn test_all_chords_in_c() {
        let chord_tuples = [
            ((C, Major, Triad), vec![C, E, G]),
            ((C, Minor, Triad), vec![C, Ds, G]),
            ((C, Augmented, Triad), vec![C, E, Gs]),
            ((C, Diminished, Triad), vec![C, Ds, Fs]),
            ((C, Major, Seventh), vec![C, E, G, B]),
            ((C, Minor, Seventh), vec![C, Ds, G, As]),
            ((C, Augmented, Seventh), vec![C, E, Gs, As]),
            ((C, Augmented, MajorSeventh), vec![C, E, Gs, B]),
            ((C, Diminished, Seventh), vec![C, Ds, Fs, A]),
            ((C, HalfDiminished, Seventh), vec![C, Ds, Fs, As]),
            ((C, Minor, MajorSeventh), vec![C, Ds, G, B]),
            ((C, Dominant, Seventh), vec![C, E, G, As]),
        ];

        for (chord, pitches) in chord_tuples.iter() {
            let symbols = &mut pitches.clone();
            for inversion in 0..pitches.len() {
                assert_notes(
                    &symbols,
                    Chord::with_inversion(Pitch::from(chord.0), chord.1, chord.2, inversion as u8).notes(),
                );
                symbols.rotate_left(1);
            }
        }
    }

    #[test]
    fn test_inversion_octaves() {
        let chord_desc = (G, Major, Ninth);
        let octaves = [
            [4u8, 4, 5, 5, 5],
            [4, 5, 5, 5, 6],
            [4, 4, 4, 5, 5],
            [4, 4, 5, 5, 6],
            [4, 5, 5, 6, 6],
        ];
        for inversion in 0..octaves[0].len() {
            let notes =
                Chord::with_inversion(Pitch::from(chord_desc.0), chord_desc.1, chord_desc.2, inversion as u8)
                    .notes();
            assert_eq!(
                notes
                    .into_iter()
                    .map(|note| note.octave)
                    .collect::<Vec<u8>>(),
                octaves[inversion]
            );
        }
    }

    #[test]
    fn test_regex() {
        let chord = Chord::from_regex("F major");
        assert!(chord.is_ok());
        let chord = chord.unwrap();
        assert_notes(&vec![F, A, C], chord.notes());
        assert_eq!(chord.inversion, 0);
    }

    #[test]
    fn test_inversion_regex() {
        let chord = Chord::from_regex("F/C");
        let chord_num = Chord::from_regex("F/2");
        assert!(chord.is_ok());
        assert!(chord_num.is_ok());
        let chord = chord.unwrap();
        let chord_num = chord_num.unwrap();
        assert_notes(&vec![C, F, A], chord.notes());
        assert_notes(&vec![C, F, A], chord_num.notes());
        assert_eq!(chord.inversion, 2);
        assert_eq!(chord_num.inversion, 2);
    }
}
