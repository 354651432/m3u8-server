use work::http::http_client::HttpClient;

fn main() {
    let res = HttpClient::new()
        .proxy("127.0.0.1:10808")
        .get("https://www.google.com/")
        .unwrap();

    println!("{:?}", res.res);
    println!("{:?}", res.headers);
    println!("{:?}", res.body_str());
}
