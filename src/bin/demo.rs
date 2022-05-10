use std::io::Write;

use work::http::http_client::HttpClient;
use work::m3u8::parse;

fn main() {
    let url="https://hls-hw.xvideos-cdn.com/videos/hls/41/b8/0d/41b80d45100bf8593ddf0d4bfeb4e79d/hls-480p-1c7af.m3u8?e=1652166462&l=0&h=dfcfebd41acf9bb6b21391ff05c48200";
    let mut req = HttpClient::new();
    req.proxy("127.0.0.1:10808");
    let res = req
        .get(url)
        // .get("https://cdn77-vid.xvideos-cdn.com/N6U89LVzQ-yk_BTRcdj4Wg==,1652111655/videos/hls/48/c5/11/48c5118c275bd352c30a2cc283440d62-1/hls-1080p-5ea9c0.ts")
        .unwrap();

    let lines = parse(url, &res.body_str());

    let mut fs = std::fs::File::create("abc.ts").unwrap();
    for line in &lines {
        let res = req.get(line).unwrap();
        println!("downloading {}", line);
        fs.write(&res.body).unwrap();
    }
}
