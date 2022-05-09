use std::{collections::HashMap, fmt::Display};

pub mod http_client;
pub mod http_listener;
pub mod request;
pub mod response;
#[cfg(test)]
mod test;

type Headers = HashMap<String, String>;
pub fn parse_headers(lines: &Vec<String>) -> Headers {
    let mut ret = HashMap::new();
    for line in lines {
        if !line.contains(":") {
            continue;
        }
        let mut iter = line.split(":");
        let key = iter.next().unwrap();
        let key = key.trim().trim();
        let value = iter.next().unwrap().trim();
        ret.insert(String::from(key), String::from(value));
    }

    ret
}

pub fn header_tostr(headers: &Headers) -> String {
    let mut ret = String::new();
    for (key, value) in headers {
        ret.push_str(format!("{}: {}\r\n", key, value).as_str());
    }
    ret
}

#[derive(Debug)]
pub struct ReqLine {
    pub method: String,
    pub path: String,
    pub version: String,
}

impl Display for ReqLine {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {} {}", self.method, self.path, self.version)
    }
}

pub fn parse_reqline(line: &str) -> Option<ReqLine> {
    let mut iter = line.split_whitespace();
    let method = iter.next()?.to_uppercase();
    let path = iter.next()?.to_string();
    let version = iter.next()?.to_string();

    Some(ReqLine {
        method,
        path,
        version,
    })
}

#[derive(Debug)]
pub struct ResLine {
    pub version: String,
    pub code: usize,
    pub code_line: String,
}

impl Display for ResLine {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {} {}", self.version, self.code, self.code_line)
    }
}

pub fn parse_resline(line: &str) -> Option<ResLine> {
    let mut iter = line.split_whitespace();
    let version = iter.next()?.to_string();
    let code = iter.next()?.parse().ok()?;
    let code_line = iter.next()?.to_string();

    Some(ResLine {
        version,
        code,
        code_line,
    })
}

pub fn getkey_ignorecase<'a>(key: &str, map: &'a Headers) -> Option<&'a str> {
    for (key1, value) in map {
        if key.to_lowercase() == key1.to_lowercase() {
            return Some(value);
        }
    }
    None
}
