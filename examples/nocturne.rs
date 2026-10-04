//! A nocturne in the style of Chopin.
//! Run with: cargo run --example nocturne --features midi-playback

use rust_music_theory::midi::playback::{MidiPlayer, MidiPorts};
use rust_music_theory::midi::{Duration, Velocity};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let ports = MidiPorts::list()?;
    if ports.is_empty() {
        println!("No MIDI ports found.");
        return Ok(());
    }

    let mut player = MidiPlayer::connect_index(0)?;
    player.set_tempo(60); // Slow, expressive tempo

    // Use piano sound
    player.program_change(0);

    // Velocities for expression
    let pp = Velocity::new(45).unwrap(); // pianissimo
    let p = Velocity::new(60).unwrap(); // piano
    let mp = Velocity::new(75).unwrap(); // mezzo piano
    let mf = Velocity::new(90).unwrap(); // mezzo forte
    let f = Velocity::new(105).unwrap(); // forte

    println!("Playing Nocturne in E minor...\n");

    // E minor: E F# G A B C D E
    // Chord tones: Em (E G B), Am (A C E), B7 (B D# F# A), C (C E G)

    player.start_clock();

    // === Section A: Melancholic opening ===

    // Bar 1: E minor arpeggiated accompaniment + melody
    // Left hand arpeggio pattern (low)
    play_arpeggio(&mut player, &[40, 47, 52, 47], pp); // E2, B2, E3, B2

    // Melody enters - singing line
    player.play_note(64, Duration::Quarter, mp); // E4
    player.play_note(67, Duration::Eighth, mf); // G4
    player.play_note(71, Duration::Eighth, mp); // B4

    // Bar 2
    play_arpeggio(&mut player, &[40, 47, 52, 47], pp);
    player.play_note(72, Duration::Half, mf); // C5 - sustained

    // Bar 3: Move to Am
    play_arpeggio(&mut player, &[45, 52, 57, 52], pp); // A2, E3, A3, E3
    player.play_note(71, Duration::Quarter, mp); // B4
    player.play_note(69, Duration::Eighth, p); // A4
    player.play_note(67, Duration::Eighth, mp); // G4

    // Bar 4: Resolution hint
    play_arpeggio(&mut player, &[47, 54, 59, 54], pp); // B2, F#3, B3
    player.play_note(66, Duration::Dotted(Box::new(Duration::Quarter)), mf); // F#4
    player.play_note(64, Duration::Eighth, p); // E4

    // === Section B: Rising passion ===

    // Bar 5: C major - brightness
    play_arpeggio(&mut player, &[48, 55, 60, 55], p); // C3, G3, C4
    player.play_note(67, Duration::Eighth, mf); // G4
    player.play_note(69, Duration::Eighth, mf); // A4
    player.play_note(71, Duration::Eighth, f); // B4
    player.play_note(72, Duration::Eighth, f); // C5

    // Bar 6: Climax
    play_arpeggio(&mut player, &[43, 50, 55, 50], mp); // G2, D3, G3
    player.play_note(74, Duration::Quarter, f); // D5 - peak
    player.play_note(72, Duration::Quarter, mf); // C5

    // Bar 7: Descending
    play_arpeggio(&mut player, &[45, 52, 57, 52], p);
    player.play_note(71, Duration::Eighth, mf); // B4
    player.play_note(69, Duration::Eighth, mp); // A4
    player.play_note(67, Duration::Eighth, p); // G4
    player.play_note(66, Duration::Eighth, pp); // F#4

    // Bar 8: Return to E minor
    play_arpeggio(&mut player, &[40, 47, 52, 47], pp);
    player.play_note(64, Duration::Half, p); // E4 - home

    // === Coda: Fading ===

    // Final bars - dying away
    player.control_change(64, 127); // Sustain pedal on

    // Slow arpeggio
    for &note in &[40, 47, 52, 59, 64, 71, 76] {
        player.play_note(note, Duration::Eighth, pp);
    }

    // Final chord - E minor
    player.play_note(40, Duration::Whole, pp); // E2
    std::thread::sleep(std::time::Duration::from_millis(50));
    player.play_note(52, Duration::Whole, pp); // E3
    std::thread::sleep(std::time::Duration::from_millis(50));
    player.play_note(64, Duration::Whole, pp); // E4
    std::thread::sleep(std::time::Duration::from_millis(50));
    player.play_note(67, Duration::Whole, pp); // G4
    std::thread::sleep(std::time::Duration::from_millis(50));
    player.play_note(71, Duration::Whole, pp); // B4

    std::thread::sleep(std::time::Duration::from_secs(4));

    player.control_change(64, 0); // Sustain pedal off
    player.stop_clock();

    println!("Fine.");
    Ok(())
}

/// Play a broken chord arpeggio pattern
fn play_arpeggio(player: &mut MidiPlayer, notes: &[u8], vel: Velocity) {
    for &note in notes {
        player.play_note(note, Duration::Sixteenth, vel);
    }
}
