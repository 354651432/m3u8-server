use work::http::http_client::HttpClient;

fn main() {
    let res = HttpClient::new().get("https://www.baidu.com/").unwrap();

    println!("{:?}", res.res);
    println!("{:?}", res.headers);
}
