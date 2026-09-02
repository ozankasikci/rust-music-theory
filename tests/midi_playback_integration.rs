#![cfg(feature = "midi-playback")]

use rust_music_theory::chord::{Chord, Number, Quality};
use rust_music_theory::midi::{Channel, Duration, MidiPlayer, MidiPorts, Velocity};
use rust_music_theory::note::{Pitch, PitchSymbol::*};

#[test]
fn list_ports_does_not_panic() {
    let ports = MidiPorts::list();
    assert!(ports.is_ok());
}

#[test]
fn ports_iteration() {
    let ports = MidiPorts::list().unwrap();
    for (i, name) in ports.iter().enumerate() {
        println!("Port {}: {}", i, name);
    }
}

#[test]
fn connect_invalid_port_name() {
    let result = MidiPlayer::connect("This Port Does Not Exist 12345");
    assert!(result.is_err());
}

#[test]
fn connect_invalid_port_index() {
    let result = MidiPlayer::connect_index(99999);
    assert!(result.is_err());
}

#[test]
#[ignore] // Requires hardware: cargo test --features midi-playback -- --ignored
fn play_chord_blocking() {
    let ports = MidiPorts::list().unwrap();
    if ports.is_empty() {
        println!("No MIDI ports available, skipping test");
        return;
    }

    let mut player = MidiPlayer::connect_index(0).unwrap();
    player.set_tempo(120);

    let chord = Chord::new(Pitch::from(C), Quality::Major, Number::Triad);
    player.play(&chord, Duration::Quarter, Velocity::new(100).unwrap());
}

#[test]
#[ignore] // Requires hardware
fn play_chord_async() {
    let ports = MidiPorts::list().unwrap();
    if ports.is_empty() {
        return;
    }

    let mut player = MidiPlayer::connect_index(0).unwrap();
    player.set_tempo(120);

    let c_chord = Chord::new(Pitch::from(C), Quality::Major, Number::Triad);
    let g_chord = Chord::new(Pitch::from(G), Quality::Major, Number::Triad);

    player.play_async(&c_chord, Duration::Quarter, Velocity::new(100).unwrap());
    player.play_async(&g_chord, Duration::Quarter, Velocity::new(100).unwrap());

    player.wait();
}

#[test]
#[ignore] // Requires hardware
fn play_with_rest() {
    let ports = MidiPorts::list().unwrap();
    if ports.is_empty() {
        return;
    }

    let mut player = MidiPlayer::connect_index(0).unwrap();
    player.set_tempo(120);

    let chord = Chord::new(Pitch::from(C), Quality::Major, Number::Triad);

    player.play_async(&chord, Duration::Quarter, Velocity::new(100).unwrap());
    player.rest_async(Duration::Quarter);
    player.play_async(&chord, Duration::Quarter, Velocity::new(100).unwrap());

    player.wait();
}

#[test]
#[ignore] // Requires hardware
fn change_channel() {
    let ports = MidiPorts::list().unwrap();
    if ports.is_empty() {
        return;
    }

    let mut player = MidiPlayer::connect_index(0).unwrap();
    player.set_tempo(120);

    let chord = Chord::new(Pitch::from(C), Quality::Major, Number::Triad);

    player.set_channel(Channel::new(0).unwrap());
    player.play_async(&chord, Duration::Quarter, Velocity::new(100).unwrap());

    player.set_channel(Channel::new(1).unwrap());
    player.play_async(&chord, Duration::Quarter, Velocity::new(100).unwrap());

    player.wait();
}

#[test]
#[ignore]
fn control_change_modulation() {
    let ports = MidiPorts::list().unwrap();
    if ports.is_empty() {
        return;
    }

    let player = MidiPlayer::connect_index(0).unwrap();
    player.control_change(1, 64); // Modulation at 50%
    player.control_change(1, 127); // Modulation at 100%
    player.control_change(1, 0); // Modulation off
}

#[test]
#[ignore]
fn program_change_switches_instrument() {
    let ports = MidiPorts::list().unwrap();
    if ports.is_empty() {
        return;
    }

    let player = MidiPlayer::connect_index(0).unwrap();
    player.program_change(0); // Piano
    player.program_change(25); // Acoustic Guitar
    player.program_change(40); // Violin
}

#[test]
#[ignore]
fn program_change_with_bank_select() {
    let ports = MidiPorts::list().unwrap();
    if ports.is_empty() {
        return;
    }

    let player = MidiPlayer::connect_index(0).unwrap();
    player.program_change_with_bank(5, 0, 1);
}

#[test]
#[ignore]
fn midi_clock_sends_to_daw() {
    use std::thread;
    use std::time::Duration;

    let ports = MidiPorts::list().unwrap();
    if ports.is_empty() {
        return;
    }

    let mut player = MidiPlayer::connect_index(0).unwrap();
    player.set_tempo(120);
    player.start_clock();
    thread::sleep(Duration::from_secs(2));
    player.stop_clock();
}
