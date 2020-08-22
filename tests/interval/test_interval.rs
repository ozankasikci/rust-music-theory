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
}
