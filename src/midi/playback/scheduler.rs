//! Background scheduler for async MIDI playback.

use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::sync::mpsc::{self, Receiver, Sender};
use std::sync::{Arc, Condvar, Mutex};
use std::thread::{self, JoinHandle};
use std::time::{Duration as StdDuration, Instant};

use midir::MidiOutputConnection;

/// A scheduled MIDI event.
#[derive(Debug, Clone)]
struct ScheduledEvent {
    time_ms: u64,
    message: Vec<u8>,
}

impl PartialEq for ScheduledEvent {
    fn eq(&self, other: &Self) -> bool {
        self.time_ms == other.time_ms
    }
}

impl Eq for ScheduledEvent {}

impl PartialOrd for ScheduledEvent {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for ScheduledEvent {
    fn cmp(&self, other: &Self) -> Ordering {
        // Reverse ordering for min-heap behavior
        other.time_ms.cmp(&self.time_ms)
    }
}

/// Messages sent to the scheduler thread.
#[allow(dead_code)]
pub enum SchedulerCommand {
    /// Schedule a MIDI message at a future time (ms from start)
    Schedule { time_ms: u64, message: Vec<u8> },
    /// Update tempo (used for clock timing)
    SetTempo(u16),
    /// Stop all notes immediately
    Stop,
    /// Shutdown the scheduler
    Shutdown,
    /// Start MIDI clock
    StartClock,
    /// Stop MIDI clock
    StopClock,
}

/// The scheduler manages a background thread for timed MIDI events.
pub struct Scheduler {
    sender: Sender<SchedulerCommand>,
    thread: Option<JoinHandle<()>>,
    #[allow(dead_code)]
    current_time_ms: Arc<Mutex<u64>>,
    idle_signal: Arc<(Mutex<bool>, Condvar)>,
}

impl Scheduler {
    /// Create a new scheduler with the given MIDI connection.
    pub fn new(connection: Arc<Mutex<MidiOutputConnection>>) -> Self {
        Self::with_tempo(connection, 120)
    }

    /// Create a new scheduler with the given MIDI connection and initial tempo.
    pub fn with_tempo(connection: Arc<Mutex<MidiOutputConnection>>, initial_tempo: u16) -> Self {
        let (sender, receiver) = mpsc::channel();
        let current_time_ms = Arc::new(Mutex::new(0u64));
        let idle_signal = Arc::new((Mutex::new(true), Condvar::new()));

        let time_clone = current_time_ms.clone();
        let idle_clone = idle_signal.clone();

        let thread = thread::spawn(move || {
            Self::run(receiver, connection, time_clone, idle_clone, initial_tempo);
        });

        Self {
            sender,
            thread: Some(thread),
            current_time_ms,
            idle_signal,
        }
    }

    /// Schedule a MIDI message at the given time offset (ms).
    pub fn schedule(&self, time_ms: u64, message: Vec<u8>) {
        // Mark as busy
        if let Ok(mut idle) = self.idle_signal.0.lock() {
            *idle = false;
        }
        let _ = self
            .sender
            .send(SchedulerCommand::Schedule { time_ms, message });
    }

    /// Stop all playing notes.
    pub fn stop(&self) {
        let _ = self.sender.send(SchedulerCommand::Stop);
    }

    /// Wait for all scheduled events to complete.
    pub fn wait(&self) {
        let (lock, cvar) = &*self.idle_signal;
        let mut idle = lock.lock().unwrap();
        while !*idle {
            idle = cvar.wait(idle).unwrap();
        }
    }

    /// Get the current playback time in ms.
    #[allow(dead_code)]
    pub fn current_time_ms(&self) -> u64 {
        *self.current_time_ms.lock().unwrap()
    }

    /// Start the MIDI clock.
    pub fn start_clock(&self) {
        let _ = self.sender.send(SchedulerCommand::StartClock);
    }

    /// Stop the MIDI clock.
    pub fn stop_clock(&self) {
        let _ = self.sender.send(SchedulerCommand::StopClock);
    }

    /// Update the tempo (affects clock speed).
    pub fn set_tempo(&self, bpm: u16) {
        let _ = self.sender.send(SchedulerCommand::SetTempo(bpm));
    }

