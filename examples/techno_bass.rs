//! Techno bass pattern.
//! Run with: cargo run --example techno_bass --features midi-playback

use rust_music_theory::midi::playback::{MidiPlayer, MidiPorts};
use rust_music_theory::midi::{Duration, Velocity};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let ports = MidiPorts::list()?;
    if ports.is_empty() {
        println!("No MIDI ports found.");
        return Ok(());
    }

    let mut player = MidiPlayer::connect_index(0)?;
    player.set_tempo(130); // Classic techno tempo

    let hard = Velocity::new(120).unwrap();
    let medium = Velocity::new(90).unwrap();
    let soft = Velocity::new(70).unwrap();

    // E1 = MIDI note 28, E2 = 40
    let root_low: u8 = 28; // E1
    let root_high: u8 = 40; // E2
    let fifth: u8 = 35; // B1

    println!("Playing techno bass... (Ctrl+C to stop)");

    // 4 bar loop, repeat 4 times
    for _ in 0..4 {
        // Bar 1-2: Driving root
        for _ in 0..8 {
            player.play_note(root_low, Duration::Eighth, hard);
        }

        // Bar 3: Syncopated pattern
        player.play_note(root_low, Duration::Eighth, hard);
        player.play_note(root_high, Duration::Sixteenth, medium);
        player.play_note(root_low, Duration::Sixteenth, soft);
        player.play_note(root_low, Duration::Eighth, hard);
        player.play_note(fifth, Duration::Eighth, medium);
        player.play_note(root_low, Duration::Eighth, hard);
        player.play_note(root_high, Duration::Sixteenth, medium);
        player.play_note(root_low, Duration::Sixteenth, soft);
        player.play_note(root_low, Duration::Eighth, hard);
        player.play_note(root_low, Duration::Eighth, medium);

        // Bar 4: Build-up
        player.play_note(root_low, Duration::Sixteenth, hard);
        player.play_note(root_low, Duration::Sixteenth, medium);
        player.play_note(root_low, Duration::Sixteenth, hard);
        player.play_note(root_low, Duration::Sixteenth, medium);
        player.play_note(root_high, Duration::Sixteenth, hard);
        player.play_note(root_high, Duration::Sixteenth, medium);
        player.play_note(root_high, Duration::Sixteenth, hard);
        player.play_note(root_high, Duration::Sixteenth, medium);
        player.play_note(fifth, Duration::Eighth, hard);
        player.play_note(root_low, Duration::Eighth, hard);
        player.play_note(root_high, Duration::Quarter, hard);
    }

    println!("Done!");
    Ok(())
}
