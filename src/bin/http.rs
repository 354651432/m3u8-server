use std::thread;

use work::http::http_listener::HttpListener;

fn main() {
    for mut ctx in HttpListener::bind("127.0.0.1:8080") {
        thread::spawn(move || {
            ctx.res.body("<h1>it works!");
        });
    }
}