    /// Scheduler thread main loop.
    fn run(
        receiver: Receiver<SchedulerCommand>,
        connection: Arc<Mutex<MidiOutputConnection>>,
        current_time_ms: Arc<Mutex<u64>>,
        idle_signal: Arc<(Mutex<bool>, Condvar)>,
        initial_tempo: u16,
    ) {
        let mut queue: BinaryHeap<ScheduledEvent> = BinaryHeap::new();
        let start = Instant::now();
        let mut clock_running = false;
        let mut tempo = initial_tempo;
        let mut last_clock_tick = Instant::now();

        loop {
            // Update current time
            let now_ms = start.elapsed().as_millis() as u64;
            if let Ok(mut time) = current_time_ms.lock() {
                *time = now_ms;
            }

            // Process any pending commands (non-blocking)
            while let Ok(cmd) = receiver.try_recv() {
                match cmd {
                    SchedulerCommand::Schedule { time_ms, message } => {
                        queue.push(ScheduledEvent { time_ms, message });
                    }
                    SchedulerCommand::Stop => {
                        queue.clear();
                        clock_running = false;
                        // Send all notes off on all channels
                        if let Ok(mut conn) = connection.lock() {
                            for ch in 0..16u8 {
                                let _ = conn.send(&[0xB0 | ch, 123, 0]);
                            }
                            // Send MIDI Stop
                            let _ = conn.send(&[0xFC]);
                        }
                    }
                    SchedulerCommand::SetTempo(bpm) => {
                        tempo = bpm;
                    }
                    SchedulerCommand::Shutdown => {
                        if clock_running {
                            if let Ok(mut conn) = connection.lock() {
                                let _ = conn.send(&[0xFC]); // Stop
                            }
                        }
                        return;
                    }
                    SchedulerCommand::StartClock => {
                        clock_running = true;
                        last_clock_tick = Instant::now();
                        if let Ok(mut conn) = connection.lock() {
                            let _ = conn.send(&[0xFA]); // MIDI Start
                        }
                    }
                    SchedulerCommand::StopClock => {
                        clock_running = false;
                        if let Ok(mut conn) = connection.lock() {
                            let _ = conn.send(&[0xFC]); // MIDI Stop
                        }
                    }
                }
            }

            // Send MIDI clock ticks if running
            if clock_running {
                let tick_interval_us = (60_000_000u64 / tempo as u64) / 24;
                let elapsed = last_clock_tick.elapsed().as_micros() as u64;

                if elapsed >= tick_interval_us {
                    if let Ok(mut conn) = connection.lock() {
                        let _ = conn.send(&[0xF8]); // Clock tick
                    }
                    last_clock_tick = Instant::now();
                }
            }

            // Send any events that are due
            while let Some(event) = queue.peek() {
                if event.time_ms <= now_ms {
                    let event = queue.pop().unwrap();
                    if let Ok(mut conn) = connection.lock() {
                        let _ = conn.send(&event.message);
                    }
                } else {
                    break;
                }
            }

            // Update idle status (clock running counts as not idle)
            if queue.is_empty() && !clock_running {
                let (lock, cvar) = &*idle_signal;
                if let Ok(mut idle) = lock.lock() {
                    *idle = true;
                    cvar.notify_all();
                }
            }

            // Small sleep to prevent busy-waiting
            thread::sleep(StdDuration::from_micros(500));
        }
    }
}

impl Drop for Scheduler {
    fn drop(&mut self) {
        let _ = self.sender.send(SchedulerCommand::Shutdown);
        if let Some(thread) = self.thread.take() {
            let _ = thread.join();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn scheduled_event_ordering() {
        // Earlier events should have higher priority (min-heap)
        let early = ScheduledEvent {
            time_ms: 100,
            message: vec![],
        };
        let late = ScheduledEvent {
            time_ms: 200,
            message: vec![],
        };

        // In a max-heap with reversed ordering, early > late
        assert!(early > late);
    }

    #[test]
    fn clock_tick_interval_at_120_bpm() {
        // 24 ticks per quarter note
        // At 120 BPM: quarter note = 500ms
        // Tick interval = 500ms / 24 = 20.833...ms
        let bpm = 120u16;
        let quarter_note_ms = 60_000u64 / bpm as u64;
        let tick_interval_us = (quarter_note_ms * 1000) / 24;

        assert_eq!(quarter_note_ms, 500);
        assert_eq!(tick_interval_us, 20833);
    }

    #[test]
    fn clock_tick_interval_at_140_bpm() {
        let bpm = 140u16;
        let quarter_note_ms = 60_000u64 / bpm as u64;
        let tick_interval_us = (quarter_note_ms * 1000) / 24;

        assert_eq!(quarter_note_ms, 428); // 60000/140 = 428.57
        assert_eq!(tick_interval_us, 17833); // 428000/24 = 17833.33
    }
}
