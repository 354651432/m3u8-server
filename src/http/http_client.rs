use std::{
    collections::HashMap,
    io::{Read, Write},
    net::TcpStream,
    time::Duration,
};

use openssl::ssl::{SslConnector, SslMethod, SslStream};
use serde::Serialize;
use socks::Socks5Stream;

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
        let reg = Regex::new(r"(?P<proto>https?)://(?P<host>[^/]+)(?P<port>:\w+)?(?P<path>/.+)?")
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
    Socks5Stream(Socks5Stream),
    SslStream(SslStream<TcpStream>),
    SslStreamSS(SslStream<Socks5Stream>),
}

impl Read for ReadWriter {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        match self {
            ReadWriter::TcpStream(stream) => stream.read(buf),
            ReadWriter::SslStream(stream) => stream.read(buf),
            ReadWriter::Socks5Stream(stream) => stream.read(buf),
            ReadWriter::SslStreamSS(stream) => stream.read(buf),
        }
    }
}
impl Write for ReadWriter {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        match self {
            ReadWriter::TcpStream(stream) => stream.write(buf),
            ReadWriter::SslStream(stream) => stream.write(buf),
            ReadWriter::Socks5Stream(stream) => stream.write(buf),
            ReadWriter::SslStreamSS(stream) => stream.write(buf),
        }
    }

    fn flush(&mut self) -> std::io::Result<()> {
        match self {
            ReadWriter::TcpStream(stream) => stream.flush(),
            ReadWriter::SslStream(stream) => stream.flush(),
            ReadWriter::Socks5Stream(stream) => stream.flush(),
            ReadWriter::SslStreamSS(stream) => stream.flush(),
        }
    }
}
pub struct HttpClient {
    headers: HashMap<String, String>,
    proxy: String,
}

static HTTP_VERION: &'static str = "HTTP/1.1";
static READ_TIMEOUT: Duration = Duration::from_millis(100);
static WRITE_TIMEOUT: Duration = Duration::from_millis(100);

impl HttpClient {
    pub fn new() -> Self {
        let mut headers = HashMap::new();
        headers.insert(String::from("User-Agent"), String::from("curl/7.76.1"));
        headers.insert(String::from("Accept"), String::from("*/*"));
        Self {
            headers,
            proxy: String::new(),
        }
    }

    pub fn proxy(&mut self, proxy: &str) -> &Self {
        self.proxy = String::from(proxy);
        self
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

    fn build_base_stream(&self, url: &Url) -> Result<ReadWriter, String> {
        if !self.proxy.trim().is_empty() {
            match Socks5Stream::connect(&self.proxy, url.to_host().as_str()) {
                Ok(stream) => return Ok(ReadWriter::Socks5Stream(stream)),
                Err(err) => return Err(err.to_string()),
            }
        }

        match TcpStream::connect(url.to_host()) {
            Ok(stream) => Ok(ReadWriter::TcpStream(stream)),
            Err(err) => return Err(err.to_string()),
        }
    }

    fn build_https_stream(&self, url: &Url, stream: ReadWriter) -> Result<ReadWriter, String> {
        match stream {
            ReadWriter::TcpStream(stream) => {
                stream.set_read_timeout(Some(READ_TIMEOUT)).unwrap();
                stream.set_write_timeout(Some(WRITE_TIMEOUT)).unwrap();
                stream.set_nodelay(true);

                let connector = SslConnector::builder(SslMethod::tls()).unwrap().build();
                let mut stream: SslStream<TcpStream> = match connector.connect(&url.host, stream) {
                    Ok(stream) => stream,
                    Err(err) => return Err(err.to_string()),
                };
                Ok(ReadWriter::SslStream(stream))
            }
            ReadWriter::Socks5Stream(stream) => {
                let connector = SslConnector::builder(SslMethod::tls()).unwrap().build();
                let mut stream: SslStream<Socks5Stream> = match connector.connect(&url.host, stream)
                {
                    Ok(stream) => stream,
                    Err(err) => return Err(err.to_string()),
                };
                Ok(ReadWriter::SslStreamSS(stream))
            }
            stream => Ok(stream),
        }
    }

    fn build_stream(&self, url: &Url) -> Result<ReadWriter, String> {
        let stream = self.build_base_stream(&url);
        if url.proto.to_lowercase() == "http" {
            return stream;
        }
        return self.build_https_stream(&url, stream?);
    }

    pub fn request(&self, url: &str, method: &str, data: Vec<u8>) -> Result<Response, String> {
        let url = match Url::new(url) {
            Some(url) => url,
            None => return Err("parse url failed".to_string()),
        };

        let mut stream = self.build_stream(&url)?;

        let mut req = Self::build_req(&url, method.to_uppercase().as_ref());
        req.header("Host", &url.host);
        req.body = data;

        for (key, value) in &self.headers {
            req.header(key, value);
        }

        // println!("{:#?}", req.headers);
        let buf = req.to_string();
        if let Err(err) = stream.write(buf.as_bytes()) {
            return Err(err.to_string());
        }

        match Response::from_stream(stream) {
            Some(res) => {
                let code = res.res.code;
                if code >= 200 && code <= 300 {
                    Ok(res)
                } else {
                    Err(format!("response code is {}", code))
                }
            }
            None => Err("read from stream failed".to_string()),
        }
    }

    pub fn get(&self, url: &str) -> Result<Response, String> {
        Self::request(&self, url, "GET", Vec::new())
    }

    pub fn post(&self, url: &str, data: Vec<u8>) -> Result<Response, String> {
        Self::request(&self, url, "POST", data)
    }

    pub fn head(&self, url: &str) -> Result<Response, String> {
        Self::request(&self, url, "HEAD", Vec::new())
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
