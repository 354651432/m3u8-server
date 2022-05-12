use clap::{CommandFactory, Parser};

#[derive(Parser, Debug)]
#[clap(version, about = "threadify m3u8 link downlder")]
struct M3u8option {
    #[clap(long = "stdin")]
    stdin: bool,
}
fn main() {
    M3u8option::command().print_help().unwrap();
}
