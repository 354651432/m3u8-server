#![allow(warnings, unused)]

use std::{
    collections::HashMap,
    fmt::Display,
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
    pub body: Vec<u8>,

    // 反序列化用的字段
    body_str: String,
}

impl Request {
    pub fn new() -> Self {
        Self {
            version: String::from("HTTP/1.1"),
            path: String::from("/"),
            method: String::from("GET"),
            headers: Default::default(),
            body: Default::default(),
            body_str: Default::default(),
        }
    }

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

        let mut body = Vec::new();
        if let Some(content_length) = headers.get("Content-Length") {
            let content_length: Result<usize, _> = content_length.parse();
            if let Ok(content_length) = content_length {
                body = Vec::with_capacity(content_length);
                reader.read(&mut body).ok()?;
            }
        }

        Some(Request {
            method,
            path,
            version,
            headers,
            body,

            body_str: Default::default(),
        })
    }

    pub fn body_str(&self) -> String {
        String::from_utf8_lossy(&self.body).to_string()
    }

    // pub fn body_str_ref(&self) -> &str {
    //     &String::from_utf8_lossy(&self.body).to_string()
    // }

    fn parse_status_line(buf: String) -> Option<(String, String, String)> {
        let mut it = buf.split_whitespace();
        let method = it.next()?.to_uppercase();
        let path = it.next()?;
        let version = it.next()?;
        Some((method, path.to_owned(), version.to_owned()))
    }

    pub fn json<'b, T>(&'b mut self) -> Option<T>
    where
        T: Deserialize<'b>,
    {
        self.body_str = self.body_str();
        let ret: T = serde_json::from_str(&self.body_str).ok()?;
        Some(ret)
    }

    pub fn header(&mut self, key: &str, value: &str) -> Option<String> {
        self.headers.insert(key.to_owned(), value.to_owned())
    }
}

impl Display for Request {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {} {}\r\n", self.method, self.path, self.version)?;

        let mut header_str = String::new();
        for (key, value) in &self.headers {
            write!(f, "{}: {}\r\n", key, value)?;
        }
        write!(f, "\r\n")?;

        write!(f, "{}", self.body_str());
        Ok(())
    }
}
