//! Musical duration values for MIDI export.

/// Musical duration values.
///
/// Standard PPQ (Pulses Per Quarter Note) is 480.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Duration {
    /// Whole note (4 beats)
    Whole,
    /// Half note (2 beats)
    Half,
    /// Quarter note (1 beat)
    Quarter,
    /// Eighth note (1/2 beat)
    Eighth,
    /// Sixteenth note (1/4 beat)
    Sixteenth,
    /// Thirty-second note (1/8 beat)
    ThirtySecond,
    /// Dotted duration (1.5x length)
    Dotted(Box<Duration>),
    /// Triplet duration (2/3 length)
    Triplet(Box<Duration>),
    /// Raw ticks for precise control
    Ticks(u32),
}

impl Duration {
    /// Convert duration to ticks at the given PPQ (Pulses Per Quarter Note).
    /// Standard PPQ is 480.
    pub fn to_ticks(&self, ppq: u16) -> u32 {
        let ppq = ppq as u32;
        match self {
            Duration::Whole => ppq * 4,
            Duration::Half => ppq * 2,
            Duration::Quarter => ppq,
            Duration::Eighth => ppq / 2,
            Duration::Sixteenth => ppq / 4,
            Duration::ThirtySecond => ppq / 8,
            Duration::Dotted(inner) => inner.to_ticks(ppq as u16) * 3 / 2,
            Duration::Triplet(inner) => inner.to_ticks(ppq as u16) * 2 / 3,
            Duration::Ticks(t) => *t,
        }
    }

    /// Create a dotted duration (1.5x length).
    pub fn dotted(base: Duration) -> Self {
        Duration::Dotted(Box::new(base))
    }

    /// Create a triplet duration (2/3 length).
    pub fn triplet(base: Duration) -> Self {
        Duration::Triplet(Box::new(base))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const PPQ: u16 = 480;

    #[test]
    fn basic_durations_to_ticks() {
        assert_eq!(Duration::Whole.to_ticks(PPQ), 1920);
        assert_eq!(Duration::Half.to_ticks(PPQ), 960);
        assert_eq!(Duration::Quarter.to_ticks(PPQ), 480);
        assert_eq!(Duration::Eighth.to_ticks(PPQ), 240);
        assert_eq!(Duration::Sixteenth.to_ticks(PPQ), 120);
        assert_eq!(Duration::ThirtySecond.to_ticks(PPQ), 60);
    }

    #[test]
    fn ticks_passthrough() {
        assert_eq!(Duration::Ticks(123).to_ticks(PPQ), 123);
        assert_eq!(Duration::Ticks(999).to_ticks(PPQ), 999);
    }

    #[test]
    fn dotted_durations() {
        // Dotted quarter = 1.5 * 480 = 720
        assert_eq!(
            Duration::Dotted(Box::new(Duration::Quarter)).to_ticks(PPQ),
            720
        );
        // Dotted half = 1.5 * 960 = 1440
        assert_eq!(
            Duration::Dotted(Box::new(Duration::Half)).to_ticks(PPQ),
            1440
        );
        // Dotted eighth = 1.5 * 240 = 360
        assert_eq!(
            Duration::Dotted(Box::new(Duration::Eighth)).to_ticks(PPQ),
            360
        );
    }

    #[test]
    fn triplet_durations() {
        // Triplet quarter = 2/3 * 480 = 320
        assert_eq!(
            Duration::Triplet(Box::new(Duration::Quarter)).to_ticks(PPQ),
            320
        );
        // Triplet eighth = 2/3 * 240 = 160
        assert_eq!(
            Duration::Triplet(Box::new(Duration::Eighth)).to_ticks(PPQ),
            160
        );
    }

    #[test]
    fn nested_modifiers() {
        // Dotted triplet quarter = 1.5 * (2/3 * 480) = 1.5 * 320 = 480
        let dotted_triplet =
            Duration::Dotted(Box::new(Duration::Triplet(Box::new(Duration::Quarter))));
        assert_eq!(dotted_triplet.to_ticks(PPQ), 480);
    }

    #[test]
    fn helper_constructors() {
        assert_eq!(
            Duration::dotted(Duration::Quarter).to_ticks(PPQ),
            Duration::Dotted(Box::new(Duration::Quarter)).to_ticks(PPQ)
        );
        assert_eq!(
            Duration::triplet(Duration::Quarter).to_ticks(PPQ),
            Duration::Triplet(Box::new(Duration::Quarter)).to_ticks(PPQ)
        );
    }
}
