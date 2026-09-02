//! Nocturne for two pianos (two MIDI channels) - with true polyphony.
//! Run with: cargo run --example nocturne_duo --features midi-playback
//!
//! Setup in Ableton:
//! - Track 1: MIDI From = IAC Driver, Channel = 1, load a bright piano
//! - Track 2: MIDI From = IAC Driver, Channel = 2, load a warm piano

use rust_music_theory::midi::playback::{MidiPlayer, MidiPorts};
use rust_music_theory::midi::{Channel, Duration, Velocity};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let ports = MidiPorts::list()?;
    if ports.is_empty() {
        println!("No MIDI ports found.");
        return Ok(());
    }

    let mut player = MidiPlayer::connect_index(0)?;
    player.set_tempo(58);

    let ch1 = Channel::new(0).unwrap(); // Melody
    let ch2 = Channel::new(1).unwrap(); // Accompaniment

    let pp = Velocity::new(40).unwrap();
    let p = Velocity::new(55).unwrap();
    let mp = Velocity::new(70).unwrap();
    let mf = Velocity::new(85).unwrap();
    let f = Velocity::new(100).unwrap();

    println!("Nocturne for Two Pianos in E minor");
    println!("Channel 1: Melody");
    println!("Channel 2: Accompaniment\n");

    player.start_clock();

    // Helper: schedule accompaniment arpeggio (4 sixteenths = 1 beat)
    fn arpeggio(player: &mut MidiPlayer, notes: &[u8], vel: Velocity) {
        for &note in notes {
            player.play_note_async(note, Duration::Sixteenth, vel);
        }
    }

    // === Bar 1: Em ===
    let bar_start = player.cursor();

    // Accompaniment (ch2) - arpeggiated E minor
    player.set_channel(ch2);
    arpeggio(&mut player, &[40, 47, 52, 47], pp); // E2, B2, E3, B2
    let ch2_end = player.cursor();

    // Seek back to bar start for melody
    player.seek(bar_start);
    player.set_channel(ch1);
    player.rest_async(Duration::Eighth); // Melody enters after 8th rest
    player.play_note_async(64, Duration::Eighth, mp); // E4
    player.play_note_async(67, Duration::Eighth, mf); // G4
    player.play_note_async(71, Duration::Eighth, mp); // B4
    let ch1_end = player.cursor();

    // Advance to end of bar (whichever is longer)
    player.seek(ch1_end.max(ch2_end));

    // === Bar 2: Em ===
    let bar_start = player.cursor();

    player.set_channel(ch2);
    arpeggio(&mut player, &[40, 47, 52, 47], pp);
    let ch2_end = player.cursor();

    player.seek(bar_start);
    player.set_channel(ch1);
    player.play_note_async(72, Duration::Dotted(Box::new(Duration::Quarter)), mf); // C5
    player.play_note_async(71, Duration::Eighth, mp); // B4
    let ch1_end = player.cursor();

    player.seek(ch1_end.max(ch2_end));

    // === Bar 3: Am ===
    let bar_start = player.cursor();

    player.set_channel(ch2);
    arpeggio(&mut player, &[45, 52, 57, 52], pp); // A2, E3, A3, E3
    let ch2_end = player.cursor();

    player.seek(bar_start);
    player.set_channel(ch1);
    player.play_note_async(69, Duration::Quarter, mf); // A4
    player.play_note_async(67, Duration::Eighth, mp); // G4
    player.play_note_async(64, Duration::Eighth, p); // E4
    let ch1_end = player.cursor();

    player.seek(ch1_end.max(ch2_end));

    // === Bar 4: B7 ===
    let bar_start = player.cursor();

    player.set_channel(ch2);
    arpeggio(&mut player, &[47, 54, 59, 54], p); // B2, F#3, B3, F#3
    let ch2_end = player.cursor();

    player.seek(bar_start);
    player.set_channel(ch1);
    player.play_note_async(66, Duration::Quarter, mf); // F#4
    player.play_note_async(64, Duration::Quarter, mp); // E4
    let ch1_end = player.cursor();

    player.seek(ch1_end.max(ch2_end));

    // === Bar 5: C major - brightness ===
    let bar_start = player.cursor();

    player.set_channel(ch2);
    arpeggio(&mut player, &[48, 55, 60, 55], mp); // C3, G3, C4, G3
    let ch2_end = player.cursor();

    player.seek(bar_start);
    player.set_channel(ch1);
    player.play_note_async(67, Duration::Eighth, mf); // G4
    player.play_note_async(72, Duration::Eighth, f); // C5
    player.play_note_async(74, Duration::Quarter, f); // D5
    let ch1_end = player.cursor();

    player.seek(ch1_end.max(ch2_end));

    // === Bar 6: G major - climax ===
    let bar_start = player.cursor();

    player.set_channel(ch2);
    arpeggio(&mut player, &[43, 50, 55, 50], mf); // G2, D3, G3, D3
    let ch2_end = player.cursor();

    player.seek(bar_start);
    player.set_channel(ch1);
    player.play_note_async(76, Duration::Quarter, f); // E5 - peak!
    player.play_note_async(74, Duration::Eighth, mf); // D5
    player.play_note_async(72, Duration::Eighth, mp); // C5
    let ch1_end = player.cursor();

    player.seek(ch1_end.max(ch2_end));

    // === Bar 7: Am - descending ===
    let bar_start = player.cursor();

    player.set_channel(ch2);
    arpeggio(&mut player, &[45, 52, 57, 52], p);
    let ch2_end = player.cursor();

    player.seek(bar_start);
    player.set_channel(ch1);
    player.play_note_async(71, Duration::Eighth, mp); // B4
    player.play_note_async(69, Duration::Eighth, p); // A4
    player.play_note_async(67, Duration::Eighth, p); // G4
    player.play_note_async(66, Duration::Eighth, pp); // F#4
    let ch1_end = player.cursor();

    player.seek(ch1_end.max(ch2_end));

    // === Bar 8: Em - brief rest ===
    let bar_start = player.cursor();

    player.set_channel(ch2);
    arpeggio(&mut player, &[40, 47, 52, 47], p);
    let ch2_end = player.cursor();

    player.seek(bar_start);
    player.set_channel(ch1);
    player.play_note_async(64, Duration::Half, mp); // E4
    let ch1_end = player.cursor();

    player.seek(ch1_end.max(ch2_end));

    // === Bar 9: Em - theme continues ===
    let bar_start = player.cursor();

    player.set_channel(ch2);
    arpeggio(&mut player, &[40, 47, 52, 47], pp);
    let ch2_end = player.cursor();

    player.seek(bar_start);
    player.set_channel(ch1);
    player.rest_async(Duration::Eighth);
    player.play_note_async(67, Duration::Eighth, mp); // G4
    player.play_note_async(71, Duration::Eighth, mf); // B4
    player.play_note_async(72, Duration::Eighth, mp); // C5
    let ch1_end = player.cursor();

    player.seek(ch1_end.max(ch2_end));

    // === Bar 10: Em ===
    let bar_start = player.cursor();

    player.set_channel(ch2);
    arpeggio(&mut player, &[40, 47, 52, 47], pp);
    let ch2_end = player.cursor();

    player.seek(bar_start);
    player.set_channel(ch1);
    player.play_note_async(71, Duration::Dotted(Box::new(Duration::Quarter)), mf); // B4
    player.play_note_async(69, Duration::Eighth, mp); // A4
    let ch1_end = player.cursor();

    player.seek(ch1_end.max(ch2_end));

    // === Bar 11: Am ===
    let bar_start = player.cursor();

    player.set_channel(ch2);
    arpeggio(&mut player, &[45, 52, 57, 52], pp);
    let ch2_end = player.cursor();

    player.seek(bar_start);
    player.set_channel(ch1);
    player.play_note_async(67, Duration::Quarter, mf); // G4
    player.play_note_async(69, Duration::Eighth, mp); // A4
    player.play_note_async(67, Duration::Eighth, p); // G4
    let ch1_end = player.cursor();

    player.seek(ch1_end.max(ch2_end));

    // === Bar 12: B7 ===
    let bar_start = player.cursor();

    player.set_channel(ch2);
    arpeggio(&mut player, &[47, 54, 59, 54], p);
    let ch2_end = player.cursor();

    player.seek(bar_start);
    player.set_channel(ch1);
    player.play_note_async(66, Duration::Quarter, mf); // F#4
    player.play_note_async(67, Duration::Quarter, mp); // G4
    let ch1_end = player.cursor();

    player.seek(ch1_end.max(ch2_end));

    // === Bar 13: C major - building ===
    let bar_start = player.cursor();

    player.set_channel(ch2);
    arpeggio(&mut player, &[48, 55, 60, 55], mp);
    let ch2_end = player.cursor();

    player.seek(bar_start);
    player.set_channel(ch1);
    player.play_note_async(72, Duration::Eighth, mf); // C5
    player.play_note_async(74, Duration::Eighth, f); // D5
    player.play_note_async(76, Duration::Quarter, f); // E5
    let ch1_end = player.cursor();

    player.seek(ch1_end.max(ch2_end));

    // === Bar 14: G major - second climax ===
    let bar_start = player.cursor();

    player.set_channel(ch2);
    arpeggio(&mut player, &[43, 50, 55, 50], mf);
    let ch2_end = player.cursor();

    player.seek(bar_start);
    player.set_channel(ch1);
    player.play_note_async(79, Duration::Quarter, f); // G5
    player.play_note_async(76, Duration::Eighth, mf); // E5
    player.play_note_async(74, Duration::Eighth, mp); // D5
    let ch1_end = player.cursor();

    player.seek(ch1_end.max(ch2_end));

    // === Bar 15: Am - winding down ===
    let bar_start = player.cursor();

    player.set_channel(ch2);
    arpeggio(&mut player, &[45, 52, 57, 52], p);
    let ch2_end = player.cursor();

    player.seek(bar_start);
    player.set_channel(ch1);
    player.play_note_async(72, Duration::Eighth, mp); // C5
    player.play_note_async(71, Duration::Eighth, p); // B4
    player.play_note_async(69, Duration::Eighth, p); // A4
    player.play_note_async(67, Duration::Eighth, pp); // G4
    let ch1_end = player.cursor();

    player.seek(ch1_end.max(ch2_end));

    // === Bar 16: Em - settling ===
    let bar_start = player.cursor();

    player.set_channel(ch2);
    arpeggio(&mut player, &[40, 47, 52, 47], pp);
    let ch2_end = player.cursor();

    player.seek(bar_start);
    player.set_channel(ch1);
    player.play_note_async(64, Duration::Half, p); // E4
    let ch1_end = player.cursor();

    player.seek(ch1_end.max(ch2_end));

    // ============================================
    // B SECTION - Slight variation of A (16 bars)
    // ============================================

    // === Bar 17: Em (melody up a step) ===
    let bar_start = player.cursor();

    player.set_channel(ch2);
    arpeggio(&mut player, &[40, 47, 52, 47], pp);
    let ch2_end = player.cursor();

    player.seek(bar_start);
    player.set_channel(ch1);
    player.rest_async(Duration::Eighth);
    player.play_note_async(67, Duration::Eighth, mp); // G4
    player.play_note_async(69, Duration::Eighth, mf); // A4
    player.play_note_async(72, Duration::Eighth, mp); // C5
    let ch1_end = player.cursor();

    player.seek(ch1_end.max(ch2_end));

    // === Bar 18: Em ===
    let bar_start = player.cursor();

    player.set_channel(ch2);
    arpeggio(&mut player, &[40, 47, 52, 47], pp);
    let ch2_end = player.cursor();

    player.seek(bar_start);
    player.set_channel(ch1);
    player.play_note_async(74, Duration::Dotted(Box::new(Duration::Quarter)), mf); // D5
    player.play_note_async(72, Duration::Eighth, mp); // C5
    let ch1_end = player.cursor();

    player.seek(ch1_end.max(ch2_end));

    // === Bar 19: Am ===
    let bar_start = player.cursor();

    player.set_channel(ch2);
    arpeggio(&mut player, &[45, 52, 57, 52], pp);
    let ch2_end = player.cursor();

    player.seek(bar_start);
    player.set_channel(ch1);
    player.play_note_async(71, Duration::Quarter, mf); // B4
    player.play_note_async(69, Duration::Eighth, mp); // A4
    player.play_note_async(67, Duration::Eighth, p); // G4
    let ch1_end = player.cursor();

    player.seek(ch1_end.max(ch2_end));

    // === Bar 20: B7 ===
    let bar_start = player.cursor();

    player.set_channel(ch2);
    arpeggio(&mut player, &[47, 54, 59, 54], p);
    let ch2_end = player.cursor();

    player.seek(bar_start);
    player.set_channel(ch1);
    player.play_note_async(66, Duration::Quarter, mf); // F#4
    player.play_note_async(64, Duration::Quarter, mp); // E4
    let ch1_end = player.cursor();

    player.seek(ch1_end.max(ch2_end));

    // === Bar 21: C major ===
    let bar_start = player.cursor();

    player.set_channel(ch2);
    arpeggio(&mut player, &[48, 55, 60, 55], mp);
    let ch2_end = player.cursor();

    player.seek(bar_start);
    player.set_channel(ch1);
    player.play_note_async(64, Duration::Eighth, mf); // E4
    player.play_note_async(67, Duration::Eighth, f); // G4
    player.play_note_async(72, Duration::Quarter, f); // C5
    let ch1_end = player.cursor();

    player.seek(ch1_end.max(ch2_end));

    // === Bar 22: G major ===
    let bar_start = player.cursor();

    player.set_channel(ch2);
    arpeggio(&mut player, &[43, 50, 55, 50], mf);
    let ch2_end = player.cursor();

    player.seek(bar_start);
    player.set_channel(ch1);
    player.play_note_async(74, Duration::Quarter, f); // D5
    player.play_note_async(72, Duration::Eighth, mf); // C5
    player.play_note_async(71, Duration::Eighth, mp); // B4
    let ch1_end = player.cursor();

    player.seek(ch1_end.max(ch2_end));

    // === Bar 23: Am ===
    let bar_start = player.cursor();

    player.set_channel(ch2);
    arpeggio(&mut player, &[45, 52, 57, 52], p);
    let ch2_end = player.cursor();

    player.seek(bar_start);
    player.set_channel(ch1);
    player.play_note_async(69, Duration::Eighth, mp); // A4
    player.play_note_async(67, Duration::Eighth, p); // G4
    player.play_note_async(66, Duration::Eighth, p); // F#4
    player.play_note_async(64, Duration::Eighth, pp); // E4
    let ch1_end = player.cursor();

    player.seek(ch1_end.max(ch2_end));

    // === Bar 24: Em ===
    let bar_start = player.cursor();

    player.set_channel(ch2);
    arpeggio(&mut player, &[40, 47, 52, 47], pp);
    let ch2_end = player.cursor();

    player.seek(bar_start);
    player.set_channel(ch1);
    player.play_note_async(64, Duration::Half, pp); // E4
    let ch1_end = player.cursor();

    player.seek(ch1_end.max(ch2_end));

    // === Bar 25: Em - B section continues ===
    let bar_start = player.cursor();

    player.set_channel(ch2);
    arpeggio(&mut player, &[40, 47, 52, 47], pp);
    let ch2_end = player.cursor();

    player.seek(bar_start);
    player.set_channel(ch1);
    player.rest_async(Duration::Eighth);
    player.play_note_async(69, Duration::Eighth, mp); // A4
    player.play_note_async(72, Duration::Eighth, mf); // C5
    player.play_note_async(74, Duration::Eighth, mp); // D5
    let ch1_end = player.cursor();

    player.seek(ch1_end.max(ch2_end));

    // === Bar 26: Em ===
    let bar_start = player.cursor();

    player.set_channel(ch2);
    arpeggio(&mut player, &[40, 47, 52, 47], pp);
    let ch2_end = player.cursor();

    player.seek(bar_start);
    player.set_channel(ch1);
    player.play_note_async(72, Duration::Dotted(Box::new(Duration::Quarter)), mf); // C5
    player.play_note_async(71, Duration::Eighth, mp); // B4
    let ch1_end = player.cursor();

    player.seek(ch1_end.max(ch2_end));

    // === Bar 27: Am ===
    let bar_start = player.cursor();

    player.set_channel(ch2);
    arpeggio(&mut player, &[45, 52, 57, 52], pp);
    let ch2_end = player.cursor();

    player.seek(bar_start);
    player.set_channel(ch1);
    player.play_note_async(69, Duration::Quarter, mf); // A4
    player.play_note_async(71, Duration::Eighth, mp); // B4
    player.play_note_async(69, Duration::Eighth, p); // A4
    let ch1_end = player.cursor();

    player.seek(ch1_end.max(ch2_end));

    // === Bar 28: B7 ===
    let bar_start = player.cursor();

    player.set_channel(ch2);
    arpeggio(&mut player, &[47, 54, 59, 54], p);
    let ch2_end = player.cursor();

    player.seek(bar_start);
    player.set_channel(ch1);
    player.play_note_async(67, Duration::Quarter, mf); // G4
    player.play_note_async(66, Duration::Quarter, mp); // F#4
    let ch1_end = player.cursor();

    player.seek(ch1_end.max(ch2_end));

    // === Bar 29: C major - building ===
    let bar_start = player.cursor();

    player.set_channel(ch2);
    arpeggio(&mut player, &[48, 55, 60, 55], mp);
    let ch2_end = player.cursor();

    player.seek(bar_start);
    player.set_channel(ch1);
    player.play_note_async(72, Duration::Eighth, mf); // C5
    player.play_note_async(76, Duration::Eighth, f); // E5
    player.play_note_async(79, Duration::Quarter, f); // G5
    let ch1_end = player.cursor();

    player.seek(ch1_end.max(ch2_end));

    // === Bar 30: G major - B section peak ===
    let bar_start = player.cursor();

    player.set_channel(ch2);
    arpeggio(&mut player, &[43, 50, 55, 50], mf);
    let ch2_end = player.cursor();

    player.seek(bar_start);
    player.set_channel(ch1);
    player.play_note_async(79, Duration::Quarter, f); // G5
    player.play_note_async(76, Duration::Eighth, mf); // E5
    player.play_note_async(74, Duration::Eighth, mp); // D5
    let ch1_end = player.cursor();

    player.seek(ch1_end.max(ch2_end));

    // === Bar 31: Am - winding down ===
    let bar_start = player.cursor();

    player.set_channel(ch2);
    arpeggio(&mut player, &[45, 52, 57, 52], p);
    let ch2_end = player.cursor();

    player.seek(bar_start);
    player.set_channel(ch1);
    player.play_note_async(72, Duration::Eighth, mp); // C5
    player.play_note_async(71, Duration::Eighth, p); // B4
    player.play_note_async(69, Duration::Eighth, p); // A4
    player.play_note_async(67, Duration::Eighth, pp); // G4
    let ch1_end = player.cursor();

    player.seek(ch1_end.max(ch2_end));

    // === Bar 32: Em - B section ends ===
    let bar_start = player.cursor();

    player.set_channel(ch2);
    arpeggio(&mut player, &[40, 47, 52, 47], pp);
    let ch2_end = player.cursor();

    player.seek(bar_start);
    player.set_channel(ch1);
    player.play_note_async(64, Duration::Half, p); // E4
    let ch1_end = player.cursor();

    player.seek(ch1_end.max(ch2_end));

    // ============================================
    // C SECTION - Return to A with minor differences
    // ============================================

    // === Bar 33: Em (like bar 1, slightly softer) ===
    let bar_start = player.cursor();

    player.set_channel(ch2);
    arpeggio(&mut player, &[40, 47, 52, 47], p);
    let ch2_end = player.cursor();

    player.seek(bar_start);
    player.set_channel(ch1);
    player.rest_async(Duration::Eighth);
    player.play_note_async(64, Duration::Eighth, p); // E4
    player.play_note_async(67, Duration::Eighth, mp); // G4
    player.play_note_async(71, Duration::Eighth, p); // B4
    let ch1_end = player.cursor();

    player.seek(ch1_end.max(ch2_end));

    // === Bar 34: Em (like bar 2) ===
    let bar_start = player.cursor();

    player.set_channel(ch2);
    arpeggio(&mut player, &[40, 47, 52, 47], p);
    let ch2_end = player.cursor();

    player.seek(bar_start);
    player.set_channel(ch1);
    player.play_note_async(72, Duration::Dotted(Box::new(Duration::Quarter)), mp); // C5
    player.play_note_async(71, Duration::Eighth, p); // B4
    let ch1_end = player.cursor();

    player.seek(ch1_end.max(ch2_end));

    // === Bar 35: Am (like bar 3, ending varies) ===
    let bar_start = player.cursor();

    player.set_channel(ch2);
    arpeggio(&mut player, &[45, 52, 57, 52], p);
    let ch2_end = player.cursor();

    player.seek(bar_start);
    player.set_channel(ch1);
    player.play_note_async(69, Duration::Quarter, mp); // A4
    player.play_note_async(67, Duration::Eighth, p); // G4
    player.play_note_async(66, Duration::Eighth, pp); // F#4 (was E4)
    let ch1_end = player.cursor();

    player.seek(ch1_end.max(ch2_end));

    // === Bar 36: B7 (like bar 4) ===
    let bar_start = player.cursor();

    player.set_channel(ch2);
    arpeggio(&mut player, &[47, 54, 59, 54], p);
    let ch2_end = player.cursor();

    player.seek(bar_start);
    player.set_channel(ch1);
    player.play_note_async(66, Duration::Quarter, mp); // F#4
    player.play_note_async(64, Duration::Quarter, p); // E4
    let ch1_end = player.cursor();

    player.seek(ch1_end.max(ch2_end));

    // === Bar 37: C major (like bar 5) ===
    let bar_start = player.cursor();

    player.set_channel(ch2);
    arpeggio(&mut player, &[48, 55, 60, 55], mp);
    let ch2_end = player.cursor();

    player.seek(bar_start);
    player.set_channel(ch1);
    player.play_note_async(67, Duration::Eighth, mp); // G4
    player.play_note_async(72, Duration::Eighth, mf); // C5
    player.play_note_async(74, Duration::Quarter, mf); // D5
    let ch1_end = player.cursor();

    player.seek(ch1_end.max(ch2_end));

    // === Bar 38: G major (like bar 6) ===
    let bar_start = player.cursor();

    player.set_channel(ch2);
    arpeggio(&mut player, &[43, 50, 55, 50], mf);
    let ch2_end = player.cursor();

    player.seek(bar_start);
    player.set_channel(ch1);
    player.play_note_async(76, Duration::Quarter, mf); // E5
    player.play_note_async(74, Duration::Eighth, mp); // D5
    player.play_note_async(72, Duration::Eighth, p); // C5
    let ch1_end = player.cursor();

    player.seek(ch1_end.max(ch2_end));

    // === Bar 39: Am (like bar 7) ===
    let bar_start = player.cursor();

    player.set_channel(ch2);
    arpeggio(&mut player, &[45, 52, 57, 52], p);
    let ch2_end = player.cursor();

    player.seek(bar_start);
    player.set_channel(ch1);
    player.play_note_async(71, Duration::Eighth, p); // B4
    player.play_note_async(69, Duration::Eighth, pp); // A4
    player.play_note_async(67, Duration::Eighth, pp); // G4
    player.play_note_async(66, Duration::Eighth, pp); // F#4
    let ch1_end = player.cursor();

    player.seek(ch1_end.max(ch2_end));

    // === Bar 40: Em (like bar 8) ===
    let bar_start = player.cursor();

    player.set_channel(ch2);
    arpeggio(&mut player, &[40, 47, 52, 47], pp);
    let ch2_end = player.cursor();

    player.seek(bar_start);
    player.set_channel(ch1);
    player.play_note_async(64, Duration::Half, p); // E4
    let ch1_end = player.cursor();

    player.seek(ch1_end.max(ch2_end));

    // === Bar 41: Em (like bar 9) ===
    let bar_start = player.cursor();

    player.set_channel(ch2);
    arpeggio(&mut player, &[40, 47, 52, 47], pp);
    let ch2_end = player.cursor();

    player.seek(bar_start);
    player.set_channel(ch1);
    player.rest_async(Duration::Eighth);
    player.play_note_async(67, Duration::Eighth, p); // G4
    player.play_note_async(71, Duration::Eighth, mp); // B4
    player.play_note_async(72, Duration::Eighth, p); // C5
    let ch1_end = player.cursor();

    player.seek(ch1_end.max(ch2_end));

    // === Bar 42: Em (like bar 10) ===
    let bar_start = player.cursor();

    player.set_channel(ch2);
    arpeggio(&mut player, &[40, 47, 52, 47], pp);
    let ch2_end = player.cursor();

    player.seek(bar_start);
    player.set_channel(ch1);
    player.play_note_async(71, Duration::Dotted(Box::new(Duration::Quarter)), mp); // B4
    player.play_note_async(67, Duration::Eighth, p); // G4 (was A4)
    let ch1_end = player.cursor();

    player.seek(ch1_end.max(ch2_end));

    // === Bar 43: Am (like bar 11) ===
    let bar_start = player.cursor();

    player.set_channel(ch2);
    arpeggio(&mut player, &[45, 52, 57, 52], pp);
    let ch2_end = player.cursor();

    player.seek(bar_start);
    player.set_channel(ch1);
    player.play_note_async(69, Duration::Quarter, mp); // A4 (was G4)
    player.play_note_async(67, Duration::Eighth, p); // G4
    player.play_note_async(64, Duration::Eighth, pp); // E4
    let ch1_end = player.cursor();

    player.seek(ch1_end.max(ch2_end));

    // === Bar 44: B7 (like bar 12) ===
    let bar_start = player.cursor();

    player.set_channel(ch2);
    arpeggio(&mut player, &[47, 54, 59, 54], pp);
    let ch2_end = player.cursor();

    player.seek(bar_start);
    player.set_channel(ch1);
    player.play_note_async(66, Duration::Quarter, p); // F#4
    player.play_note_async(64, Duration::Quarter, pp); // E4 (was G4)
    let ch1_end = player.cursor();

    player.seek(ch1_end.max(ch2_end));

    // === Bar 45: C major (like bar 13, gentler) ===
    let bar_start = player.cursor();

    player.set_channel(ch2);
    arpeggio(&mut player, &[48, 55, 60, 55], p);
    let ch2_end = player.cursor();

    player.seek(bar_start);
    player.set_channel(ch1);
    player.play_note_async(72, Duration::Eighth, p); // C5
    player.play_note_async(74, Duration::Eighth, mp); // D5
    player.play_note_async(72, Duration::Quarter, mp); // C5 (was E5)
    let ch1_end = player.cursor();

    player.seek(ch1_end.max(ch2_end));

    // === Bar 46: G major (like bar 14, lower peak) ===
    let bar_start = player.cursor();

    player.set_channel(ch2);
    arpeggio(&mut player, &[43, 50, 55, 50], p);
    let ch2_end = player.cursor();

    player.seek(bar_start);
    player.set_channel(ch1);
    player.play_note_async(74, Duration::Quarter, mp); // D5 (was G5)
    player.play_note_async(72, Duration::Eighth, p); // C5
    player.play_note_async(71, Duration::Eighth, pp); // B4
    let ch1_end = player.cursor();

    player.seek(ch1_end.max(ch2_end));

    // === Bar 47: Am (like bar 15, fading) ===
    let bar_start = player.cursor();

    player.set_channel(ch2);
    arpeggio(&mut player, &[45, 52, 57, 52], pp);
    let ch2_end = player.cursor();

    player.seek(bar_start);
    player.set_channel(ch1);
    player.play_note_async(69, Duration::Eighth, p); // A4
    player.play_note_async(67, Duration::Eighth, pp); // G4
    player.play_note_async(66, Duration::Eighth, pp); // F#4
    player.play_note_async(64, Duration::Eighth, pp); // E4
    let ch1_end = player.cursor();

    player.seek(ch1_end.max(ch2_end));

    // === Bar 48: Em - C section ends ===
    let bar_start = player.cursor();

    player.set_channel(ch2);
    arpeggio(&mut player, &[40, 47, 52, 47], pp);
    let ch2_end = player.cursor();

    player.seek(bar_start);
    player.set_channel(ch1);
    player.play_note_async(64, Duration::Half, pp); // E4
    let ch1_end = player.cursor();

    player.seek(ch1_end.max(ch2_end));

    // ============================================
    // CODA - Extended ending
    // ============================================

    // === Coda bar 1: Rising together ===
    let bar_start = player.cursor();

    player.set_channel(ch2);
    for &note in &[40, 47, 52, 59] {
        player.play_note_async(note, Duration::Eighth, pp);
    }
    let ch2_end = player.cursor();

    player.seek(bar_start);
    player.set_channel(ch1);
    for &note in &[64, 71, 76, 83] {
        player.play_note_async(note, Duration::Eighth, pp);
    }
    let ch1_end = player.cursor();

    player.seek(ch1_end.max(ch2_end));

    // === Coda bar 2: Gentle descent ===
    let bar_start = player.cursor();

    player.set_channel(ch2);
    for &note in &[52, 47, 45, 40] {
        player.play_note_async(note, Duration::Eighth, pp);
    }
    let ch2_end = player.cursor();

    player.seek(bar_start);
    player.set_channel(ch1);
    for &note in &[76, 71, 69, 64] {
        player.play_note_async(note, Duration::Eighth, pp);
    }
    let ch1_end = player.cursor();

    player.seek(ch1_end.max(ch2_end));

    // === Coda bar 3: Final sigh - Am to Em ===
    let bar_start = player.cursor();

    player.set_channel(ch2);
    player.play_note_async(45, Duration::Quarter, pp); // A2
    player.play_note_async(40, Duration::Quarter, pp); // E2
    let ch2_end = player.cursor();

    player.seek(bar_start);
    player.set_channel(ch1);
    player.play_note_async(69, Duration::Quarter, pp); // A4
    player.play_note_async(67, Duration::Quarter, pp); // G4
    let ch1_end = player.cursor();

    player.seek(ch1_end.max(ch2_end));

    // === Final chord: E minor - both together ===
    let bar_start = player.cursor();

    // Sustain on both channels
    player.set_channel(ch2);
    player.control_change(64, 127); // Sustain
    player.play_note_async(40, Duration::Whole, pp); // E2
    player.seek(bar_start);
    player.play_note_async(52, Duration::Whole, pp); // E3
    let ch2_end = player.cursor();

    player.seek(bar_start);
    player.set_channel(ch1);
    player.control_change(64, 127); // Sustain
    player.play_note_async(64, Duration::Whole, pp); // E4
    player.seek(bar_start);
    player.play_note_async(67, Duration::Whole, pp); // G4
    player.seek(bar_start);
    player.play_note_async(71, Duration::Whole, pp); // B4
    let ch1_end = player.cursor();

    player.seek(ch1_end.max(ch2_end));

    // Wait for everything to finish
    player.wait();
    std::thread::sleep(std::time::Duration::from_secs(3));

    // Release sustain
    player.set_channel(ch1);
    player.control_change(64, 0);
    player.set_channel(ch2);
    player.control_change(64, 0);

    player.stop_clock();

    println!("Fine.");
    Ok(())
}
