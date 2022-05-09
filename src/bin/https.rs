use openssl::ssl::{SslConnector, SslMethod};
use std::io::{Read, Write};
use std::net::TcpStream;

fn main() {
    let connector = SslConnector::builder(SslMethod::tls()).unwrap().build();

    let stream = TcpStream::connect("stackoverflow.com:443").unwrap();
    let mut stream = connector.connect("stackoverflow.com", stream).unwrap();

    stream.write("GET /questions/57641402/deviant-rust-how-can-i-disable-all-the-warnings-and-checks-possible HTTP/1.0\r
Host: stackoverflow.com\r\nUser-Agent: Rust\r\n\r\n".as_bytes()).unwrap();
    let mut res = vec![];
    stream.read_to_end(&mut res).unwrap();
    println!("{}", String::from_utf8_lossy(&res));
}
