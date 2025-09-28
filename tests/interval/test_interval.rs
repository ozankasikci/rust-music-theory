extern crate rust_music_theory as theory;
use theory::interval::Interval;
use theory::note::{Note, Pitch, PitchSymbol::*};

#[cfg(test)]
mod test_interval {
    use super::*;

    #[test]
    fn test_second_note_from() {
        let notes = vec![(C, 3), (D, 3), (E, 3), (Fs, 3), (Gs, 3), (As, 3), (C, 4)]
            .into_iter()
            .map(|note| Note {
                pitch: Pitch::from(note.0),
                octave: note.1,
            })
            .collect::<Vec<Note>>();

        let major_second = Interval::from_semitone(2).unwrap();
        for i in 0..(notes.len() - 1) {
            let next_note = major_second.second_note_from(notes[i].clone());
            assert_eq!(next_note.pitch, notes[i + 1].pitch);
            assert_eq!(next_note.octave, notes[i + 1].octave);
        }
    }

    #[test]
    fn test_second_note_down_from() {
        let notes = vec![(C, 4), (As, 3), (Gs, 3), (Fs, 3), (E, 3), (D, 3), (C, 3)]
            .into_iter()
            .map(|note| Note {
                pitch: Pitch::from(note.0),
                octave: note.1,
            })
            .collect::<Vec<Note>>();

        let major_second = Interval::from_semitone(2).unwrap();
        for i in 0..(notes.len() - 1) {
            let next_note = major_second.second_note_down_from(notes[i].clone());
            assert_eq!(next_note.pitch, notes[i + 1].pitch);
            assert_eq!(next_note.octave, notes[i + 1].octave);
        }
    }

    #[test]
    fn test_octave_jump() {
        let octave_interval = Interval::from_semitone(12).unwrap();
        for octave in 0..=8 {
            let note = Note {
                pitch: Pitch::from(C),
                octave,
            };
            let next_note = octave_interval.second_note_from(note.clone());
            assert_eq!(next_note.octave, note.octave + 1);
        }
    }

    #[test]
    fn test_octave_jump_down() {
        let octave_interval = Interval::from_semitone(12).unwrap();
        for octave in 8..=0 {
            let note = Note {
                pitch: Pitch::from(C),
                octave,
            };
            let next_note = octave_interval.second_note_down_from(note.clone());
            assert_eq!(next_note.octave, note.octave - 1);
        }
    }

    #[test]
    fn test_invert_unison() {
        let unison = Interval::from_semitone(0).unwrap();
        let inverted = Interval::invert(&unison);
        assert!(inverted.is_ok());
        assert_eq!(inverted.unwrap().semitone_count, unison.semitone_count);
    }

    #[test]
    fn test_invert() {
        let list = vec![12, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12];

        for i in 0..list.len() {
            let interval = Interval::from_semitone(list[i]).unwrap();
            let inverted = Interval::invert(&interval);
            assert!(inverted.is_ok());
            assert_eq!(inverted.unwrap().semitone_count, list[list.len() - i - 1]);
        }
    }

    #[test]
    fn test_display() {
        let intervals = vec![
            (Interval::from_semitone(0).unwrap(), "1"),
            (Interval::from_semitone(1).unwrap(), "m2"),
            (Interval::from_semitone(2).unwrap(), "M2"),
            (Interval::from_semitone(3).unwrap(), "m3"),
            (Interval::from_semitone(4).unwrap(), "M3"),
            (Interval::from_semitone(5).unwrap(), "P4"),
            (Interval::from_semitone(6).unwrap(), "T"),
            (Interval::from_semitone(7).unwrap(), "P5"),
            (Interval::from_semitone(8).unwrap(), "m6"),
            (Interval::from_semitone(9).unwrap(), "M6"),
            (Interval::from_semitone(10).unwrap(), "m7"),
            (Interval::from_semitone(11).unwrap(), "M7"),
            (Interval::from_semitone(12).unwrap(), "1"),
        ];

        for interval in intervals {
            assert_eq!(interval.0.to_string(), interval.1);
        }
    }

    #[test]
    fn test_from_semitone_error() {
        // Test that values > 12 return an error
        assert!(Interval::from_semitone(13).is_err());
        assert!(Interval::from_semitone(14).is_err());
        assert!(Interval::from_semitone(100).is_err());
        assert!(Interval::from_semitone(255).is_err());
    }

    #[test]
    fn test_from_semitones() {
        // Valid semitones
        let semitones = vec![0, 2, 4, 5, 7, 9, 11];
        let result = Interval::from_semitones(&semitones);
        assert!(result.is_ok());
        let intervals = result.unwrap();
        assert_eq!(intervals.len(), semitones.len());
        for (i, &semitone) in semitones.iter().enumerate() {
            assert_eq!(intervals[i].semitone_count, semitone);
        }

        // Empty input should return error
        let empty: Vec<u8> = vec![];
        assert!(Interval::from_semitones(&empty).is_err());

        // Invalid semitone (> 12) should return error
        let invalid_semitones = vec![0, 2, 4, 13];
        assert!(Interval::from_semitones(&invalid_semitones).is_err());
    }

