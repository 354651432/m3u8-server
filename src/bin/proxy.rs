use std::{
    io::{Read, Write},
    net::TcpStream,
};

use openssl::ssl::{SslConnector, SslMethod};

fn main() {
    let mut stream = TcpStream::connect("127.0.0.1:10808").unwrap();
    stream.write(b"\x05\x01\x00").unwrap();

    let mut buf = [0; 10];
    let size = stream.read(&mut buf).unwrap();
    if size == 2 && buf[1] == 0x00 {
        println!("auth success!");
    } else {
        panic!("auth failed!");
    }

    let host = "www.github.com";
    let port: u16 = 443;
    let mut arr = Vec::from("\x05\x01\x00\x03".as_bytes());
    arr.push(host.len() as u8);
    for c in host.chars() {
        arr.push(c as u8);
    }
    arr.push((port / 255) as u8);
    arr.push((port % 255) as u8);

    println!("connect: {arr:#x?}");

    stream.write(&arr).unwrap();

    let mut buf = [0; 10];
    let size = stream.read(&mut buf).unwrap();

    println!("connect recv {:#x?}", &buf[..size]);

    if size > 1 && buf[1] == 0x00 {
        println!("connection success! size={size}");

        let port: u16 = (buf[size - 2] as u16) * 255 + (buf[size - 1] as u16);
        println!("new port {port}");
    }
    let connector = SslConnector::builder(SslMethod::tls()).unwrap().build();

    let mut stream = connector
        .connect(format!("{host}").as_str(), stream)
        .unwrap();

    stream.write(b"GET / HTTP/2.0\r\n\r\n").unwrap();
    let mut res = Vec::with_capacity(20);
    unsafe {
        res.set_len(20);
    }
    stream.read_exact(&mut res).unwrap();
    println!("{}", String::from_utf8_lossy(&res));
}
