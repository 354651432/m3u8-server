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
        while line.trim().is_empty() && cnt < 100 {
            reader.read_line(&mut line).ok()?;
            cnt += 1
        }

        let res = parse_resline(&line)?;

        let mut lines = Vec::new();
        let mut cnt = 0;
        while cnt < 1024 {
            let mut buf = String::new();
            reader.read_line(&mut buf).ok()?;
            if buf.trim().is_empty() {
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
                body = vec![0u8; capacity];
                reader.read_exact(&mut body);
            }
        } else if let Some(value) = getkey_ignorecase("Transfer-Encoding", &headers) {
            if value.to_lowercase() == "chunked" {
                body = Self::read_trunked_stream(reader);
            }
            // reader.read_to_end(&mut body);
        }

        Some(Self { res, headers, body })
    }

    fn read_trunked_stream(stream: impl Read) -> Vec<u8> {
        let mut buf = Vec::new();
        let mut reader = BufReader::new(stream);
        // loop {
        //     let mut line = String::new();
        //     reader.read_line(&mut line).unwrap();
        //     println!("{line}");
        //     if line.trim().is_empty() {
        //         break;
        //     }
        // }

        println!("begin read trunk");

        loop {
            let mut line = String::new();
            reader.read_line(&mut line).unwrap();

            let line = line.trim();
            if line == "0" || line.is_empty() {
                break;
            }

            println!("line {line}");
            let length = usize::from_str_radix(line, 16).unwrap();
            let mut local_buf = vec![0u8; length];

            reader.read_exact(&mut local_buf).unwrap();
            for i in local_buf {
                buf.push(i);
            }

            // 读数据结束的\r\n
            let mut buf = vec![0u8; 2];
            reader.read_exact(&mut buf).unwrap();
        }

        buf
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

    pub fn success_res() -> Self {
        Self {
            res: ResLine {
                version: "HTTP/1.0".to_string(),
                code: 200,
                code_line: "OK".to_string(),
            },
            headers: HashMap::new(),
            body: Vec::new(),
        }
    }

    pub fn get_headers(&self) -> Headers {
        let mut headers = self.headers.clone();
        if !self.body.is_empty() {
            headers.insert("Content-Length".to_string(), self.body.len().to_string());
        }
        headers.insert("Power-By".to_string(), "rust".to_string());
        headers
    }
}

impl Display for Response {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}\r\n", self.res)?;

        write!(f, "{}", header_tostr(&self.get_headers()))?;

        write!(f, "\r\n{}", self.body_str())
    }
}

#[derive(Default)]
pub struct ResponseBuilder {
    res: Option<Response>,
}

/// * Example
/// ```
/// let res: Response = ResponseBuilder::new.code(200).build();
/// ```
impl ResponseBuilder {
    pub fn new() -> ResponseBuilder {
        ResponseBuilder {
            res: Some(Response {
                res: ResLine {
                    version: "HTTP/1.1".to_string(),
                    code: 200,
                    code_line: "OK".to_string(),
                },
                headers: HashMap::new(),
                body: Vec::new(),
            }),
        }
    }

    // TODO
    #[deny(clippy::single_match)]
    pub fn code(&mut self, code: usize) -> &mut Self {
        if code == 200 {}
        self
    }

    pub fn build(&mut self) -> Response {
        self.res.take().unwrap()
    }
}
