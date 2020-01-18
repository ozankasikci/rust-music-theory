use crate::note::PitchClass;

#[derive(Debug)]
pub struct Note {
    pub pitch_class: PitchClass,
    pub octave: u8,
}

impl Note {
    pub fn new(pitch_class: PitchClass, octave: u8) -> Self {
        Note {
            pitch_class,
            octave,
        }
    }
}
