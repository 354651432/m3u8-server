use std::{
    io::{Read, Write},
    net::TcpStream,
};

use socks::Socks5Stream;

// 组合 Read + Write
pub trait IReadWriter {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize, std::io::Error>;
    fn write(&mut self, buf: &[u8]) -> Result<usize, std::io::Error>;
}

impl<T: Read + Write> IReadWriter for T {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize, std::io::Error> {
        self.read(buf)
    }

    fn write(&mut self, buf: &[u8]) -> Result<usize, std::io::Error> {
        self.write(buf)
    }
}

// 封装没有Https的流
pub enum HttpWriter {
    TcpStream(TcpStream),
    Scoks5(Socks5Stream),
}
impl IReadWriter for HttpWriter {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize, std::io::Error> {
        match self {
            HttpWriter::TcpStream(stream) => IReadWriter::read(stream, buf),
            HttpWriter::Scoks5(stream) => IReadWriter::read(stream, buf),
        }
    }

    fn write(&mut self, buf: &[u8]) -> Result<usize, std::io::Error> {
        match self {
            HttpWriter::TcpStream(stream) => Write::write(stream, buf),
            HttpWriter::Scoks5(stream) => Write::write(stream, buf),
        }
    }
}

// 为了让 IReadWriter 特征对象实现 Read
pub struct LocalRead(pub Box<dyn IReadWriter>);

impl Read for LocalRead {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        self.0.read(buf)
    }
}
