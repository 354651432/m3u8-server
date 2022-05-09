use work::http::http_client::HttpClient;

fn main() {
    let res = HttpClient::new()
        .proxy("127.0.0.1:10808")
        .get("https://cdn77-vid.xvideos-cdn.com/N6U89LVzQ-yk_BTRcdj4Wg==,1652111655/videos/hls/48/c5/11/48c5118c275bd352c30a2cc283440d62-1/hls-1080p-5ea9c.m3u8")
        .unwrap();

    println!("{:?}", res.res);
    println!("{:?}", res.headers);
    println!("{:?}", res.body_str());
}
