use std::{io::Write, net::TcpStream, time::Duration};

use serde::Serialize;

use super::{request::Request, response::*};
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

struct HttpClient {}

impl HttpClient {
    pub fn new() -> Self {
        Self {}
    }

    #[no_mangle]
    pub fn get(&self, url: &str) -> Option<Response> {
        let url = Url::new(url)?;
        let mut socket = match TcpStream::connect(url.to_host()) {
            Ok(socket) => socket,
            Err(err) => panic!("{}", err),
        };

        let mut req = Request::new();
        req.path = url.path;
        req.header("Host", &url.host);
        socket.write(req.to_string().as_bytes()).ok()?;

        socket.set_write_timeout(Some(Duration::from_secs(3)));
        Response::from_stream(socket)
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
