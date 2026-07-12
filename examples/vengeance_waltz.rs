//! An original, melancholic waltz for two hands, in the spirit of the string
//! waltzes heard in Korean revenge-cinema scores.
//!
//! The piece is built on a single "leap and sigh" motif — a pickup that leaps
//! up a minor sixth, sighs down a step, and settles (A4 → F5–E5–D5). The motif
//! is stated, ornamented, sequenced to its peak in octaves over an Andalusian
//! bass, denied by a deceptive cadence, and finally whispered over a chromatic
//! lament bass.
//!
//! It borrows Chopin's salon vocabulary: crushed grace notes, a nocturne-style
//! left hand that leaps to a bass then sweeps up through a wide chord, and
//! generous tempo rubato. Rolled chords and small timing nudges keep it from
//! sounding sequenced. It is an original composition rather than a transcription
//! of any film score.
//!
//! Run with:
//!   cargo run --example vengeance_waltz --features midi
//!
//! An optional first argument changes the output path:
//!   cargo run --example vengeance_waltz --features midi -- /tmp/vengeance_waltz.mid

use std::error::Error;

use rust_music_theory::midi::{Channel, Duration, MidiBuilder, MidiFile, Velocity};
use rust_music_theory::note::{Note, Notes, Pitch};

const PPQ: u16 = 480;
const TICKS_PER_BEAT: u32 = PPQ as u32;
const TICKS_PER_BAR: u32 = TICKS_PER_BEAT * 3;
const BARS: usize = 37;

/// How far ahead of the beat a grace note is crushed, in ticks.
const GRACE_LEAD: u32 = 70;
/// Tick spread between the tones of a rolled chord.
const ROLL_SPREAD: u32 = 18;

/// A small adapter that lets `MidiBuilder` place MIDI pitches directly.
struct Voicing(Vec<u8>);

impl Notes for Voicing {
    fn notes(&self) -> Vec<Note> {
        self.0
            .iter()
            .map(|&midi_pitch| {
                let pitch = Pitch::from_u8(midi_pitch % 12);
                let octave = midi_pitch / 12 - 1;
                Note::new(pitch, octave)
            })
            .collect()
    }
}

#[derive(Clone, Copy)]
struct MelodyNote {
    /// Position in eighth notes from the start of the bar (0..6 in 3/4).
    eighth: u8,
    /// Length in eighth notes.
    length: u8,
    pitch: u8,
    velocity: u8,
    /// Grace note crushed just before the main note (0 = none).
    grace: u8,
    /// Double the melody an octave below, for the climax.
    octave_below: bool,
}

const fn n(eighth: u8, length: u8, pitch: u8, velocity: u8) -> MelodyNote {
    MelodyNote {
        eighth,
        length,
        pitch,
        velocity,
        grace: 0,
        octave_below: false,
    }
}

impl MelodyNote {
    const fn g(mut self, grace: u8) -> Self {
        self.grace = grace;
        self
    }

    const fn oct(mut self) -> Self {
        self.octave_below = true;
        self
    }
}

