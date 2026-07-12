//! Chords.

mod chord;
mod errors;
mod number;
mod parser;
mod quality;
mod spec;

pub use chord::{Chord, ChordBuilder};
pub use errors::ChordError;
pub use number::Number;
pub use quality::Quality;
pub use spec::{
    ChordExtension, ChordFormula, ChordModifier, ChordSpec, ChordTone, SeventhQuality, Suspension,
    TriadQuality,
};

/// Human-readable syntax registry used by documentation and command-line discovery.
pub const SUPPORTED_CHORD_SYNTAX: &[&str] = &[
    "Major Triad: C",
    "Minor Triad: Cm",
    "Diminished Triad: Cdim",
    "Augmented Triad: Caug",
    "Power Chord: C5",
    "Sixths: C6, Cm6, C6/9, Cm6/9",
    "Sevenths: C7, Cmaj7, Cm7, CmMaj7, Cm7b5, Cdim7, Caug7, CaugMaj7",
    "Ninths: C9, Cmaj9, Cm9, CmMaj9",
    "Elevenths: C11, Cmaj11, Cm11, CmMaj11",
    "Thirteenths: C13, Cmaj13, Cm13, CmMaj13",
    "Suspensions: Csus2, Csus4, C7sus4, Cm7sus4",
    "Added tones: Cadd2, Cadd4, Cadd6, Cadd9, Cadd11, Cadd13",
    "Alterations: C7b5, C7#5, C7b9, C7#9, Cmaj9#11, C13b13",
    "Omissions: Cno3, C7no5, C13omit11",
    "Altered dominant: C7alt",
    "Slash basses and inversions: C/E, C/F#, C/1",
    "Aliases: CΔ, CΔ9, CmΔ, C^7, CM7, Cma7, C-7, Cmi7, Cm/M7, C°7, Cø7, Ch7, C+7, C69, C6add9",
    "Grouped modifiers: C7(b9,#11)",
];
