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
