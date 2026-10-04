//! Internal MIDI event representation.

use crate::midi::{Channel, Velocity};

/// Internal representation of a MIDI event.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MidiEvent {
    /// Note on event
    NoteOn {
        tick: u32,
        channel: Channel,
        pitch: u8,
        velocity: Velocity,
    },
    /// Note off event
    NoteOff {
        tick: u32,
        channel: Channel,
        pitch: u8,
    },
    /// Tempo change (microseconds per beat)
    Tempo {
        tick: u32,
        microseconds_per_beat: u32,
    },
    /// Time signature change
    TimeSignature {
        tick: u32,
        numerator: u8,
        denominator: u8,
    },
}

impl MidiEvent {
    /// Get the tick position of this event.
    pub fn tick(&self) -> u32 {
        match self {
            MidiEvent::NoteOn { tick, .. } => *tick,
            MidiEvent::NoteOff { tick, .. } => *tick,
            MidiEvent::Tempo { tick, .. } => *tick,
            MidiEvent::TimeSignature { tick, .. } => *tick,
        }
    }

    /// Convert BPM to microseconds per beat for tempo events.
    pub fn bpm_to_microseconds(bpm: u16) -> u32 {
        60_000_000 / bpm as u32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bpm_conversion() {
        // 120 BPM = 500,000 microseconds per beat
        assert_eq!(MidiEvent::bpm_to_microseconds(120), 500_000);
        // 60 BPM = 1,000,000 microseconds per beat
        assert_eq!(MidiEvent::bpm_to_microseconds(60), 1_000_000);
    }
}
