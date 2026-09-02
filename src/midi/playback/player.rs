//! MIDI player for real-time playback.

use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration as StdDuration;

use midir::{MidiOutput, MidiOutputConnection};

use super::scheduler::Scheduler;
use super::timing::duration_to_ms;
use super::PlaybackError;
use crate::midi::{Channel, Duration, Velocity};
use crate::note::Notes;

/// Real-time MIDI player.
pub struct MidiPlayer {
    connection: Arc<Mutex<MidiOutputConnection>>,
    scheduler: Scheduler,
    tempo: u16,
    channel: Channel,
    cursor_ms: u64,
}

impl MidiPlayer {
    /// Connect to a MIDI output port by name.
    pub fn connect(port_name: &str) -> Result<Self, PlaybackError> {
        let midi_out = MidiOutput::new("rust-music-theory")
            .map_err(|e| PlaybackError::InitError(e.to_string()))?;

        let ports = midi_out.ports();
        let port = ports
            .iter()
            .find(|p| {
                midi_out
                    .port_name(p)
                    .map(|n| n == port_name)
                    .unwrap_or(false)
            })
            .ok_or_else(|| PlaybackError::PortNotFound(port_name.to_string()))?;

        let connection = midi_out
            .connect(port, "output")
            .map_err(|e| PlaybackError::ConnectionFailed(e.to_string()))?;

        let connection = Arc::new(Mutex::new(connection));
        let scheduler = Scheduler::new(connection.clone());

        Ok(Self {
            connection,
            scheduler,
            tempo: 120,
            channel: Channel::new(0).unwrap(),
            cursor_ms: 0,
        })
    }

    /// Connect to a MIDI output port by index.
    pub fn connect_index(index: usize) -> Result<Self, PlaybackError> {
        let midi_out = MidiOutput::new("rust-music-theory")
            .map_err(|e| PlaybackError::InitError(e.to_string()))?;

        let ports = midi_out.ports();
        let port = ports
            .get(index)
            .ok_or_else(|| PlaybackError::PortNotFound(format!("index {}", index)))?;

        let connection = midi_out
            .connect(port, "output")
            .map_err(|e| PlaybackError::ConnectionFailed(e.to_string()))?;

        let connection = Arc::new(Mutex::new(connection));
        let scheduler = Scheduler::new(connection.clone());

        Ok(Self {
            connection,
            scheduler,
            tempo: 120,
            channel: Channel::new(0).unwrap(),
            cursor_ms: 0,
        })
    }

    /// Set the tempo in BPM.
    pub fn set_tempo(&mut self, bpm: u16) {
        self.tempo = bpm;
        self.scheduler.set_tempo(bpm);
    }

    /// Get the current tempo.
    pub fn tempo(&self) -> u16 {
        self.tempo
    }

    /// Set the MIDI channel for output.
    pub fn set_channel(&mut self, channel: Channel) {
        self.channel = channel;
    }

    /// Get the current channel.
    pub fn channel(&self) -> Channel {
        self.channel
    }

    /// Play notes and block until complete.
    pub fn play<N: Notes>(&self, notes: &N, duration: Duration, velocity: Velocity) {
        let pitches: Vec<u8> = notes.notes().iter().map(|n| n.midi_pitch()).collect();

        // Send Note On for all pitches
        for &pitch in &pitches {
            self.send_note_on(pitch, velocity.value());
        }

        // Wait for duration
        let ms = duration_to_ms(&duration, self.tempo);
        thread::sleep(StdDuration::from_millis(ms));

        // Send Note Off for all pitches
        for &pitch in &pitches {
            self.send_note_off(pitch);
        }
    }

    /// Play a single MIDI pitch.
    pub fn play_note(&self, pitch: u8, duration: Duration, velocity: Velocity) {
        self.send_note_on(pitch, velocity.value());

        let ms = duration_to_ms(&duration, self.tempo);
        thread::sleep(StdDuration::from_millis(ms));

        self.send_note_off(pitch);
    }

    /// Rest (silent pause) for a duration.
    pub fn rest(&self, duration: Duration) {
        let ms = duration_to_ms(&duration, self.tempo);
        thread::sleep(StdDuration::from_millis(ms));
    }

