use crate::note::PitchClass;

#[derive(Debug)]
pub struct Note {
    pub pitch_class: PitchClass,
    pub octave: i8,
}

