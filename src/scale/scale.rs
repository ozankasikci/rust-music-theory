use crate::note::{PitchClass, Note};
use crate::scale::{ScaleType};

#[derive(Debug)]
pub struct Scale {
    pub tonic: PitchClass,
    pub octave: i8,
    pub scale_type: ScaleType,
    pub steps: Vec<i8>,
}

impl Scale {
    pub fn get_notes() -> &[Note] {

    }

    pub fn get_steps(&self) -> &[i8] {
        match self.scale_type {
            ScaleType::Chromatic => &[1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
            ScaleType::Octatonic => &[],
            ScaleType::Heptatonic => {}
            ScaleType::Hexatonic => {}
            ScaleType::Pentatonic => {}
            ScaleType::Tetratonic => {}
            ScaleType::Monotonic => {}
        }
    }
}