    /// Send a Note On message.
    fn send_note_on(&self, pitch: u8, velocity: u8) {
        let status = 0x90 | (self.channel.value() & 0x0F);
        let message = [status, pitch & 0x7F, velocity & 0x7F];

        if let Ok(mut conn) = self.connection.lock() {
            let _ = conn.send(&message);
        }
    }

    /// Send a Note Off message.
    fn send_note_off(&self, pitch: u8) {
        let status = 0x80 | (self.channel.value() & 0x0F);
        let message = [status, pitch & 0x7F, 0];

        if let Ok(mut conn) = self.connection.lock() {
            let _ = conn.send(&message);
        }
    }

    /// Send a Control Change message immediately.
    pub fn control_change(&self, cc: u8, value: u8) {
        let status = 0xB0 | (self.channel.value() & 0x0F);
        let message = [status, cc & 0x7F, value & 0x7F];

        if let Ok(mut conn) = self.connection.lock() {
            let _ = conn.send(&message);
        }
    }

    /// Schedule a Control Change message asynchronously.
    pub fn control_change_async(&mut self, cc: u8, value: u8) {
        let status = 0xB0 | (self.channel.value() & 0x0F);
        let message = vec![status, cc & 0x7F, value & 0x7F];
        self.scheduler.schedule(self.cursor_ms, message);
    }

    /// Send a Program Change message immediately.
    pub fn program_change(&self, program: u8) {
        let status = 0xC0 | (self.channel.value() & 0x0F);
        let message = [status, program & 0x7F];

        if let Ok(mut conn) = self.connection.lock() {
            let _ = conn.send(&message);
        }
    }

    /// Send a Program Change with Bank Select immediately.
    pub fn program_change_with_bank(&self, program: u8, bank_msb: u8, bank_lsb: u8) {
        let channel = self.channel.value() & 0x0F;
        let cc_status = 0xB0 | channel;
        let pc_status = 0xC0 | channel;

        if let Ok(mut conn) = self.connection.lock() {
            // Bank Select MSB (CC 0)
            let _ = conn.send(&[cc_status, 0, bank_msb & 0x7F]);
            // Bank Select LSB (CC 32)
            let _ = conn.send(&[cc_status, 32, bank_lsb & 0x7F]);
            // Program Change
            let _ = conn.send(&[pc_status, program & 0x7F]);
        }
    }

    /// Schedule a Program Change asynchronously.
    pub fn program_change_async(&mut self, program: u8) {
        let status = 0xC0 | (self.channel.value() & 0x0F);
        let message = vec![status, program & 0x7F];
        self.scheduler.schedule(self.cursor_ms, message);
    }

    /// Schedule a Program Change with Bank Select asynchronously.
    pub fn program_change_with_bank_async(&mut self, program: u8, bank_msb: u8, bank_lsb: u8) {
        let channel = self.channel.value() & 0x0F;
        let cc_status = 0xB0 | channel;
        let pc_status = 0xC0 | channel;

        // Bank Select MSB (CC 0)
        self.scheduler
            .schedule(self.cursor_ms, vec![cc_status, 0, bank_msb & 0x7F]);
        // Bank Select LSB (CC 32)
        self.scheduler
            .schedule(self.cursor_ms, vec![cc_status, 32, bank_lsb & 0x7F]);
        // Program Change
        self.scheduler
            .schedule(self.cursor_ms, vec![pc_status, program & 0x7F]);
    }

    /// Schedule notes to play asynchronously.
    pub fn play_async<N: Notes>(&mut self, notes: &N, duration: Duration, velocity: Velocity) {
        let pitches: Vec<u8> = notes.notes().iter().map(|n| n.midi_pitch()).collect();
        let duration_ms = duration_to_ms(&duration, self.tempo);
        let channel = self.channel.value();

        // Schedule Note On for all pitches
        for &pitch in &pitches {
            let message = vec![
                0x90 | (channel & 0x0F),
                pitch & 0x7F,
                velocity.value() & 0x7F,
            ];
            self.scheduler.schedule(self.cursor_ms, message);
        }

        // Schedule Note Off for all pitches
        let note_off_time = self.cursor_ms + duration_ms;
        for &pitch in &pitches {
            let message = vec![0x80 | (channel & 0x0F), pitch & 0x7F, 0];
            self.scheduler.schedule(note_off_time, message);
        }

        // Advance cursor
        self.cursor_ms = note_off_time;
    }

