//! Scales.

mod errors;
mod mode;
mod scale;
mod scale_type;

pub use mode::Mode;
pub use errors::ScaleError;
pub use scale::{Direction, Scale};
pub use scale_type::ScaleType;
