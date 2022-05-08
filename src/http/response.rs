use std::{collections::HashMap, fmt::Display};

use serde::Serialize;

#[cfg(test)]
#[path = "response_test.rs"]
mod response_test;

#[derive(Debug)]
pub struct Response {
    version: String,
    status: String,
    headers: HashMap<String, String>,
    body: Option<String>,
}

impl Default for Response {
    fn default() -> Self {
        Self {
            version: String::from("HTTP/1.1"),
            status: String::from("200 OK"),
            headers: HashMap::new(),
            body: None,
        }
    }
}

impl Display for Response {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let body = self.body.as_ref();

        let mut headers: Vec<String> = self
            .headers
            .iter()
            .map(|(key, value)| format!("{}: {}", key, value))
            .collect();

        if body.is_some() && self.headers.get("Content-Type").is_none() {
            headers.push(String::from("Content-Type: text/html"));
        }

        if body.is_some() && self.headers.get("Content-Length").is_none() {
            headers.push(format!("Content-Length: {}", body.as_ref().unwrap().len()));
        }

        let body = if self.body.is_none() {
            "".to_owned()
        } else {
            self.body.as_ref().unwrap().to_owned()
        };

        write!(
            f,
            "{} {}\r\n{}\r\n\r\n{}",
            self.version,
            self.status,
            &headers[..].join("\r\n"),
            body
        )
    }
}

impl Response {
    pub fn new() {
        Default::default()
    }

    pub fn header<T, U>(&mut self, key: T, value: U) -> Option<String>
    where
        String: From<T>,
        String: From<U>,
    {
        self.headers.insert(String::from(key), String::from(value))
    }

    pub fn body(&mut self, body: &str) {
        self.body = Some(String::from(body));
    }

    pub fn body_json<T>(&mut self, obj: T)
    where
        T: Serialize,
    {
        let str1 = serde_json::to_string(&obj).unwrap();
        self.header("Content-Type", "text/json");
        self.header("Content-Length", str1.len().to_string());
        self.body = Some(str1);
    }
}
