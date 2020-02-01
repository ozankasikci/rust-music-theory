extern crate rust_music_theory as theory;
use theory::note::{PitchClass::*, Accidental::*, *};
use theory::scale::{Mode::*, ScaleType::*, *};

fn assert_notes(pitches: &Vec<PitchClass>, notes: Vec<Note>) {
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
                Scale::new(Diatonic, C, 4, Some(Ionian), None).unwrap(),
                vec![C, D, E, F, G, A, B, C],
            ),
            (
                Scale::new(Diatonic, C, 4, Some(Locrian), None).unwrap(),
                vec![C, Cs, Ds, F, Fs, Gs, As, C],
            ),
            (
                Scale::new(Diatonic, B, 4, Some(Locrian), None).unwrap(),
                vec![B, C, D, E, F, G, A, B],
            ),
            (
                Scale::new(Diatonic, C, 4, Some(Dorian), None).unwrap(),
                vec![C, D, Ds, F, G, A, As, C],
            ),
            (
                Scale::new(Diatonic, A, 4, Some(Aeolian), None).unwrap(),
                vec![A, B, C, D, E, F, G, A],
            ),
            (
                Scale::new(Diatonic, C, 4, Some(Lydian), None).unwrap(),
                vec![C, D, E, Fs, G, A, B, C],
            ),
            (
                Scale::new(Diatonic, C, 4, Some(Mixolydian), None).unwrap(),
                vec![C, D, E, F, G, A, As, C],
            ),
            (
                Scale::new(Diatonic, C, 4, Some(Phrygian), None).unwrap(),
                vec![C, Cs, Ds, F, G, Gs, As, C],
            ),
            (
                Scale::new(ScaleType::HarmonicMinor, C, 4, None, None).unwrap(),
                vec![C, D, Ds, F, G, Gs, B, C],
            ),
            (
                Scale::new(
                    ScaleType::Diatonic,
                    C,
                    4,
                    Some(Locrian),
                    Some(vec![(Sharp, 6)]),
                )
                .unwrap(),
                vec![C, Cs, Ds, F, Fs, A, As, C],
            ),
            (
                Scale::new( ScaleType::Diatonic,
                    C,
                    4,
                    Some(Ionian),
                    Some(vec![(Sharp, 5)]),
                )
                    .unwrap(),
                vec![C, D, E, F, Gs, A, B, C],
            ),
            (
                Scale::new(
                    ScaleType::Diatonic,
                    C,
                    4,
                    Some(Mixolydian),
                    Some(vec![(Flat, 2), (Flat, 6)]),
                )
                    .unwrap(),
                vec![C, Cs, E, F, G, Gs, As, C],
            ),
            (
                Scale::new(
                    ScaleType::Diatonic,
                    C,
                    4,
                    Some(Lydian),
                    Some(vec![(Sharp, 2)]),
                )
                    .unwrap(),
                vec![C, Ds, E, Fs, G, A, B, C],
            ),
            (
                Scale::new(ScaleType::MelodicMinor, C, 4, None, None).unwrap(),
                vec![C, D, Ds, F, G, A, B, C],
            ),
        ];

        for scale_tuple in scale_tuples.iter() {
            let (scale, pitches) = scale_tuple;
            assert_notes(pitches, scale.notes());
        }
    }
}
