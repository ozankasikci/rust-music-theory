extern crate rust_music_theory as theory;
use theory::note::{PitchClass, PitchClass::*};

#[cfg(test)]
mod test_note {
    use super::*;

    #[test]
    fn test_pitch_class_from_str() {
       let table = vec![
           ("Cb", B),
           ("C#", Cs),
           ("C#", Cs),
           ("C♯", Cs),
           ("D", D),
           ("Db", Cs),
           ("Ds", Ds),
           ("E", E),
           ("Es", F),
           ("Eb", Ds),
           ("F", F),
           ("f", F),
           ("Fb", E),
           ("G", G),
           ("Gb", Fs),
           ("Gs", Gs),
           ("A", A),
           ("As", As),
           ("Ab", Gs),
           ("B", B),
           ("B♯", C),
           ("Bb", As),
       ];

        for (string, pitch_class) in table {
           let p = PitchClass::from_str(string).unwrap();
           assert_eq!(p, pitch_class);
        }
    }
}
