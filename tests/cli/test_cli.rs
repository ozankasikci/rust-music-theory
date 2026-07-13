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
    assert!(stdout(&scales).contains("Phrygian Dominant"));
    assert!(stdout(&scales).contains("Lydian Augmented"));
    assert!(stdout(&scales).contains("Altered"));

    let chords = rustmt(&["chord", "list"]);
    assert!(chords.status.success());
    assert!(stdout(&chords).contains("Ninths: C9, Cmaj9, Cm9, CmMaj9"));
    assert!(stdout(&chords).contains("Altered dominant: C7alt"));
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
    assert_eq!(stdout(&chord), "Notes:\n  1: F#\n  2: C\n  3: E\n  4: G\n");
}

#[test]
fn rejects_invalid_music_input() {
    assert!(!rustmt(&["chord", "C", "garbage"]).status.success());
    assert!(!rustmt(&["scale", "C", "garbage"]).status.success());
}

#[test]
fn prints_complete_minor_modes_and_aliases() {
    let phrygian_dominant = rustmt(&["scale", "C", "phrygian", "dominant"]);
    assert!(phrygian_dominant.status.success());
    assert_eq!(
        stdout(&phrygian_dominant),
        "Notes:\n  1: C\n  2: Db\n  3: E\n  4: F\n  5: G\n  6: Ab\n  7: Bb\n  8: C\n"
    );

    let altered = rustmt(&["scale", "C", "super", "locrian", "--descending"]);
    assert!(altered.status.success());
    assert_eq!(
        stdout(&altered),
        "Notes:\n  1: C\n  2: Bb\n  3: Ab\n  4: Gb\n  5: Fb\n  6: Eb\n  7: Db\n  8: C\n"
    );

    let unicode = rustmt(&["scale", "C", "dorian", "♭2"]);
    assert!(unicode.status.success());

    let invalid = rustmt(&["scale", "C", "not-a-mode"]);
    assert!(!invalid.status.success());
    let stderr = String::from_utf8(invalid.stderr).unwrap();
    assert!(stderr.contains("Can't determine the mode"));
    assert!(!stderr.contains("panicked"));
}

#[test]
fn accepts_compact_symbols_and_normalizes_aliases() {
    let chord = rustmt(&["chord", "C7sus4"]);
    assert!(chord.status.success());
    assert_eq!(stdout(&chord), "Notes:\n  1: C\n  2: F\n  3: G\n  4: Bb\n");

    let normalized = rustmt(&["chord", "normalize", "C7(b9,#11)"]);
    assert!(normalized.status.success());
    assert_eq!(stdout(&normalized), "C7b9#11\n");

    let invalid = rustmt(&["chord", "normalize", "C7altb9"]);
    assert!(!invalid.status.success());
    assert!(String::from_utf8(invalid.stderr)
        .unwrap()
        .contains("Conflicting chord modifiers"));

    let composed_alias = rustmt(&["chord", "normalize", "C+M7"]);
    assert!(composed_alias.status.success());
    assert_eq!(stdout(&composed_alias), "CaugMaj7\n");

    let malformed_group = rustmt(&["chord", "normalize", "C7(b9,,#11)"]);
    assert!(!malformed_group.status.success());
    assert!(String::from_utf8(malformed_group.stderr)
        .unwrap()
        .contains("Unexpected token"));
}

#[test]
fn second_audit_regressions_match_the_rust_parser() {
    let unicode = rustmt(&["chord", "normalize", "C𝄫maj7"]);
    assert!(unicode.status.success());
    assert_eq!(stdout(&unicode), "Cbbmaj7\n");

    for invalid in ["C5add9", "CmMaj6", "Cdom6"] {
        let result = rustmt(&["chord", "normalize", invalid]);
        assert!(!result.status.success(), "{} should fail", invalid);
        assert!(String::from_utf8(result.stderr).unwrap().contains("error:"));
    }
}
