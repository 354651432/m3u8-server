use work::http::http_listener::HttpListener;

fn main() {
    for mut ctx in HttpListener::bind("127.0.0.1:2020") {
        ctx.res.body("<h1>it works!");
    }
}
