use std::{
    collections::HashMap,
    io::{Read, Write},
    net::TcpStream,
    time::Duration,
};

use openssl::ssl::{SslConnector, SslMethod, SslStream};
use serde::Serialize;

use super::{request::Request, response::*, ReqLine};
use regex::*;

#[cfg(test)]
#[path = "http_client_test.rs"]
mod test;

#[derive(Debug)]
pub struct Url {
    pub proto: String,
    pub host: String,
    pub port: usize,
    pub path: String,
}

impl Url {
    //r"'(?P<title>[^']+)'\s+\((?P<year>\d{4})\)"
    pub fn new(url: &str) -> Option<Self> {
        let reg = Regex::new(r"(?P<proto>https?)://(?P<host>[.\w]+)(?P<port>:\w+)?(?P<path>/.+)?")
            .unwrap();
        let mch = reg.captures(url)?;

        let proto = mch.name("proto")?.as_str().to_lowercase().to_owned();
        Some(Self {
            host: mch.name("host")?.as_str().to_owned(),
            port: match mch.name("port") {
                Some(port) => {
                    let port = port.as_str()[1..].parse().ok()?;
                    port
                }
                None => {
                    if &proto == "http" {
                        80
                    } else {
                        443
                    }
                }
            },
            proto,
            path: match mch.name("path") {
                Some(path) => path.as_str().to_owned(),
                None => "/".to_owned(),
            },
        })
    }

    pub fn to_host(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }
}

pub enum ReadWriter {
    TcpStream(TcpStream),
    SslStream(SslStream<TcpStream>),
}

impl Read for ReadWriter {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        match self {
            ReadWriter::TcpStream(stream) => stream.read(buf),
            ReadWriter::SslStream(stream) => stream.read(buf),
        }
    }
}
impl Write for ReadWriter {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        match self {
            ReadWriter::TcpStream(stream) => stream.write(buf),
            ReadWriter::SslStream(stream) => stream.write(buf),
        }
    }

    fn flush(&mut self) -> std::io::Result<()> {
        match self {
            ReadWriter::TcpStream(stream) => stream.flush(),
            ReadWriter::SslStream(stream) => stream.flush(),
        }
    }
}
pub struct HttpClient {
    headers: HashMap<String, String>,
}

static HTTP_VERION: &'static str = "HTTP/1.1";

impl HttpClient {
    pub fn new() -> Self {
        let mut headers = HashMap::new();
        headers.insert(String::from("User-Agent"), String::from("curl/7.76.1"));
        headers.insert(String::from("Accept"), String::from("*/*"));
        Self { headers }
    }

    fn build_req(url: &Url, method: &str) -> Request {
        Request::new(
            ReqLine {
                method: String::from(method),
                path: String::from(&url.path),
                version: String::from(HTTP_VERION),
            },
            HashMap::new(),
            Vec::new(),
        )
    }

    fn build_stream(url: &Url) -> Result<ReadWriter, String> {
        let mut stream: TcpStream = match TcpStream::connect(url.to_host()) {
            Ok(stream) => stream,
            Err(err) => return Err(err.to_string()),
        };

        if url.proto == "http" {
            return Ok(ReadWriter::TcpStream(stream));
        }

        let connector = SslConnector::builder(SslMethod::tls()).unwrap().build();
        let mut stream: SslStream<TcpStream> = match connector.connect(&url.host, stream) {
            Ok(stream) => stream,
            Err(err) => return Err(err.to_string()),
        };
        Ok(ReadWriter::SslStream(stream))
    }

    pub fn request(&self, url: &str, method: &str, data: Vec<u8>) -> Option<Response> {
        let url = Url::new(url)?;
        let mut stream = Self::build_stream(&url).ok()?;

        let mut req = Self::build_req(&url, method.to_uppercase().as_ref());
        req.header("Host", &url.host);
        req.body = data;

        for (key, value) in &self.headers {
            req.header(key, value);
        }

        let buf = req.to_string();
        stream.write(buf.as_bytes()).ok()?;

        Response::from_stream(stream)
    }

    pub fn get(&self, url: &str) -> Option<Response> {
        Self::request(&self, url, "GET", Vec::new())
    }

    pub fn post(&self, url: &str, data: Vec<u8>) -> Option<Response> {
        Self::request(&self, url, "POST", data)
    }

    pub fn header<T, U>(&mut self, key: T, value: U) -> Option<String>
    where
        String: std::convert::From<T>,
        String: std::convert::From<U>,
    {
        self.headers.insert(String::from(key), String::from(value))
    }

    // pub fn post
}
