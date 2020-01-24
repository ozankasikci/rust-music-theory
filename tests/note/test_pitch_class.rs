extern crate rust_music_theory as theory;
use theory::note::{PitchClass, PitchClass::*};

#[cfg(test)]
mod test_note {
    use super::*;

    #[test]
    fn test_pitch_class_from_str() {
       let table = vec![
           ("As", As),
           ("Ab", Gs),
           ("Cb", B),
           ("C#", Cs),
           ("C#", Cs),
           ("C♯", Cs),
       ];

        for (string, pitch_class) in table {
           let p = PitchClass::from_str(string).unwrap();
           assert_eq!(p, pitch_class);
        }
    }
}
