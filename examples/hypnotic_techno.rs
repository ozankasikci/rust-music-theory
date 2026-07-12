//! Hypnotic techno bass with MIDI CC automation.
//! Run with: cargo run --example hypnotic_techno --features midi-playback

use rust_music_theory::midi::playback::{MidiPorts, MidiPlayer};
use rust_music_theory::midi::{Duration as NoteDuration, Velocity};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let ports = MidiPorts::list()?;
    if ports.is_empty() {
        println!("No MIDI ports found.");
        return Ok(());
    }

    let mut player = MidiPlayer::connect_index(0)?;
    player.set_tempo(128);

    let hard = Velocity::new(120).unwrap();
    let medium = Velocity::new(90).unwrap();
    let soft = Velocity::new(60).unwrap();

    // Bass notes (E1 and octave)
    let root: u8 = 28;      // E1
    let fifth: u8 = 35;     // B1
    let octave: u8 = 40;    // E2

    println!("Starting hypnotic techno bass...");
    println!("Using CC 1 (mod wheel) and CC 74 (filter cutoff)");
    println!("Map these CCs to your synth's filter in Ableton!");
    println!("Press Ctrl+C to stop\n");

    // Start MIDI clock so Ableton can sync
    player.start_clock();

    // 8 bar loop, repeat 4 times
    for cycle in 0..4 {
        println!("Cycle {} of 4", cycle + 1);

        for bar in 0..8 {
            // Calculate filter sweep position (0-127 over 8 bars)
            let filter_base = ((bar as f32 / 8.0) * 100.0) as u8;

            for beat in 0..4 {
                // Pulsing filter on each beat
                let filter_val = filter_base + (beat * 8);
                player.control_change(74, filter_val.min(127)); // Filter cutoff

                // Modulation builds through the bar
                let mod_val = (beat * 32) as u8;
                player.control_change(1, mod_val); // Mod wheel

                // Beat pattern
                match beat {
                    0 => {
                        // Downbeat - hard hit
                        player.play_note(root, NoteDuration::Eighth, hard);
                        player.play_note(root, NoteDuration::Eighth, medium);
                    }
                    1 => {
                        // Offbeat groove
                        player.play_note(root, NoteDuration::Sixteenth, soft);
                        player.rest(NoteDuration::Sixteenth);
                        player.play_note(octave, NoteDuration::Sixteenth, medium);
                        player.play_note(root, NoteDuration::Sixteenth, soft);
                    }
                    2 => {
                        // Syncopation
                        player.rest(NoteDuration::Sixteenth);
                        player.play_note(root, NoteDuration::Sixteenth, hard);
                        player.play_note(fifth, NoteDuration::Eighth, medium);
                    }
                    3 => {
                        // Build tension
                        player.play_note(root, NoteDuration::Sixteenth, medium);
                        player.play_note(root, NoteDuration::Sixteenth, soft);
                        player.play_note(octave, NoteDuration::Sixteenth, hard);
                        player.play_note(root, NoteDuration::Sixteenth, medium);
                    }
                    _ => {}
                }
            }
        }

        // Reset filter for next cycle
        player.control_change(74, 40);
        player.control_change(1, 0);
    }

    player.stop_clock();
    println!("\nDone!");
    Ok(())
}
