use std::{
    collections::HashMap,
    io::{Read, Write},
    net::TcpStream,
    time::Duration,
};

use openssl::ssl::{SslConnector, SslMethod, SslStream};
use serde::Serialize;
use socks::Socks5Stream;

use super::rwiter::*;
use super::url::*;
use super::{request::Request, response::*, ReqLine};
use regex::*;

#[cfg(test)]
#[path = "http_client_test.rs"]
mod test;

#[derive(Default)]
pub struct HttpClient {
    headers: HashMap<String, String>,
    proxy: String,
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
            proxy: String::new(),
        }
    }

    pub fn proxy(&mut self, proxy: &str) -> &Self {
        // self.proxy = proxy.to_lowercase().replace("socks5://", "");
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

    fn build_base_stream(&self, url: &Url) -> Result<HttpWriter, String> {
        if !self.proxy.trim().is_empty() {
            // TODO 增加http
            let proxy = self.proxy.to_lowercase().replace("socks5://", "");
            match Socks5Stream::connect(proxy, url.to_host().as_str()) {
                Ok(stream) => {
                    // cannot set timeout for Socks5Stream
                    return Ok(HttpWriter::Scoks5(stream));
                }
                Err(err) => return Err(err.to_string()),
            }
        }

        match TcpStream::connect(url.to_host()) {
            Ok(stream) => {
                stream.set_read_timeout(Some(READ_TIMEOUT));
                stream.set_write_timeout(Some(WRITE_TIMEOUT));
                Ok(HttpWriter::TcpStream(stream))
            }
            Err(err) => Err(err.to_string()),
        }
    }

    fn build_https_stream(
        &self,
        url: &Url,
        stream: HttpWriter,
    ) -> Result<Box<dyn IReadWriter>, String> {
        let connector = SslConnector::builder(SslMethod::tls()).unwrap().build();
        match stream {
            HttpWriter::TcpStream(stream) => {
                let mut stream = match connector.connect(&url.host, stream) {
                    Ok(stream) => stream,
                    Err(err) => return Err(err.to_string()),
                };
                Ok(Box::new(stream))
            }
            HttpWriter::Scoks5(stream) => {
                let mut stream = match connector.connect(&url.host, stream) {
                    Ok(stream) => stream,
                    Err(err) => return Err(err.to_string()),
                };
                Ok(Box::new(stream))
            }
        }
    }

    fn build_stream(&self, url: Url) -> Result<Box<dyn IReadWriter>, String> {
        let stream = self.build_base_stream(&url);
        if url.proto.to_lowercase() == "http" {
            return Ok(Box::new(stream?));
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
        if let Err(err) = stream.write(buf.as_bytes()) {
            return Err(err.to_string());
        }

        match Response::from_stream(LocalRead::new(stream)) {
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