/// The right-hand line. Each inner slice is one 3/4 bar, measured in eighths.
/// MIDI pitches are used here to keep the composition easy to reshape.
const MELODY: [&[MelodyNote]; BARS] = [
    // Intro: the left hand waltzes alone, then the melody breathes in.
    &[],
    &[],
    &[n(0, 4, 69, 46)],
    &[n(0, 4, 73, 50), n(5, 1, 69, 52)], // ...with the pickup that launches the motif.
    // A: the leap-and-sigh motif, stated plainly.
    &[n(0, 3, 77, 66), n(3, 1, 76, 60), n(4, 2, 74, 58)], // F5-E5-D5: the motif.
    &[n(0, 4, 70, 60), n(4, 2, 69, 56)],                  // Bb-A: the sigh, echoed.
    &[n(0, 3, 73, 62), n(3, 1, 74, 64), n(4, 2, 76, 66)],
    &[n(0, 4, 74, 62), n(5, 1, 69, 58)],
    &[n(0, 3, 77, 70), n(3, 1, 76, 63), n(4, 2, 74, 60)],
    &[n(0, 3, 70, 62), n(3, 1, 72, 64), n(4, 2, 74, 66)],
    &[n(0, 2, 76, 68), n(2, 2, 74, 63), n(4, 2, 70, 60)],
    &[n(0, 4, 73, 60), n(5, 1, 69, 58)],
    // A': the motif ornamented with grace notes, then sequenced upward.
    &[n(0, 3, 77, 70).g(79), n(3, 1, 76, 64), n(4, 2, 74, 62)],
    &[n(0, 3, 70, 64).g(72), n(3, 1, 69, 58), n(4, 2, 67, 56)],
    &[n(0, 3, 73, 66).g(74), n(3, 1, 74, 68), n(4, 2, 76, 70)],
    &[n(0, 3, 74, 64), n(3, 1, 77, 68), n(4, 2, 81, 72)],
    &[n(0, 3, 81, 76).g(82), n(3, 1, 79, 68), n(4, 2, 77, 66)], // The motif, a fifth higher.
    &[n(0, 3, 79, 72).g(81), n(3, 1, 77, 66), n(4, 2, 76, 64)],
    &[n(0, 2, 76, 68), n(2, 2, 79, 72), n(4, 2, 82, 76)],
    &[n(0, 3, 81, 76), n(3, 1, 79, 70), n(4, 1, 76, 66), n(5, 1, 81, 72)],
    // B: the climax — octaves over an Andalusian descent, then the deception.
    &[n(0, 2, 86, 84).oct(), n(2, 2, 88, 88).oct(), n(4, 2, 89, 92).oct()],
    &[n(0, 4, 89, 94).oct(), n(4, 2, 88, 86).oct()],
    &[n(0, 3, 86, 88).oct(), n(3, 1, 84, 80).oct(), n(4, 2, 82, 78).oct()],
    &[n(0, 4, 85, 84).oct(), n(4, 2, 88, 88).oct()],
    &[n(0, 3, 89, 92).oct(), n(3, 1, 88, 84).oct(), n(4, 2, 86, 82).oct()], // Motif, fortissimo.
    &[
        n(0, 2, 88, 92).oct(),
        n(2, 2, 89, 104).oct(), // The peak: F6 against the b9 of A7.
        n(4, 1, 88, 90).oct(),
        n(5, 1, 85, 84).oct(),
    ],
    &[n(0, 4, 86, 88).oct(), n(4, 2, 84, 78).oct()], // Deceptive cadence: V lands on Bb.
    &[n(0, 2, 82, 74), n(2, 2, 81, 70), n(4, 1, 79, 64), n(5, 1, 69, 56)],
    // Coda: the motif whispers over the lament bass, then dissolves.
    &[n(0, 3, 77, 54), n(3, 1, 76, 50), n(4, 2, 74, 48)],
    &[n(0, 4, 74, 50), n(4, 2, 73, 46)],
    &[n(0, 4, 77, 48), n(4, 2, 76, 45)],
    &[n(0, 6, 74, 46)],
    &[n(0, 4, 70, 48), n(4, 2, 69, 44)], // The sigh, one last time.
    &[n(0, 6, 69, 42)],
    &[n(0, 3, 77, 45), n(3, 1, 76, 42), n(4, 2, 74, 40)],
    &[n(0, 4, 76, 40), n(4, 2, 73, 37)],
    &[n(0, 6, 74, 40)],
];

