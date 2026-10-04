//! MIDI CC, Program Change, and Clock demo.
//! Run with: cargo run --example midi_cc_demo --features midi-playback

use std::thread;
use std::time::Duration;

use rust_music_theory::midi::playback::{MidiPlayer, MidiPorts};
use rust_music_theory::midi::{Duration as NoteDuration, Velocity};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let ports = MidiPorts::list()?;
    println!("Available MIDI ports:");
    for (i, name) in ports.iter().enumerate() {
        println!("  {}: {}", i, name);
    }

    if ports.is_empty() {
        println!("\nNo MIDI ports found.");
        return Ok(());
    }

    let mut player = MidiPlayer::connect_index(0)?;
    player.set_tempo(120);

    // Demo 1: Program Change
    println!("\n--- Program Change Demo ---");
    println!("Switching to Piano (Program 0)...");
    player.program_change(0);
    player.play_note(60, NoteDuration::Quarter, Velocity::new(100).unwrap());

    println!("Switching to Electric Piano (Program 4)...");
    player.program_change(4);
    player.play_note(60, NoteDuration::Quarter, Velocity::new(100).unwrap());

    // Demo 2: Control Change
    println!("\n--- Control Change Demo ---");
    println!("Modulation wheel sweep...");
    for i in (0..=127).step_by(16) {
        player.control_change(1, i); // Modulation
        thread::sleep(Duration::from_millis(100));
    }
    player.control_change(1, 0); // Reset

    // Demo 3: MIDI Clock
    println!("\n--- MIDI Clock Demo ---");
    println!("Starting clock at 120 BPM for 4 seconds...");
    println!("(Enable 'Ext' sync in your DAW to see it sync)");
    player.start_clock();
    thread::sleep(Duration::from_secs(4));
    player.stop_clock();
    println!("Clock stopped.");

    println!("\nDemo complete!");
    Ok(())
}
