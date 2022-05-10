#![allow(unused)]
extern crate colorful;

use std::collections::HashMap;
use std::fmt::Write;
use std::time;
use std::time::Instant;

use colorful::Color;
use colorful::Colorful;
use work::config::get_config;
use work::download;

fn main() {
    let mut args = std::env::args();
    let name = args.next().unwrap();
    let (url, file_name) = match parse_args(&mut args) {
        Some(ret) => ret,
        None => {
            usage(name.as_str());
            return;
        }
    };

    let config = get_config();
    let time = Instant::now();
    match download::download(&url, &file_name, Some(&config.proxy), HashMap::default()) {
        Err(err) => {
            eprintln!("{}", err.light_yellow().bold());
            return;
        }
        Ok(size) => {
            let span = Instant::now() - time;

            let msg = format!(
                "complete downloaded {} size:{} secs:{}",
                file_name,
                size,
                span.as_secs(),
            );
            println!("{}", msg.light_green().bold())
        }
    }
}

fn usage(name: &str) {
    let vec;
    if name.contains("/") {
        vec = name.split("/");
    } else {
        vec = name.split("\\");
    }
    let name = vec.last().unwrap();
    println!(
        r"Usage: {name} [URL] [FILENAME]
        filename is optional,if none gen md5 as filename
    "
    )
}

fn parse_args(args: &mut std::env::Args) -> Option<(String, String)> {
    let url = match args.next() {
        Some(url) => url.clone(),
        None => return None,
    };

    let filename = match args.next() {
        Some(url) => url + ".ts",
        None => {
            let md5 = md5::compute(&url);
            let mut filename = String::new();
            for c in md5.0 {
                write!(&mut filename, "{:x}", c);
            }
            filename
        }
    };

    Some((url, filename))
}