/// Bass note and the three upper notes used by the left-hand waltz in each bar.
const HARMONY: [(u8, [u8; 3]); BARS] = [
    // Intro: tonic pedal under shifting colors.
    (38, [53, 57, 62]), // Dm
    (38, [53, 57, 62]),
    (38, [55, 58, 64]), // Gm6/D
    (37, [52, 55, 61]), // A7/C#
    // A
    (38, [53, 57, 62]), // Dm
    (43, [55, 58, 64]), // Gm6
    (45, [52, 58, 61]), // A7b9
    (38, [53, 57, 62]), // Dm
    (38, [53, 57, 62]),
    (43, [55, 58, 64]),
    (40, [50, 55, 58]), // Em7b5
    (45, [52, 55, 61]), // A7
    // A'
    (38, [53, 57, 62]),
    (43, [55, 58, 64]),
    (45, [52, 58, 61]),
    (38, [53, 57, 62]),
    (38, [53, 57, 62]),
    (43, [55, 58, 64]),
    (40, [50, 55, 58]),
    (45, [52, 55, 61]),
    // B: Andalusian descent (D-C-Bb-A), then the deceptive turn.
    (38, [53, 57, 62]), // Dm
    (36, [53, 57, 62]), // Dm/C
    (46, [53, 58, 62]), // Bb
    (45, [52, 55, 61]), // A7
    (43, [50, 55, 58]), // Gm
    (45, [52, 58, 61]), // A7b9
    (46, [53, 58, 62]), // Bb  <- deceptive cadence
    (45, [52, 55, 61]), // A7
    // Coda: chromatic lament bass, D-C#-C-B-Bb-A.
    (38, [53, 57, 62]), // Dm
    (37, [53, 57, 61]), // DmMaj7/C#
    (36, [53, 57, 62]), // Dm/C
    (35, [50, 55, 59]), // G/B
    (34, [50, 55, 58]), // Gm/Bb
    (33, [52, 55, 61]), // A7
    (38, [53, 57, 62]), // Dm
    (45, [52, 55, 61]), // A7
    (38, [53, 57, 62]), // Dm (final chord is voiced specially below)
];

fn place_at_tick(track: &mut MidiBuilder, tick: u32, length_ticks: u32, pitches: &[u8], velocity: u8) {
    track.at_tick(tick).add(
        &Voicing(pitches.to_vec()),
        Duration::Ticks(length_ticks),
        Velocity::new(velocity).expect("composition velocities are valid MIDI values"),
    );
}

/// Arpeggiate a chord bottom-to-top, the way a pianist rolls it.
fn roll(track: &mut MidiBuilder, tick: u32, length_ticks: u32, pitches: &[u8], velocity: u8, spread: u32) {
    for (i, &pitch) in pitches.iter().enumerate() {
        let offset = i as u32 * spread;
        place_at_tick(
            track,
            tick + offset,
            length_ticks.saturating_sub(offset).max(60),
            &[pitch],
            velocity,
        );
    }
}

fn build_right_hand() -> MidiBuilder {
    let mut track = MidiBuilder::with_ppq(PPQ);

    for (bar, notes) in MELODY.iter().enumerate() {
        let bar_start = bar as u32 * TICKS_PER_BAR;
        for note in *notes {
            // Rubato: lean into downbeats a touch late, push pickups a touch early.
            let nudge: i32 = match (note.eighth, note.length) {
                (0, _) => 12,
                (5, 1) => -12,
                _ => 0,
            };
            let tick = (bar_start + note.eighth as u32 * (TICKS_PER_BEAT / 2)).saturating_add_signed(nudge);
            let length = note.length as u32 * (TICKS_PER_BEAT / 2) * 98 / 100;

            if note.grace != 0 {
                let grace_tick = tick.saturating_sub(GRACE_LEAD);
                place_at_tick(
                    &mut track,
                    grace_tick,
                    60,
                    &[note.grace],
                    note.velocity.saturating_sub(12),
                );
            }

            if note.octave_below {
                place_at_tick(&mut track, tick, length, &[note.pitch - 12], note.velocity.saturating_sub(8));
            }
            place_at_tick(&mut track, tick, length, &[note.pitch], note.velocity);
        }
    }

    track
}

#[derive(Clone, Copy, PartialEq)]
enum Texture {
    /// Bare bass and a single mid-bar chord.
    Sparse,
    /// Classic bass-chord-chord waltz.
    Waltz,
    /// Octave bass and fuller rolled chords for the climax.
    Full,
}

