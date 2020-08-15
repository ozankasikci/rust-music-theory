extern crate rust_music_theory as theory;
use theory::interval::Interval;
use theory::note::{Note, PitchClass::*};

#[cfg(test)]
mod test_interval {
    use super::*;

    #[test]
    fn test_second_note_from() {
        let notes = vec![(C, 3), (D, 3), (E, 3), (Fs, 3), (Gs, 3), (As, 3), (C, 4)]
            .into_iter()
            .map(|note| Note {
                pitch_class: note.0,
                octave: note.1,
            })
            .collect::<Vec<Note>>();

        let major_second = Interval::from_semitone(2).unwrap();
        for i in 0..(notes.len() - 1) {
            let next_note = major_second.second_note_from(notes[i].clone());
            assert_eq!(next_note.pitch_class, notes[i + 1].pitch_class);
            assert_eq!(next_note.octave, notes[i + 1].octave);
        }
    }

    #[test]
    fn test_second_note_down_from() {
        let notes = vec![(C, 4), (As, 3), (Gs, 3), (Fs, 3), (E, 3), (D, 3), (C, 3)]
            .into_iter()
            .map(|note| Note {
                pitch_class: note.0,
                octave: note.1,
            })
            .collect::<Vec<Note>>();

        let major_second = Interval::from_semitone(2).unwrap();
        for i in 0..(notes.len() - 1) {
            let next_note = major_second.second_note_down_from(notes[i].clone());
            assert_eq!(next_note.pitch_class, notes[i + 1].pitch_class);
            assert_eq!(next_note.octave, notes[i + 1].octave);
        }
    }
}
