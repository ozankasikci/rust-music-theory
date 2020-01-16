enum Quality {
    Perfect,
    Major,
    Minor,
    Augmented,
    Diminished,
}

enum Number {
    Unison,
    Second,
    Third,
    Fourth,
    Fifth,
    Sixth,
    Seventh,
    Octave,
}

enum Step {
    Half,
    Whole,
    Tritone,
}

pub struct Interval {
    pub semitone_count: i8,
    pub quality: Quality,
    pub number: Number,
    pub step: Step,
}

impl Interval {
    pub fn new_by_semitone_count(sc: i8) -> Self {
        let (mut number, mut quality) : (Number, Quality);
        match sc {
            0 => { number = Number::Unison; quality = Quality::Perfect; }
            1 => { number = Number::Unison }
            2 => { number = Number::Unison }
            3 => { number = Number::Unison }
            4 => { number = Number::Unison }
            5 => { number = Number::Unison }
            6 => { number = Number::Unison }
            7 => { number = Number::Unison }
            8 => { number = Number::Unison }
            9 => { number = Number::Unison }
            10 => { number = Number::Unison }
            11 => { number = Number::Unison }
            12 => { number = Number::Unison }
            _ => {}
        };

        Interval
    }
}