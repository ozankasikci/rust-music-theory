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
}
