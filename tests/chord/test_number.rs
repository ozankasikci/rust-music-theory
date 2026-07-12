extern crate rust_music_theory as theory;
use theory::chord::Number;

#[cfg(test)]
mod chord_number_tests {
    use super::*;

    #[test]
    fn test_number_seventh() {
        let string = "seventh";
        let (number, _) = Number::from_regex(string).unwrap();
        assert_eq!(Number::Seventh, number);
    }

    #[test]
    fn test_number_triad() {
        let string = "triad";
        let (number, _) = Number::from_regex(string).unwrap();
        assert_eq!(Number::Triad, number);
    }

    #[test]
    fn test_number_aliases_and_major_seventh_precedence() {
        let cases = [
            ("7", Number::Seventh),
            ("maj7", Number::MajorSeventh),
            ("major seventh", Number::MajorSeventh),
            ("9", Number::Ninth),
            ("11", Number::Eleventh),
            ("13", Number::Thirteenth),
        ];

        for (symbol, expected) in cases {
            assert_eq!(Number::from_regex(symbol).unwrap().0, expected);
        }
    }
}
