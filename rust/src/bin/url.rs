fn main() {
    let url="https://hls-hw.xvideos-cdn.com/videos_new/hls/3c/07/13/3c071395b2bb4fa3a002a2f2675731fd-1/hls-1080p-44e7c.m3u8?e=1653217594&l=0&h=683406bf2c914ac6aa70b15089fd79ce";
    let url = url::Url::parse(url).unwrap();
    let url = url.join("111.ts?fuck").unwrap();
    println!("{url}");
}
