use crate::note::PitchClass;

#[derive(Debug)]
pub struct Note {
    pub pitch_class: PitchClass,
    pub octave: i8,
}

impl Note {
    pub fn new(pitch_class: PitchClass, octave: i8) -> Self {
        Note {
            pitch_class,
            octave,
        }
    }
}
