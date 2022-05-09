use work::http::http_client::HttpClient;

fn main() {
    let res = HttpClient::new()
        .post("https://www.baidu.com/", Vec::new())
        .unwrap();

    println!("{:?}", res.res);
    println!("{:?}", res.headers);
    println!("{:?}", res.body_str());
}
