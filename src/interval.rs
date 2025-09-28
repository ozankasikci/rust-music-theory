//! Intervals between notes.

mod errors;
mod interval;

pub use errors::IntervalError;
pub use interval::{Interval, Number, Quality, Step};
