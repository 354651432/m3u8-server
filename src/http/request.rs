#![allow(warnings, unused)]

use std::{
    collections::HashMap,
    io::{BufRead, BufReader, Read},
};

use serde::Deserialize;

#[cfg(test)]
#[path = "request_test.rs"]
mod tests;

#[derive(Debug)]
pub struct Request {
    pub version: String,
    pub path: String,
    pub method: String,
    pub headers: HashMap<String, String>,
    pub body: Option<String>,
}

impl Request {
    // 这个时候 谁拥有stream是有由主调方决定 可以传值或者引用
    pub fn read_from_stream(stream: impl Read) -> Option<Request> {
        let mut reader = BufReader::new(stream);

        let mut buf = String::new();

        while buf.len() <= 0 {
            reader.read_line(&mut buf).ok()?;
        }

        let (method, path, version) = Self::parse_status_line(buf)?;

        let mut header_str = String::new();
        let mut cnt = 0;
        while cnt < 1024 {
            let mut buf = String::new();
            reader.read_line(&mut buf).ok()?;
            header_str += &buf;
            header_str += "\r\n";
            if header_str.ends_with("\r\n\r\n") {
                break;
            }
            cnt += 1;
        }
        let mut headers: HashMap<String, String> = HashMap::new();
        for line in header_str.split("\r\n") {
            let mut it = line.split(":");
            let key = it.next();
            if key.is_none() {
                continue;
            }

            let value = it.next();
            if value.is_none() {
                continue;
            }
            headers.insert(key?.trim().to_owned(), value?.trim().to_owned());
        }

        let mut body = None;
        if let Some(content_length) = headers.get("Content-Length") {
            let content_length: Result<usize, _> = content_length.parse();
            if let Ok(content_length) = content_length {
                let mut buf = Vec::with_capacity(content_length);
                reader.read(&mut buf).ok()?;
                body = Some(String::from_utf8_lossy(&buf).to_string());
            }
        }

        Some(Request {
            method,
            path,
            version,
            headers,
            body,
        })
    }

    fn parse_status_line(buf: String) -> Option<(String, String, String)> {
        let mut it = buf.split_whitespace();
        let method = it.next()?.to_uppercase();
        let path = it.next()?;
        let version = it.next()?;
        Some((method, path.to_owned(), version.to_owned()))
    }

    pub fn json<'a, T>(&'a self) -> Option<T>
    where
        T: Deserialize<'a>,
    {
        let ret: T = serde_json::from_str(self.body.as_ref()?).ok()?;
        Some(ret)
    }
}
