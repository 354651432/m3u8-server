use std::{
    collections::HashMap,
    fmt::Display,
    io::{BufRead, BufReader, Read},
};

use serde::Serialize;

#[cfg(test)]
#[path = "response_test.rs"]
mod response_test;

#[derive(Debug)]
pub struct Response {
    pub version: String,
    pub status: String,
    pub headers: HashMap<String, String>,
    pub body: Vec<u8>,
}

impl Default for Response {
    fn default() -> Self {
        Self {
            version: String::from("HTTP/1.1"),
            status: String::from("200 OK"),
            headers: HashMap::new(),
            body: Vec::new(),
        }
    }
}

impl Display for Response {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}\r\n", self.version, self.status)?;

        for (key, value) in &self.headers {
            write!(f, "{}: {}\r\n", key, value)?;
        }

        if self.body.len() > 0 && self.headers.get("Content-Type").is_none() {
            write!(f, "Content-Type: text/html\r\n")?;
        }

        if self.body.len() > 0 && self.headers.get("Content-Length").is_none() {
            write!(f, "Content-Length: {}\r\n", self.body.len())?;
        }

        write!(f, "\r\n{}", self.body_str())
    }
}

impl Response {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn from_stream(socket: impl Read) -> Option<Self> {
        let mut ret = Self::new();
        let mut reader = BufReader::new(socket);
        reader.read_line(&mut ret.status).ok()?;
        while ret.status.len() <= 0 {
            reader.read_line(&mut ret.status).ok()?;
        }

        loop {
            let mut buf = String::new();
            reader.read_line(&mut buf).ok()?;
            if buf.contains(":") {
                let mut arr = buf.split(":");
                ret.header(arr.next()?.trim(), arr.next()?.trim());
            }
            if buf.trim() == "" {
                break;
            }
        }

        if let Some(clen) = ret.headers.get("Content-Length") {
            let clen = clen.trim().parse().ok()?;
            ret.body = Vec::with_capacity(clen);
            reader.read(&mut ret.body).ok()?;
        }

        Some(ret)
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
