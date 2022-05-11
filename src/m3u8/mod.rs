use std::fmt::Write;

use regex::*;

#[cfg(test)]
mod test;

pub mod download;

pub fn parse(url: &str, content: &str) -> Vec<String> {
    let url = url.split("?").next().unwrap();
    let url = Regex::new(r"[^/]+\.m3u8.*$")
        .unwrap()
        .replace(url, "")
        .to_string();

    let mut ret = Vec::new();
    for line in content.lines() {
        if line.starts_with("#") {
            continue;
        }
        if line.trim().is_empty() {
            continue;
        }
        ret.push(format!("{}{}", &url, line));
    }
    ret
}

pub fn gen_file_name(url: &str) -> String {
    let md5 = md5::compute(&url);
    let mut filename = String::new();
    for c in md5.0 {
        write!(&mut filename, "{:x}", c);
    }
    filename + ".ts"
}
