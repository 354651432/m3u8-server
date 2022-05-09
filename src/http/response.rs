use core::panic;
use std::{
    collections::HashMap,
    fmt::Display,
    io::{BufRead, BufReader, Read},
};

use serde::Serialize;

use super::{getkey_ignorecase, header_tostr, parse_headers, parse_resline, Headers, ResLine};

#[cfg(test)]
#[path = "response_test.rs"]
mod response_test;

#[derive(Debug)]
pub struct Response {
    pub res: ResLine,
    pub headers: Headers,
    pub body: Vec<u8>,
}

impl Response {
    pub fn new(res: ResLine, headers: Headers, body: Vec<u8>) -> Self {
        Self { res, headers, body }
    }

    pub fn from_stream(stream: impl Read) -> Option<Self> {
        let mut reader = BufReader::new(stream);

        // 去掉开头的空行
        let mut line = String::new();

        let mut cnt = 0;
        while line.len() <= 0 && cnt < 100 {
            reader.read_line(&mut line).ok()?;
            cnt += 1
        }

        let res = parse_resline(&line)?;

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
        } else {
            reader.read_to_end(&mut body);
        }

        Some(Self { res, headers, body })
    }

    pub fn body_str(&self) -> String {
        String::from_utf8_lossy(&self.body).to_string()
    }

    pub fn header<T, U>(&mut self, key: T, value: U) -> Option<String>
    where
        String: From<T>,
        String: From<U>,
    {
        self.headers.insert(String::from(key), String::from(value))
    }

    pub fn body(&mut self, body: &str) {
        self.body = body.as_bytes().to_vec();
    }

    pub fn body_json<T>(&mut self, obj: T)
    where
        T: Serialize,
    {
        let str1 = serde_json::to_string(&obj).unwrap();
        self.header("Content-Type", "text/json");
        self.header("Content-Length", str1.len().to_string());
        self.body = str1.as_bytes().to_vec();
    }
}

impl Display for Response {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}\r\n", self.res)?;
        if self.headers.len() > 0 {
            write!(f, "{}\r\n", header_tostr(&self.headers))?;
        }
        write!(f, "\r\n{}", self.body_str())
    }
}