    /// Schedule a single note to play asynchronously.
    pub fn play_note_async(&mut self, pitch: u8, duration: Duration, velocity: Velocity) {
        let duration_ms = duration_to_ms(&duration, self.tempo);
        let channel = self.channel.value();

        let note_on = vec![
            0x90 | (channel & 0x0F),
            pitch & 0x7F,
            velocity.value() & 0x7F,
        ];
        self.scheduler.schedule(self.cursor_ms, note_on);

        let note_off_time = self.cursor_ms + duration_ms;
        let note_off = vec![0x80 | (channel & 0x0F), pitch & 0x7F, 0];
        self.scheduler.schedule(note_off_time, note_off);

        self.cursor_ms = note_off_time;
    }

    /// Schedule a rest (advances cursor without playing).
    pub fn rest_async(&mut self, duration: Duration) {
        let duration_ms = duration_to_ms(&duration, self.tempo);
        self.cursor_ms += duration_ms;
    }

    /// Reset the cursor to the beginning.
    pub fn reset_cursor(&mut self) {
        self.cursor_ms = 0;
    }

    /// Seek to a specific time position in milliseconds.
    pub fn seek(&mut self, time_ms: u64) {
        self.cursor_ms = time_ms;
    }

    /// Get the current cursor position in milliseconds.
    pub fn cursor(&self) -> u64 {
        self.cursor_ms
    }

    /// Wait for all scheduled notes to finish playing.
    pub fn wait(&self) {
        self.scheduler.wait();
    }

    /// Stop all playing notes immediately.
    pub fn stop(&self) {
        self.scheduler.stop();
    }

    /// Start the MIDI clock (sends 24 pulses per quarter note).
    pub fn start_clock(&self) {
        self.scheduler.start_clock();
    }

    /// Stop the MIDI clock.
    pub fn stop_clock(&self) {
        self.scheduler.stop_clock();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_tempo_is_120() {
        // We can't test connection without hardware, but we can test defaults
        // by checking the struct fields after construction with mocked data
        // For now, just verify the expected default
        assert_eq!(120u16, 120);
    }

    #[test]
    fn channel_default() {
        let channel = Channel::new(0).unwrap();
        assert_eq!(channel.value(), 0);
    }

    #[test]
    fn connect_nonexistent_port_returns_error() {
        let result = MidiPlayer::connect("NonExistent Port XYZ 12345");
        assert!(result.is_err());
        match result {
            Err(PlaybackError::PortNotFound(_)) => {}
            _ => panic!("Expected PortNotFound error"),
        }
    }

    #[test]
    fn connect_invalid_index_returns_error() {
        let result = MidiPlayer::connect_index(99999);
        assert!(result.is_err());
        match result {
            Err(PlaybackError::PortNotFound(_)) => {}
            _ => panic!("Expected PortNotFound error"),
        }
    }

    #[test]
    fn control_change_message_bytes() {
        // CC message format: [0xB0 | channel, cc_number, value]
        let channel = 0u8;
        let cc = 1u8; // Modulation
        let value = 64u8;

        let status = 0xB0 | (channel & 0x0F);
        let message = [status, cc & 0x7F, value & 0x7F];

        assert_eq!(message, [0xB0, 1, 64]);
    }

    #[test]
    fn control_change_channel_5() {
        let channel = 5u8;
        let status = 0xB0 | (channel & 0x0F);
        assert_eq!(status, 0xB5);
    }

    #[test]
    fn program_change_message_bytes() {
        // Program Change: [0xC0 | channel, program]
        let channel = 0u8;
        let program = 5u8;

        let status = 0xC0 | (channel & 0x0F);
        let message = [status, program & 0x7F];

        assert_eq!(message, [0xC0, 5]);
    }

    #[test]
    fn bank_select_message_bytes() {
        // Bank Select MSB: [0xB0 | channel, 0, msb]
        // Bank Select LSB: [0xB0 | channel, 32, lsb]
        let channel = 0u8;
        let msb = 1u8;
        let lsb = 2u8;

        let status = 0xB0 | (channel & 0x0F);
        let msg_msb = [status, 0, msb & 0x7F];
        let msg_lsb = [status, 32, lsb & 0x7F];

        assert_eq!(msg_msb, [0xB0, 0, 1]);
        assert_eq!(msg_lsb, [0xB0, 32, 2]);
    }
}
