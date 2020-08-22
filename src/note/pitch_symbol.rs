use crate::note::{Pitch, pitch, NoteLetter};

/// All possible pitches with accidentals.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum PitchSymbol {
    Bs, C,
    Cs, Db,
    D,
    Ds, Eb,
    E,
    Es, F,
    Fs, Gb,
    G,
    Gs, Ab,
    A,
    As, Bb,
    B,
}

impl From<PitchSymbol> for Pitch {
    fn from(symbol: PitchSymbol) -> Self {
        use PitchSymbol::*;
        match symbol {
            Bs => pitch(NoteLetter::B, 1),
            C => pitch(NoteLetter::C, 0),
            Cs => pitch(NoteLetter::C, 1),
            Db => pitch(NoteLetter::D, -1),
            D => pitch(NoteLetter::D, 0),
            Ds => pitch(NoteLetter::D, 1),
            Eb => pitch(NoteLetter::E, -1),
            E => pitch(NoteLetter::E, 0),
            Es => pitch(NoteLetter::E, -1),
            F => pitch(NoteLetter::F, 0),
            Fs => pitch(NoteLetter::F, 1),
            Gb => pitch(NoteLetter::G, -1),
            G => pitch(NoteLetter::G, 0),
            Gs => pitch(NoteLetter::G, 1),
            Ab => pitch(NoteLetter::A, -1),
            A => pitch(NoteLetter::A, 0),
            As => pitch(NoteLetter::A, 1),
            Bb => pitch(NoteLetter::B, -1),
            B => pitch(NoteLetter::B, 0),
        }
    }
}
