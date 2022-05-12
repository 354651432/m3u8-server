use clap::Parser;

#[derive(Parser, Debug)]
#[clap(version, about = "threadify m3u8 link downlder")]
struct M3u8option {
    #[clap(long = "stdin")]
    stdin: bool,
}
fn main() {
    let m3u_opt = M3u8option::parse();
    println!("{:#?}", m3u_opt)
}
