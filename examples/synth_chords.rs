//! Synth chord progression with CC automation.
//! Run with: cargo run --example synth_chords --features midi-playback

use rust_music_theory::chord::{Chord, Number, Quality};
use rust_music_theory::midi::playback::{MidiPlayer, MidiPorts};
use rust_music_theory::midi::{Duration, Velocity};
use rust_music_theory::note::{Pitch, PitchSymbol::*};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let ports = MidiPorts::list()?;
    if ports.is_empty() {
        println!("No MIDI ports found.");
        return Ok(());
    }

    let mut player = MidiPlayer::connect_index(0)?;
    player.set_tempo(110);

    // Use program 81 (Synth Lead) or 89 (Pad) - adjust for your setup
    player.program_change(89); // Warm Pad

    let vel = Velocity::new(85).unwrap();

    println!("Playing ethereal synth chords...");
    println!("CC 1 = filter/mod, CC 74 = brightness");
    println!("Press Ctrl+C to stop\n");

    player.start_clock();

    // Chord progression: Am - F - C - G (classic emotional progression)
    let progression = [
        (Pitch::from(A), Quality::Minor),
        (Pitch::from(F), Quality::Major),
        (Pitch::from(C), Quality::Major),
        (Pitch::from(G), Quality::Major),
    ];

    for cycle in 0..3 {
        println!("Cycle {}", cycle + 1);

        for (i, (root, quality)) in progression.iter().enumerate() {
            let chord = Chord::new(root.clone(), quality.clone(), Number::Triad);

            // Evolving filter - opens up through progression
            let filter = 50 + (i as u8 * 20);
            player.control_change(74, filter);

            // Subtle modulation swell
            for mod_val in (0..=60).step_by(15) {
                player.control_change(1, mod_val as u8);

                if mod_val == 0 {
                    // Play the chord
                    player.play(&chord, Duration::Half, vel);
                } else {
                    std::thread::sleep(std::time::Duration::from_millis(125));
                }
            }

            // Decay modulation
            for mod_val in (0..=60).rev().step_by(20) {
                player.control_change(1, mod_val as u8);
                std::thread::sleep(std::time::Duration::from_millis(80));
            }
        }
    }

    // Fade out
    for filter in (0..=50).rev().step_by(10) {
        player.control_change(74, filter as u8);
        std::thread::sleep(std::time::Duration::from_millis(100));
    }

    player.stop_clock();
    println!("\nDone!");
    Ok(())
}
