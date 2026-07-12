#![cfg(feature = "midi")]

use rust_music_theory::chord::{Chord, Quality, Number};
use rust_music_theory::scale::{Scale, ScaleType, Mode, Direction};
use rust_music_theory::note::{Pitch, PitchSymbol::*};
use rust_music_theory::midi::{
    MidiBuilder, MidiFile, Duration, Velocity, Channel, ToMidi,
};

#[test]
fn full_composition_workflow() {
    // Create chord progression
    let mut chords = MidiBuilder::new();
    chords
        .tempo(120)
        .add(&Chord::new(Pitch::from(C), Quality::Major, Number::Triad),
             Duration::Whole, Velocity::new(90).unwrap())
        .add(&Chord::new(Pitch::from(G), Quality::Major, Number::Triad),
             Duration::Whole, Velocity::new(90).unwrap())
        .add(&Chord::new(Pitch::from(A), Quality::Minor, Number::Triad),
             Duration::Whole, Velocity::new(90).unwrap())
        .add(&Chord::new(Pitch::from(F), Quality::Major, Number::Triad),
             Duration::Whole, Velocity::new(90).unwrap());

    // Create melody
    let mut melody = MidiBuilder::new();
    melody
        .rest(Duration::Half)  // Start after half note rest
        .add(&Scale::new(ScaleType::PentatonicMajor, Pitch::from(C), 5, None, Direction::Ascending).unwrap(),
             Duration::Eighth, Velocity::new(100).unwrap());

    // Combine into file
    let file = MidiFile::new()
        .tempo(120)
        .time_signature(4, 4)
        .track(chords, Channel::new(0).unwrap())
        .track(melody, Channel::new(1).unwrap());

    let bytes = file.to_bytes();

    // Verify MIDI header
    assert_eq!(&bytes[0..4], b"MThd");

    // Verify we have 3 tracks (tempo + 2 note tracks)
    let mtrk_count = bytes.windows(4).filter(|w| w == b"MTrk").count();
    assert_eq!(mtrk_count, 3);
}

#[test]
fn simple_chord_export() {
    let chord = Chord::new(Pitch::from(C), Quality::Major, Number::Seventh);
    let bytes = chord.to_midi(Duration::Half, Velocity::new(100).unwrap())
        .tempo(90)
        .to_bytes();

    assert_eq!(&bytes[0..4], b"MThd");
}

#[test]
fn all_chord_qualities() {
    let chords = [
        (Quality::Major, Number::Triad),
        (Quality::Minor, Number::Triad),
        (Quality::Diminished, Number::Triad),
        (Quality::Augmented, Number::Triad),
        (Quality::HalfDiminished, Number::Seventh),
        (Quality::Dominant, Number::Seventh),
        (Quality::Suspended2, Number::Triad),
        (Quality::Suspended4, Number::Triad),
    ];

    for (quality, number) in chords {
        let chord = Chord::new(Pitch::from(C), quality, number);
        let bytes = chord.to_midi(Duration::Quarter, Velocity::new(100).unwrap())
            .to_bytes();
        assert_eq!(&bytes[0..4], b"MThd", "Failed for quality {:?}", quality);
    }
}

#[test]
fn all_scale_types() {
    let scale_configs = [
        (ScaleType::Diatonic, Some(Mode::Ionian)),
        (ScaleType::Diatonic, Some(Mode::Dorian)),
        (ScaleType::Diatonic, Some(Mode::Aeolian)),
        (ScaleType::HarmonicMinor, None),
        (ScaleType::MelodicMinor, None),
        (ScaleType::PentatonicMajor, None),
        (ScaleType::PentatonicMinor, None),
        (ScaleType::Blues, None),
    ];

    for (scale_type, mode) in scale_configs {
        let scale = Scale::new(scale_type, Pitch::from(C), 4, mode, Direction::Ascending).unwrap();
        let bytes = scale.to_midi(Duration::Eighth, Velocity::new(80).unwrap())
            .to_bytes();
        assert_eq!(&bytes[0..4], b"MThd", "Failed for scale {:?}", scale_type);
    }
}

#[test]
fn duration_varieties() {
    let durations = [
        Duration::Whole,
        Duration::Half,
        Duration::Quarter,
        Duration::Eighth,
        Duration::Sixteenth,
        Duration::dotted(Duration::Quarter),
        Duration::triplet(Duration::Quarter),
    ];

    let chord = Chord::new(Pitch::from(C), Quality::Major, Number::Triad);

    for duration in durations {
        let bytes = chord.to_midi(duration.clone(), Velocity::new(100).unwrap())
            .to_bytes();
        assert_eq!(&bytes[0..4], b"MThd", "Failed for duration {:?}", duration);
    }
}

#[test]
fn builder_chaining() {
    let c = Chord::new(Pitch::from(C), Quality::Major, Number::Triad);
    let vel = Velocity::new(100).unwrap();

    let mut builder = MidiBuilder::new();
    builder
        .tempo(120)
        .time_signature(4, 4)
        .add(&c, Duration::Quarter, vel)
        .rest(Duration::Quarter)
        .add(&c, Duration::Quarter, vel)
        .at_beat(4.0)
        .add(&c, Duration::Whole, vel);

    let file = MidiFile::new()
        .track(builder, Channel::new(0).unwrap());

    let bytes = file.to_bytes();
    assert_eq!(&bytes[0..4], b"MThd");
}

#[test]
fn lead_sheet_theoretical_accidentals_export_at_the_correct_midi_octaves() {
    use midly::{MidiMessage, Smf, TrackEventKind};

    fn exported_pitches(symbol: &str) -> Vec<u8> {
        let chord = Chord::parse(symbol).unwrap();
        let bytes = chord
            .to_midi(Duration::Quarter, Velocity::new(100).unwrap())
            .to_bytes();
        let midi = Smf::parse(&bytes).unwrap();
        midi.tracks
            .iter()
            .flat_map(|track| track.iter())
            .filter_map(|event| match event.kind {
                TrackEventKind::Midi {
                    message: MidiMessage::NoteOn { key, vel },
                    ..
                } if vel.as_int() > 0 => Some(key.as_int()),
                _ => None,
            })
            .collect()
    }

    assert_eq!(exported_pitches("Cbmaj7"), [59, 63, 66, 70]);
    assert_eq!(exported_pitches("B#maj7"), [72, 76, 79, 83]);
}
