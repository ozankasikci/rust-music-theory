//! Timing utilities for MIDI playback.

use crate::midi::Duration;

/// Default PPQ for timing calculations.
pub const DEFAULT_PPQ: u16 = 480;

/// Convert a Duration to milliseconds at the given BPM.
pub fn duration_to_ms(duration: &Duration, bpm: u16) -> u64 {
    let beat_ms = 60_000u64 / bpm as u64; // ms per quarter note
    let ticks = duration.to_ticks(DEFAULT_PPQ);
    (ticks as u64 * beat_ms) / DEFAULT_PPQ as u64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn quarter_note_at_120_bpm() {
        // 120 BPM = 500ms per quarter note
        assert_eq!(duration_to_ms(&Duration::Quarter, 120), 500);
    }

    #[test]
    fn half_note_at_120_bpm() {
        // 120 BPM = 1000ms per half note
        assert_eq!(duration_to_ms(&Duration::Half, 120), 1000);
    }

    #[test]
    fn eighth_note_at_120_bpm() {
        // 120 BPM = 250ms per eighth note
        assert_eq!(duration_to_ms(&Duration::Eighth, 120), 250);
    }

    #[test]
    fn whole_note_at_120_bpm() {
        // 120 BPM = 2000ms per whole note
        assert_eq!(duration_to_ms(&Duration::Whole, 120), 2000);
    }

    #[test]
    fn quarter_note_at_60_bpm() {
        // 60 BPM = 1000ms per quarter note
        assert_eq!(duration_to_ms(&Duration::Quarter, 60), 1000);
    }

    #[test]
    fn quarter_note_at_240_bpm() {
        // 240 BPM = 250ms per quarter note
        assert_eq!(duration_to_ms(&Duration::Quarter, 240), 250);
    }

    #[test]
    fn dotted_quarter_at_120_bpm() {
        // Dotted quarter = 1.5 * 500 = 750ms
        assert_eq!(
            duration_to_ms(&Duration::dotted(Duration::Quarter), 120),
            750
        );
    }

    #[test]
    fn triplet_quarter_at_120_bpm() {
        // Triplet quarter = 2/3 * 500 = 333ms (rounded)
        assert_eq!(
            duration_to_ms(&Duration::triplet(Duration::Quarter), 120),
            333
        );
    }
}
