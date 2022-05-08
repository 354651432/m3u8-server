use std::{
    cell::RefCell,
    io::Write,
    net::{TcpListener, TcpStream},
};

use super::{request::Request, response::Response};

pub struct HttpListener {
    tcp: TcpListener,
}

pub struct Context {
    pub req: Request,
    pub res: Response,
    pub stream: RefCell<TcpStream>,
}

impl HttpListener {
    pub fn bind(addr: &str) -> Self {
        let tcp = TcpListener::bind(addr).unwrap();
        HttpListener { tcp }
    }
}

impl Iterator for HttpListener {
    type Item = Context;

    fn next(&mut self) -> Option<Self::Item> {
        let stream = self.tcp.incoming().next()?.ok()?;

        let req = Request::read_from_stream(&stream)?;
        let res = Response::default();
        Some(Context {
            req,
            res,
            stream: RefCell::new(stream),
        })
    }
}

impl Context {
    pub fn send(&self) -> Result<usize, std::io::Error> {
        let stream: &mut TcpStream = &mut self.stream.borrow_mut();
        stream.write(self.res.to_string().as_bytes())
    }
}

impl Drop for Context {
    fn drop(&mut self) {
        self.send().unwrap();
    }
}