    #[test]
    fn test_to_notes() {
        let root = Note {
            pitch: Pitch::from(C),
            octave: 4,
        };

        // Create a major triad (C-E-G)
        let major_third = Interval::from_semitone(4).unwrap();
        let minor_third = Interval::from_semitone(3).unwrap();
        let intervals = vec![major_third, minor_third];

        let notes = Interval::to_notes(root.clone(), intervals.into_iter());
        assert_eq!(notes.len(), 3);

        // Check the root note
        assert_eq!(notes[0].pitch, Pitch::from(C));
        assert_eq!(notes[0].octave, 4);

        // Check E (major third from C)
        assert_eq!(notes[1].pitch, Pitch::from(E));
        assert_eq!(notes[1].octave, 4);

        // Check G (minor third from E, perfect fifth from C)
        assert_eq!(notes[2].pitch, Pitch::from(G));
        assert_eq!(notes[2].octave, 4);
    }

    #[test]
    fn test_to_notes_reverse() {
        let root = Note {
            pitch: Pitch::from(C),
            octave: 4,
        };

        // Create a descending pattern - intervals will be applied in reverse order
        // So if we provide [perfect_fifth, perfect_fourth],
        // the function will apply perfect_fourth first, then perfect_fifth
        let perfect_fifth = Interval::from_semitone(7).unwrap();
        let perfect_fourth = Interval::from_semitone(5).unwrap();
        let intervals = vec![perfect_fifth, perfect_fourth];

        let notes = Interval::to_notes_reverse(root.clone(), intervals.into_iter());
        assert_eq!(notes.len(), 3);

        // Check the root note (C4)
        assert_eq!(notes[0].pitch, Pitch::from(C));
        assert_eq!(notes[0].octave, 4);

        // Because intervals are reversed, the second note is a perfect fourth down from C4 (G3)
        assert_eq!(notes[1].pitch, Pitch::from(G));
        assert_eq!(notes[1].octave, 3);

        // The third note is a perfect fifth down from G3, which is C3
        assert_eq!(notes[2].pitch, Pitch::from(C));
        assert_eq!(notes[2].octave, 3);
    }

    #[test]
    fn test_interval_quality_and_number() {
        use theory::interval::{Quality, Number};

        // Test perfect intervals
        let unison = Interval::from_semitone(0).unwrap();
        assert_eq!(unison.quality, Quality::Perfect);
        assert_eq!(unison.number, Number::Unison);

        let perfect_fourth = Interval::from_semitone(5).unwrap();
        assert_eq!(perfect_fourth.quality, Quality::Perfect);
        assert_eq!(perfect_fourth.number, Number::Fourth);

        let perfect_fifth = Interval::from_semitone(7).unwrap();
        assert_eq!(perfect_fifth.quality, Quality::Perfect);
        assert_eq!(perfect_fifth.number, Number::Fifth);

        let octave = Interval::from_semitone(12).unwrap();
        assert_eq!(octave.quality, Quality::Perfect);
        assert_eq!(octave.number, Number::Octave);

        // Test major intervals
        let major_second = Interval::from_semitone(2).unwrap();
        assert_eq!(major_second.quality, Quality::Major);
        assert_eq!(major_second.number, Number::Second);

        let major_third = Interval::from_semitone(4).unwrap();
        assert_eq!(major_third.quality, Quality::Major);
        assert_eq!(major_third.number, Number::Third);

        // Test minor intervals
        let minor_second = Interval::from_semitone(1).unwrap();
        assert_eq!(minor_second.quality, Quality::Minor);
        assert_eq!(minor_second.number, Number::Second);

        let minor_third = Interval::from_semitone(3).unwrap();
        assert_eq!(minor_third.quality, Quality::Minor);
        assert_eq!(minor_third.number, Number::Third);

        // Test diminished interval
        let tritone = Interval::from_semitone(6).unwrap();
        assert_eq!(tritone.quality, Quality::Diminished);
        assert_eq!(tritone.number, Number::Fifth);
    }

    #[test]
    fn test_interval_step() {
        use theory::interval::Step;

        // Test intervals with steps
        let minor_second = Interval::from_semitone(1).unwrap();
        assert_eq!(minor_second.step, Some(Step::Half));

        let major_second = Interval::from_semitone(2).unwrap();
        assert_eq!(major_second.step, Some(Step::Whole));

        let tritone = Interval::from_semitone(6).unwrap();
        assert_eq!(tritone.step, Some(Step::Tritone));

        // Test intervals without steps
        let major_third = Interval::from_semitone(4).unwrap();
        assert_eq!(major_third.step, None);

        let perfect_fifth = Interval::from_semitone(7).unwrap();
        assert_eq!(perfect_fifth.step, None);
    }
}
