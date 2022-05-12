use std::{
    collections::HashMap,
    io::{BufRead, BufReader, Read, Write},
    net::TcpStream,
};

use openssl::ssl::{SslConnector, SslMethod};
use work::http::{request::Request, ReqLine};

fn main() {
    let mut headers = HashMap::new();
    headers.insert("Host".to_string(), "google.com".to_string());
    let req = Request::new(
        ReqLine {
            method: "GET".to_string(),
            path: "http://www.google.com/".to_string(),
            version: "HTTP/1.1".to_string(),
        },
        headers,
        Vec::new(),
    );

    let mut tcp = TcpStream::connect("127.0.0.1:1087").unwrap();
    tcp.write(req.to_string().as_bytes()).unwrap();

    // let connector = SslConnector::builder(SslMethod::tls()).unwrap().build();
    // let mut stream = connector.connect("www.google.com", tcp).unwrap();

    // let mut buf = Vec::new();
    // tcp.read_to_end(&mut buf).unwrap();

    let buf = read_trunked_stream(tcp);

    println!("{}", String::from_utf8_lossy(&buf));
}

fn read_trunked_stream(stream: impl Read) -> Vec<u8> {
    let mut buf = Vec::new();
    let mut reader = BufReader::new(stream);
    loop {
        let mut line = String::new();
        reader.read_line(&mut line).unwrap();
        println!("{line}");
        if line.trim().is_empty() {
            break;
        }
    }

    loop {
        let mut line = String::new();
        reader.read_line(&mut line).unwrap();

        let line = line.trim();
        if line == "0" || line.is_empty() {
            break;
        }

        // println!("line {line}");
        let length = usize::from_str_radix(&line, 16).unwrap();
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
