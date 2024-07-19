use crate::note::{NoteLetter, Pitch};

/// All possible pitches with accidentals.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum PitchSymbol {
    Bs,
    C,
    Cs,
    Db,
    D,
    Ds,
    Eb,
    E,
    Es,
    F,
    Fs,
    Gb,
    G,
    Gs,
    Ab,
    A,
    As,
    Bb,
    B,
    Cb,
}

impl From<PitchSymbol> for Pitch {
    fn from(symbol: PitchSymbol) -> Self {
        use PitchSymbol::*;
        match symbol {
            Bs => Pitch::new(NoteLetter::B, 1),
            C => Pitch::new(NoteLetter::C, 0),
            Cs => Pitch::new(NoteLetter::C, 1),
            Db => Pitch::new(NoteLetter::D, -1),
            D => Pitch::new(NoteLetter::D, 0),
            Ds => Pitch::new(NoteLetter::D, 1),
            Eb => Pitch::new(NoteLetter::E, -1),
            E => Pitch::new(NoteLetter::E, 0),
            Es => Pitch::new(NoteLetter::E, -1),
            F => Pitch::new(NoteLetter::F, 0),
            Fs => Pitch::new(NoteLetter::F, 1),
            Gb => Pitch::new(NoteLetter::G, -1),
            G => Pitch::new(NoteLetter::G, 0),
            Gs => Pitch::new(NoteLetter::G, 1),
            Ab => Pitch::new(NoteLetter::A, -1),
            A => Pitch::new(NoteLetter::A, 0),
            As => Pitch::new(NoteLetter::A, 1),
            Bb => Pitch::new(NoteLetter::B, -1),
            B => Pitch::new(NoteLetter::B, 0),
            Cb => Pitch::new(NoteLetter::C, -1),
        }
    }
}
