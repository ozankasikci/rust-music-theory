use std::process::{Command, Output};

fn rustmt(args: &[&str]) -> Output {
    Command::new(env!("CARGO_BIN_EXE_rustmt"))
        .args(args)
        .output()
        .expect("rustmt should execute")
}

fn stdout(output: &Output) -> String {
    String::from_utf8(output.stdout.clone()).unwrap()
}

#[test]
fn lists_every_supported_scale_and_chord() {
    let scales = rustmt(&["scale", "list"]);
    assert!(scales.status.success());
    assert!(stdout(&scales).contains("Whole Tone"));

    let chords = rustmt(&["chord", "list"]);
    assert!(chords.status.success());
    assert!(stdout(&chords).contains("Minor Ninth"));
    assert!(stdout(&chords).contains("Minor Thirteenth"));
}

#[test]
fn prints_reference_scales_and_chords() {
    let scale = rustmt(&["scale", "C", "melodic", "minor", "--descending"]);
    assert!(scale.status.success());
    assert_eq!(
        stdout(&scale),
        "Notes:\n  1: C\n  2: Bb\n  3: Ab\n  4: G\n  5: F\n  6: Eb\n  7: D\n  8: C\n"
    );

    let chord = rustmt(&["chord", "C", "augmented", "major", "seventh"]);
    assert!(chord.status.success());
    assert_eq!(stdout(&chord), "Notes:\n  1: C\n  2: E\n  3: G#\n  4: B\n");
}

#[test]
fn prints_non_chord_slash_bass_first() {
    let chord = rustmt(&["chord", "C/Fs"]);
    assert!(chord.status.success());
    assert_eq!(
        stdout(&chord),
        "Notes:\n  1: F#\n  2: C\n  3: E\n  4: G\n"
    );
}

#[test]
fn rejects_invalid_music_input() {
    assert!(!rustmt(&["chord", "C", "garbage"]).status.success());
    assert!(!rustmt(&["scale", "C", "garbage"]).status.success());
}
