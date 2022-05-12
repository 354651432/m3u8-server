use clap::{Args, Parser, Subcommand};

#[derive(Parser, Debug)]
#[clap(
    version,
    about = "threadify m3u8 link downlder",
    arg_required_else_help = true
)]
pub struct Config {
    #[clap(
        long,
        help = "read copy as fetch from chrome dev network tool in stdin "
    )]
    pub stdin: bool,

    #[clap(
        long,
        short,
        default_value_t = 20,
        help = "number of threads default 20"
    )]
    pub threads: usize,

    #[clap(long, short, help = "start a webserver bind option eg: 127.0.0.1:2022")]
    pub bind: Option<String>,

    #[clap(long, short, help = "socks5 proxy eg: 127.0.0.1:10808")]
    pub proxy: Option<String>,

    #[clap(help = ".m3u8 download url")]
    pub url: Option<String>,

    #[clap(help = ".m3u8 saved file name")]
    pub file: Option<String>,
}
