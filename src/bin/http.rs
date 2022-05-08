use std::{
    io::Read,
    net::{TcpListener, TcpStream},
};

use stream_httparse::streaming_parser::ReqParser;

fn main() {
    let tcp = TcpListener::bind("0.0.0.0:2020").unwrap();
    for stream in tcp.incoming() {
        println!("incoming");
        let mut stream: TcpStream = stream.unwrap();

        let str1 = read_to_string(&mut stream);

        let mut parser = ReqParser::new_capacity(str1.len());
        let (ret, _) = parser.block_parse(str1.as_bytes());
        if !ret {
            eprintln!("parse error");
            continue;
        }
        let req = parser.finish().unwrap();

        println!("{:?}", req);
    }
}

fn read_to_string(stream: &mut TcpStream) -> String {
    let mut str1 = String::new();
    let mut buf = [0; 1024];
    loop {
        if let Ok(len) = stream.read(&mut buf) {
            let sub_str = String::from_utf8_lossy(&buf[..len]).into_owned();
            str1.push_str(&sub_str);
        } else {
            return str1;
        }
    }
}
