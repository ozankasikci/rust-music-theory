use strum_macros::{Display};

#[derive(Display, Debug)]
pub enum ScaleType {
    Chromatic,
    Octatonic,
    Heptatonic,
    Hexatonic,
    Pentatonic,
    Tetratonic,
    Monotonic,
}