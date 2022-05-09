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
pub struct HttpClient {}

impl HttpClient {
    pub fn new() -> Self {
        Self {}
    }

    pub fn get(&self, url: &str) -> Option<Response> {
        let url = Url::new(url)?;
        let mut stream = Self::build_stream(&url).ok()?;

        let mut req = Self::build_req(&url, "GET");
        req.header("Host", &url.host);

        let buf = req.to_string();
        stream.write(buf.as_bytes()).ok()?;

        Response::from_stream(stream)
    }

    fn build_req(url: &Url, method: &str) -> Request {
        Request::new(
            ReqLine {
                method: String::from(method),
                path: String::from(&url.path),
                version: String::from("HTTP/1.1"),
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

    pub fn post<T>(&self, url: &str, data: T) -> Option<Response>
    where
        T: Serialize,
    {
        todo!()
    }

    pub fn add_header(key: &str, value: &str) {}

    fn parse_url() {}
}
