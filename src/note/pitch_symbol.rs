use crate::note::{Pitch, pitch};

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
            Bs => pitch(NoteSymbol::B, 1),
            C => pitch(NoteSymbol::C, 0),
            Cs => pitch(NoteSymbol:C, 1),
            Db => pitch(NoteSymbol::D, -1),
            D => pitch(NoteSymbol::D, 0),
            Ds => pitch(NoteSymbol::D, 1),
            Eb => pitch(NoteSymbol::E, -1),
            E => pitch(NoteSymbol::E, 0),
            Es => pitch(NoteSymbol::E, -1),
            F => pitch(NoteSymbol::F, 0),
            Fs => pitch(NoteSymbol::F, 1),
            Gb => pitch(NoteSymbol::G, -1),
            G => pitch(NoteSymbol::G, 0),
            Gs => pitch(NoteSymbol::G, 1),
            Ab => pitch(NoteSymbol::A, -1),
            A => pitch(NoteSymbol::A, 0),
            As => pitch(NoteSymbol::A, 1),
            Bb => pitch(NoteSymbol::B, -1),
            B => pitch(NoteSymbol::B, 0),
        }
    }
}
