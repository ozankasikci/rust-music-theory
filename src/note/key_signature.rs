use crate::note::{NoteLetter, Pitch, PitchSymbol};
use crate::scale::Mode;
use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
    static ref KEY_SIGNATURE_SPELLINGS: HashMap<(NoteLetter, i8), Vec<PitchSymbol>> = {
        use NoteLetter::*;
        use PitchSymbol::*;
        let mut m = HashMap::new();
        m.insert((C, 0), vec![C, D, E, F, G, A, B]);
        m.insert((G, 0), vec![G, A, B, C, D, E, Fs]);
        m.insert((D, 0), vec![D, E, Fs, G, A, B, Cs]);
        m.insert((A, 0), vec![A, B, Cs, D, E, Fs, Gs]);
        m.insert((E, 0), vec![E, Fs, Gs, A, B, Cs, Ds]);
        m.insert((B, 0), vec![B, Cs, Ds, E, Fs, Gs, As]);
        m.insert((F, 0), vec![F, G, A, Bb, C, D, E]);
        m.insert((Bb, 0), vec![Bb, C, D, Eb, F, G, A]);
        m.insert((Eb, 0), vec![Eb, F, G, Ab, Bb, C, D]);
        m.insert((Ab, 0), vec![Ab, Bb, C, Db, Eb, F, G]);
        m.insert((Db, 0), vec![Db, Eb, F, Gb, Ab, Bb, C]);
        m.insert((Gb, 0), vec![Gb, Ab, Bb, Cb, Db, Eb, F]);
        m
    };
}

/// A key signature.
#[derive(Debug, Clone)]
pub struct KeySignature {
    /// The tonic of the key signature.
    pub tonic: Pitch,
    /// The mode of the key signature.
    pub mode: Option<Mode>,
}

impl KeySignature {
    /// Create a new key signature.
    pub fn new(tonic: Pitch) -> Self {
        KeySignature {
            tonic,
            mode: None,
        }
    }

    /// Create a new key signature with a mode.
    pub fn new_with_mode(tonic: Pitch, mode: Option<Mode>) -> Self {
        KeySignature { tonic, mode }
    }

    pub fn get_preferred_spelling(&self, pitch: Pitch) -> PitchSymbol {
        use PitchSymbol::*;
        
        // Get the key signature accidentals
        if let Some(key_accidentals) = KEY_SIGNATURE_SPELLINGS.get(&(self.tonic.letter, self.tonic.accidental)) {
            // Check if this pitch has a preferred spelling in this key
            for &accidental in key_accidentals {
                if Pitch::from(accidental).into_u8() == pitch.into_u8() {
                    return accidental;
                }
            }
        }

        // For C major and its modes, we prefer sharp spellings
        if self.tonic.letter == NoteLetter::C && self.tonic.accidental == 0 {
            match pitch.into_u8() {
                0 => C,   // C
                1 => Cs,  // C♯
                2 => D,   // D
                3 => Ds,  // D♯
                4 => E,   // E
                5 => F,   // F
                6 => Fs,  // F♯
                7 => G,   // G
                8 => Gs,  // G♯
                9 => A,   // A
                10 => As, // A♯
                11 => B,  // B
                _ => unreachable!(),
            }
        } else {
            // For other keys, follow traditional rules
            let is_sharp_key = match self.tonic.letter {
                NoteLetter::G | NoteLetter::D | NoteLetter::A | NoteLetter::E | NoteLetter::B => true,
                _ => false,
            };

            // Use sharps for sharp keys and leading tones, flats for flat keys
            match pitch.into_u8() {
                0 => C,   // C
                1 => if is_sharp_key { Cs } else { Db },  // C♯/D♭
                2 => D,   // D
                3 => if is_sharp_key { Ds } else { Eb },  // D♯/E♭
                4 => E,   // E
                5 => F,   // F
                6 => if is_sharp_key { Fs } else { Gb },  // F♯/G♭
                7 => G,   // G
                8 => if is_sharp_key { Gs } else { Ab },  // G♯/A♭
                9 => A,   // A
                10 => if is_sharp_key { As } else { Bb }, // A♯/B♭
                11 => B,  // B
                _ => unreachable!(),
            }
        }
    }
}