fn build_left_hand() -> MidiBuilder {
    let mut track = MidiBuilder::with_ppq(PPQ);

    for (bar, &(bass, chord)) in HARMONY.iter().enumerate() {
        let bar_start = bar as u32 * TICKS_PER_BAR;
        let (texture, lift) = match bar {
            0..=3 => (Texture::Sparse, 0),
            4..=11 => (Texture::Waltz, 6),
            12..=19 => (Texture::Waltz, 9),
            20..=27 => (Texture::Full, 18),
            _ => (Texture::Sparse, 2),
        };
        // The coda dies away bar by bar.
        let fade = bar.saturating_sub(27) as u8 * 2;

        if bar == BARS - 1 {
            // A long, slow-rolled Dm(add9) left to ring.
            roll(
                &mut track,
                bar_start,
                TICKS_PER_BEAT * 5,
                &[38, 45, 50, 53, 57, 62, 64],
                44,
                60,
            );
            continue;
        }

        let bass_velocity = (50u8 + lift).saturating_sub(fade);
        let chord_velocity = (40u8 + lift).saturating_sub(fade);

        match texture {
            Texture::Sparse => {
                // Nocturne left hand: a low bass, then a wide upward sweep that
                // spans more than an octave, held under the singing melody.
                place_at_tick(&mut track, bar_start, TICKS_PER_BEAT * 29 / 10, &[bass], bass_velocity);
                let wide = [chord[0], chord[1], chord[2], chord[0] + 12];
                roll(
                    &mut track,
                    bar_start + TICKS_PER_BEAT,
                    TICKS_PER_BEAT * 18 / 10,
                    &wide,
                    chord_velocity,
                    ROLL_SPREAD + 10,
                );
            }
            Texture::Waltz => {
                place_at_tick(&mut track, bar_start, TICKS_PER_BEAT * 27 / 10, &[bass], bass_velocity);
                for beat in 1..=2 {
                    roll(
                        &mut track,
                        bar_start + beat * TICKS_PER_BEAT,
                        TICKS_PER_BEAT * 85 / 100,
                        &chord,
                        chord_velocity,
                        ROLL_SPREAD,
                    );
                }
            }
            Texture::Full => {
                place_at_tick(
                    &mut track,
                    bar_start,
                    TICKS_PER_BEAT * 27 / 10,
                    &[bass, bass + 12],
                    bass_velocity,
                );
                for beat in 1..=2 {
                    roll(
                        &mut track,
                        bar_start + beat * TICKS_PER_BEAT,
                        TICKS_PER_BEAT * 95 / 100,
                        &chord,
                        chord_velocity,
                        ROLL_SPREAD + 7,
                    );
                }
            }
        }
    }

    track
}

fn compose() -> MidiFile {
    MidiFile::new()
        .tempo(69)
        .time_signature(3, 4)
        .ppq(PPQ)
        .track(build_right_hand(), Channel::new(0).unwrap())
        .track(build_left_hand(), Channel::new(1).unwrap())
}

fn main() -> Result<(), Box<dyn Error>> {
    let output = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "vengeance_waltz.mid".to_string());

    compose().save(&output)?;

    println!("Wrote {output}");
    println!("37 bars, D minor, 3/4, 69 BPM, Chopin-flavored waltz");
    println!("Tracks: right hand (channel 1), left hand (channel 2)");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn composition_is_a_two_hand_midi_file() {
        let composition = compose();
        let bytes = composition.to_bytes();

        assert_eq!(composition.track_count(), 2);
        assert_eq!(&bytes[..4], b"MThd");
        // Tempo/meta track + right hand + left hand.
        assert_eq!(
            bytes.windows(4).filter(|window| *window == b"MTrk").count(),
            3
        );
    }

    #[test]
    fn every_bar_fits_inside_the_waltz_meter() {
        assert_eq!(MELODY.len(), BARS);
        assert_eq!(HARMONY.len(), BARS);
        assert!(MELODY
            .iter()
            .flat_map(|bar| bar.iter())
            .all(|note| note.eighth < 6 && note.eighth + note.length <= 6));
    }

    #[test]
    fn octave_doubling_and_graces_stay_in_midi_range() {
        assert!(MELODY
            .iter()
            .flat_map(|bar| bar.iter())
            .all(|note| (!note.octave_below || note.pitch >= 12) && note.velocity <= 127));
    }

}
