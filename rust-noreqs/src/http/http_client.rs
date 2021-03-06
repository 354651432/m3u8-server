use std::{
    collections::HashMap,
    io::{BufRead, BufReader, Read, Write},
    net::TcpStream,
    time::Duration,
};

use openssl::ssl::{SslConnector, SslMethod, SslStream};
use serde::Serialize;
use socks::Socks5Stream;

use super::url::*;
use super::{proxy::Proxy, rwiter::*};
use super::{request::Request, response::*, ReqLine};
use regex::*;

#[cfg(test)]
#[path = "http_client_test.rs"]
mod test;

#[derive(Default)]
pub struct HttpClient {
    headers: HashMap<String, String>,
    proxy: Proxy,
}

static HTTP_VERION: &str = "HTTP/1.1";
static READ_TIMEOUT: Duration = Duration::from_millis(100);
static WRITE_TIMEOUT: Duration = Duration::from_millis(100);

impl HttpClient {
    pub fn new() -> Self {
        let mut headers = HashMap::new();
        headers.insert(String::from("User-Agent"), String::from("curl/7.76.1"));
        headers.insert(String::from("Accept"), String::from("*/*"));
        Self {
            headers,
            proxy: Proxy::default(),
        }
    }

    pub fn proxy(&mut self, proxy: &str) -> &Self {
        self.proxy = Proxy::new(proxy);
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

    fn build_base_stream(&self, url: &Url) -> Result<StreamWapper, String> {
        if let Proxy::Http(addr) = &self.proxy {
            let mut stream = match TcpStream::connect(addr) {
                Ok(mut stream) => stream,
                Err(err) => return Err(err.to_string()),
            };
            // 设置 超时以后 ssl 连接 构建会失败
            // stream.set_read_timeout(Some(READ_TIMEOUT));
            // stream.set_write_timeout(Some(WRITE_TIMEOUT));

            if let Err(err) = Write::write(
                &mut stream,
                format!("CONNECT {}:{} HTTP/1.1\r\n\r\n", url.host, url.port).as_bytes(),
            ) {
                return Err(err.to_string());
            }

            let mut reader = BufReader::new(stream);
            let mut buf = String::new();
            if let Err(err) = reader.read_line(&mut buf) {
                return Err(err.to_string());
            }

            return Ok(StreamWapper::from_stream(reader.into_inner()));
        }

        if let Proxy::Socks5(addr) = &self.proxy {
            match Socks5Stream::connect(addr, url.to_host().as_str()) {
                Ok(stream) => {
                    // cannot set timeout for Socks5Stream
                    return Ok(StreamWapper::from_stream(stream));
                }
                Err(err) => return Err(err.to_string()),
            }
        }

        match TcpStream::connect(url.to_host()) {
            Ok(stream) => {
                stream.set_read_timeout(Some(READ_TIMEOUT));
                stream.set_write_timeout(Some(WRITE_TIMEOUT));
                Ok(StreamWapper::from_stream(stream))
            }
            Err(err) => Err(err.to_string()),
        }
    }

    fn build_https_stream(&self, url: &Url, stream: StreamWapper) -> Result<StreamWapper, String> {
        let connector = SslConnector::builder(SslMethod::tls()).unwrap().build();
        let mut stream = match connector.connect(&url.host, stream) {
            Ok(stream) => stream,
            Err(err) => return Err("build https stream failed".to_string()),
        };
        Ok(StreamWapper::from_stream(stream))
    }

    fn build_stream(&self, url: Url) -> Result<StreamWapper, String> {
        let stream = self.build_base_stream(&url);
        if url.proto.to_lowercase() == "http" {
            return stream;
        }

        self.build_https_stream(&url, stream?)
    }

    pub fn request(&self, url: &str, method: &str, data: Vec<u8>) -> Result<Response, String> {
        let url = match Url::new(url.trim()) {
            Some(url) => url,
            None => return Err("parse url failed".to_string()),
        };

        let mut req = Self::build_req(&url, method.to_uppercase().as_ref());
        req.header("Host", &url.host);
        req.body = data;

        let mut stream = self.build_stream(url)?;

        for (key, value) in &self.headers {
            req.header(key, value);
        }

        // println!("{:#?}", req.headers);
        let buf = req.to_string();
        if let Err(err) = Write::write(&mut stream, buf.as_bytes()) {
            return Err(err.to_string());
        }

        match Response::from_stream(stream) {
            Some(res) => {
                let code = res.res.code;
                if (200..=300).contains(&code) {
                    Ok(res)
                } else {
                    Err(format!("response code is {}", code))
                }
            }
            None => Err("read from stream failed".to_string()),
        }
    }

    pub fn get(&self, url: &str) -> Result<Response, String> {
        Self::request(self, url, "GET", Vec::new())
    }

    pub fn post(&self, url: &str, data: Vec<u8>) -> Result<Response, String> {
        Self::request(self, url, "POST", data)
    }

    pub fn head(&self, url: &str) -> Result<Response, String> {
        Self::request(self, url, "HEAD", Vec::new())
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
