use std::io::{Read, Write};

use openssl::ssl::{SslConnector, SslMethod};
use socks::Socks5Stream;

fn main() {
    let connector = SslConnector::builder(SslMethod::tls()).unwrap().build();

    let stream = Socks5Stream::connect("127.0.0.1:10808", "www.google.com:443").unwrap();
    let mut stream = connector.connect("www.google.com", stream).unwrap();

    stream.write(b"GET / HTTP/1.0\r\n\r\n").unwrap();
    let mut res = vec![];
    stream.read_to_end(&mut res).unwrap();
    println!("{}", String::from_utf8_lossy(&res));
}
