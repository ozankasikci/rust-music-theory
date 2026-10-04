//! Quick test for MIDI playback.
//! Run with: cargo run --example midi_test --features midi-playback

use rust_music_theory::chord::{Chord, Number, Quality};
use rust_music_theory::midi::playback::{MidiPlayer, MidiPorts};
use rust_music_theory::midi::{Duration, Velocity};
use rust_music_theory::note::{Pitch, PitchSymbol::*};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // List available ports
    let ports = MidiPorts::list()?;
    println!("Available MIDI ports:");
    for (i, name) in ports.iter().enumerate() {
        println!("  {}: {}", i, name);
    }

    if ports.is_empty() {
        println!("\nNo MIDI ports found. Connect a MIDI device and try again.");
        return Ok(());
    }

    // Connect to first port
    println!("\nConnecting to port 0...");
    let mut player = MidiPlayer::connect_index(0)?;
    player.set_tempo(120);

    println!("Playing C Major chord...");
    let c_major = Chord::new(Pitch::from(C), Quality::Major, Number::Triad);
    player.play(&c_major, Duration::Half, Velocity::new(100).unwrap());

    println!("Playing G Major chord...");
    let g_major = Chord::new(Pitch::from(G), Quality::Major, Number::Triad);
    player.play(&g_major, Duration::Half, Velocity::new(100).unwrap());

    println!("Playing A Minor chord...");
    let a_minor = Chord::new(Pitch::from(A), Quality::Minor, Number::Triad);
    player.play(&a_minor, Duration::Half, Velocity::new(100).unwrap());

    println!("Playing F Major chord...");
    let f_major = Chord::new(Pitch::from(F), Quality::Major, Number::Triad);
    player.play(&f_major, Duration::Whole, Velocity::new(100).unwrap());

    println!("Done!");
    Ok(())
}
