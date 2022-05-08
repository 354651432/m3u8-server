use super::*;

#[test]
fn test_build_url() {
    let str = "https://w.wallhaven.cc/full/k7/wallhaven-k7v9yq.png?f=uck";
    let res = build_url(str, "fucing.ing").unwrap();

    assert_eq!(res, "https://w.wallhaven.cc/full/k7/fucing.ing");
}

#[tokio::test]
async fn test_get_text() {
    let map: HashMap<String, String> = HashMap::new();
    let client = build_client(&map);
    let text = get_text(
        &client.unwrap(),
        "https://docs.rs/reqwest/0.11.10/reqwest/struct.Error.html",
    )
    .await
    .unwrap();

    assert_ne!(text, "");
}

#[tokio::test]
async fn test_download() -> std::result::Result<(), Box<dyn std::error::Error>> {
    let url=" https://cdn77-vid.xvideos-cdn.com/F6G0ZcMr3MRQykiqZ9Y1HA==,1651514262/videos/hls/94/ca/f1/94caf1872c3c28214ec2be535243eead/hls-360p.m3u8";

    let map: HashMap<String, String> = HashMap::new();
    download_m3u8(url, "result/1.ts", &map).await
}