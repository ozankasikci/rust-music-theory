use structopt::StructOpt;

mod chord;
mod scale;

fn main() {
    let opt = Cli::from_args();
    opt.execute();
}

#[derive(StructOpt, Debug)]
#[structopt(name = "rust-music-theory", author, about = "A music theory guide")]
enum Cli {
    Scale(scale::Command),
    Chord(chord::Command),
}

impl Cli {
    fn execute(self) {
        match self {
            Cli::Scale(scale_command) => {
                scale_command.execute();
            }
            Cli::Chord(chord_command) => {
                chord_command.execute();
            }
        }
    }
}
