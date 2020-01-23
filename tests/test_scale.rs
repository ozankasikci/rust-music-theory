extern crate fantasy_in_rust as fir;
use fir::note::{PitchClass::*, *};
use fir::scale::{Mode::*, ScaleType::*, *};

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
                Scale::new(Diatonic, C, 4, Some(Ionian)).unwrap(),
                vec![C, D, E, F, G, A, B, C],
            ),
            (
                Scale::new(Diatonic, C, 4, Some(Locrian)).unwrap(),
                vec![C, Cs, Ds, F, Fs, Gs, As, C],
            ),
            (
                Scale::new(Diatonic, B, 4, Some(Locrian)).unwrap(),
                vec![B, C, D, E, F, G, A, B],
            ),
            (
                Scale::new(Diatonic, C, 4, Some(Dorian)).unwrap(),
                vec![C, D, Ds, F, G, A, As, C],
            ),
            (
                Scale::new(Diatonic, A, 4, Some(Aeolian)).unwrap(),
                vec![A, B, C, D, E, F, G, A],
            ),
            (
                Scale::new(Diatonic, C, 4, Some(Lydian)).unwrap(),
                vec![C, D, E, Fs, G, A, B, C],
            ),
            (
                Scale::new(Diatonic, C, 4, Some(Mixolydian)).unwrap(),
                vec![C, D, E, F, G, A, As, C],
            ),
            (
                Scale::new(Diatonic, C, 4, Some(Phrygian)).unwrap(),
                vec![C, Cs, Ds, F, G, Gs, As, C],
            ),
            (
                Scale::new(HarmonicMinor, C, 4, None).unwrap(),
                vec![C, D, Ds, F, G, Gs, B, C],
            ),
            (
                Scale::new(MelodicMinor, C, 4, None).unwrap(),
                vec![C, D, Ds, F, G, A, B, C],
            ),
        ];

        for scale_tuple in scale_tuples.iter() {
            let (scale, pitches) = scale_tuple;
            assert_notes(pitches, scale.notes());
        }
    }
}
