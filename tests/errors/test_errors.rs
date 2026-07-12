use rust_music_theory::chord::ChordError;
use rust_music_theory::interval::IntervalError;
use rust_music_theory::note::NoteError;
use rust_music_theory::scale::ScaleError;
use std::error::Error;

#[test]
fn error_messages_are_specific_and_stable() {
    assert_eq!(ChordError::InvalidRegex.to_string(), "Invalid Regex!");
    assert_eq!(
        ChordError::UnknownIntervalPattern(vec![1, 2]).to_string(),
        "Unknown chord interval pattern: [1, 2]"
    );
    assert_eq!(
        ChordError::UnsupportedChord("Diminished Ninth".to_string()).to_string(),
        "Unsupported chord: Diminished Ninth"
    );
    assert_eq!(ChordError::InvalidInversion(4).to_string(), "Invalid chord inversion: 4");
    assert_eq!(IntervalError::InvalidInterval.to_string(), "Invalid interval!");
    assert_eq!(NoteError::InvalidPitch.to_string(), "Invalid Pitch Class!");
    assert_eq!(ScaleError::ModeFromRegex.to_string(), "Can't determine the mode!");
    assert_eq!(ScaleError::InvalidRegex.to_string(), "Invalid scale regex!");
    assert_eq!(
        ScaleError::InvalidInterval.to_string(),
        "Can't determine the intervals for the scale!"
    );
    assert!(ScaleError::InvalidRegex.source().is_none());
}

#[test]
fn conversion_errors_map_to_public_error_categories() {
    assert!(matches!(ChordError::from(NoteError::InvalidPitch), ChordError::InvalidRegex));
    assert!(matches!(ScaleError::from(NoteError::InvalidPitch), ScaleError::InvalidRegex));
    assert!(matches!(
        ScaleError::from(IntervalError::InvalidInterval),
        ScaleError::InvalidInterval
    ));

    let invalid_pattern = String::from("(");
    let regex_error = regex::Regex::new(&invalid_pattern).unwrap_err();
    assert!(matches!(
        ChordError::from(regex_error.clone()),
        ChordError::InvalidRegex
    ));
    assert!(matches!(
        NoteError::from(regex_error.clone()),
        NoteError::InvalidPitch
    ));
    assert!(matches!(
        ScaleError::from(regex_error),
        ScaleError::ModeFromRegex
    ));
}
