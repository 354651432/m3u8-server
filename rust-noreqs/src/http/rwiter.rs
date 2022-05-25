use std::{
    fmt::{Display, Pointer},
    io::{Read, Write},
    net::TcpStream,
};

use socks::Socks5Stream;

// 组合 Read + Write
pub trait Stream: Read + Write {}

impl<T: Read + Write> Stream for T {}

// 为了让 StreamWapper 特征对象实现 Read
pub struct StreamWapper(pub Box<dyn Stream>);

impl StreamWapper {
    pub fn from_stream<T: Read + Write + 'static>(stream: T) -> Self {
        Self(Box::new(stream))
    }
}

impl Display for StreamWapper {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl Read for StreamWapper {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        self.0.read(buf)
    }
}

impl Write for StreamWapper {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.0.write(buf)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        self.0.flush()
    }
}
