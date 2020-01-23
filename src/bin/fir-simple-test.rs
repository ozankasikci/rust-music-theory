use fantasy_in_rust::chord::{Chord, Quality as ChordQuality};
use fantasy_in_rust::note::{Note, PitchClass};
use fantasy_in_rust::scale::{Mode, Scale, ScaleType};
use std::io::{stdin, stdout, StdoutLock, Write};
use strum::IntoEnumIterator;
use termion::color::Color;
use termion::event::{Event, Key, MouseEvent};
use termion::input::{MouseTerminal, TermRead};
use termion::raw::IntoRawMode;
use termion::{clear, color, style};

fn print_color<T: Color>(stdout: &mut StdoutLock, text: String, color_arg: color::Fg<T>) {
    stdout
        .write_all(
            format!(
                "{color}{}{reset}\n",
                text,
                color = color_arg,
                reset = color::Fg(color::Reset),
            )
            .as_bytes(),
        )
        .unwrap();
}

fn main() {
    let stdout = stdout();
    let mut stdout = stdout.lock();

    let stdin = stdin();
    let mut stdin = stdin.lock();

    print_color(
        &mut stdout,
        "\nEnter a number to select the scale type:".to_string(),
        color::Fg(color::Green),
    );
    let mut index = 1;
    for value in ScaleType::iter() {
        println!("{}) {:?}", index, value);
        index += 1;
    }
    let scale_choice_u8 = stdin.read_line().unwrap().unwrap().parse::<u8>().unwrap();
    let scale_choice = ScaleType::from_u8(scale_choice_u8);

    print_color(
        &mut stdout,
        "\nEnter a number to select the mode:".to_string(),
        color::Fg(color::Green),
    );
    let mut index = 1;
    for value in Mode::iter() {
        println!("{}) {:?}", index, value);
        index += 1;
    }
    let mode_choice_u8 = stdin.read_line().unwrap().unwrap().parse::<u8>().unwrap();
    let mode_choice = Mode::from_u8(mode_choice_u8);

    print_color(
        &mut stdout,
        "\nEnter a number to select the pitch:".to_string(),
        color::Fg(color::Green),
    );
    let mut index = 1;
    for value in PitchClass::iter() {
        println!("{}) {:?}", index, value);
        index += 1;
    }
    let pitch_choice_u8 = stdin.read_line().unwrap().unwrap().parse::<u8>().unwrap();
    let pitch_choice = PitchClass::from_u8(pitch_choice_u8);

    let scale = Scale::new(
        ScaleType::from_u8(scale_choice_u8),
        PitchClass::from_u8(pitch_choice_u8),
        4,
        Some(mode_choice),
    )
    .unwrap();

    let notes = scale.notes();

    print_color(
        &mut stdout,
        format!(
            "\nThe notes for {} {} {} scale",
            pitch_choice,
            scale_choice,
            scale.mode.unwrap()
        ),
        color::Fg(color::Green),
    );
    for (i, note) in notes.iter().enumerate() {
        print!("{}-", i + 1);
        print_color(
            &mut stdout,
            format!("{}  ", note.pitch_class),
            color::Fg(color::Red),
        );
    }
    println!("");
    println!("");
}
