#![allow(warnings, unused)]

use std::{
    collections::HashMap,
    fmt::Display,
    io::{BufRead, BufReader, Read},
};

use serde::Deserialize;

use super::{getkey_ignorecase, header_tostr, parse_headers, parse_reqline, Headers, ReqLine};

#[cfg(test)]
#[path = "request_test.rs"]
mod tests;

#[derive(Debug)]
pub struct Request {
    pub req: ReqLine,
    pub headers: Headers,
    pub body: Vec<u8>,
}

impl Request {
    pub fn new(req: ReqLine, headers: Headers, body: Vec<u8>) -> Self {
        Self { req, headers, body }
    }

    // 这个时候 谁拥有stream是有由主调方决定 可以传值或者引用
    pub fn from_stream(stream: impl Read) -> Option<Request> {
        let mut reader = BufReader::new(stream);

        // 去掉开头的空行
        let mut line = String::new();

        let mut cnt = 0;
        while line.len() <= 0 && cnt < 100 {
            reader.read_line(&mut line).ok()?;
            cnt += 1
        }

        let req = parse_reqline(&line)?;

        let mut lines = Vec::new();
        let mut cnt = 0;
        while cnt < 1024 {
            let mut buf = String::new();
            reader.read_line(&mut buf).ok()?;
            if buf.is_empty() {
                break;
            }
            lines.push(buf);
            cnt += 1;
        }

        let headers = parse_headers(&lines);

        let mut body = Vec::new();
        let content_length = getkey_ignorecase("content-length", &headers);

        if let Some(content_length) = content_length {
            if let Ok(capacity) = content_length.parse() {
                body = Vec::with_capacity(capacity);
                unsafe {
                    body.set_len(capacity);
                }
                reader.read(&mut body);
            }
        } else if (req.method == "POST" || req.method == "PUT") {
            reader.read_to_end(&mut body);
        }

        Some(Self { req, headers, body })
    }

    pub fn body_str(&self) -> String {
        String::from_utf8_lossy(&self.body).to_string()
    }

    // TODO
    // pub fn json<'de, T>(&'de mut self) -> Result<T, serde_json::Error>
    // where
    //     T: Deserialize<'de>,
    // {
    //     let rdr = BufReader::new(self.body_str().as_bytes());
    //     serde_json::from_reader(rdr)
    // }

    pub fn header(&mut self, key: &str, value: &str) -> Option<String> {
        self.headers.insert(key.to_owned(), value.to_owned())
    }
}

impl Display for Request {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}\r\n", self.req)?;
        if self.headers.len() > 0 {
            write!(f, "{}\r\n", header_tostr(&self.headers))?;
        }
        write!(f, "\r\n{}", self.body_str());
        Ok(())
    }
}
