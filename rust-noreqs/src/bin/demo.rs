#![allow(unused)]
use std::{
    io::{BufRead, BufReader, Read, Write},
    net::TcpStream,
};

use openssl::ssl::{Ssl, SslConnector, SslMethod, SslStream};

fn main() {
    let mut stream = TcpStream::connect("127.0.0.1:1087").unwrap();

    let connector = SslConnector::builder(SslMethod::tls()).unwrap().build();

    stream
        .write_all(b"CONNECT www.google.com:80 HTTP/1.1\r\n\r\n")
        .unwrap();

    let mut reader = BufReader::new(stream);

    let mut buf = String::new();
    let size = reader.read_line(&mut buf).unwrap();
    // println!("{buf} {size}");

    // let mut ssl_stream = connector
    //     .connect("www.google.com", reader.get_ref())
    //     .unwrap();
    reader
        .get_ref()
        .write_all(b"GET / HTTP/1.1\r\nConnection: Close\r\n\r\n");

    let mut buf = vec![];
    reader.read_to_end(&mut buf).unwrap();
    println!("{}", String::from_utf8_lossy(&buf));
}
