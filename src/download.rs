use bytes::Bytes;
use std::collections::HashMap;
use std::fs;
use std::io::Write;

pub async fn download_m3u8(
    base_url: &str,
    file_name: &str,
    headers: &HashMap<String, String>,
) -> std::result::Result<(), Box<dyn std::error::Error>> {
    println!("beginning downloading {}", base_url);

    let client = build_client(headers)?;
    let m3u8 = get_text(&client, base_url).await?;

    let lines = m3u8.split("\n");
    let tmp_name = format!("{}_tmp", file_name);

    let mut fs = match fs::File::create(&tmp_name) {
        Ok(fs) => fs,
        Err(err) => {
            eprintln!("{}", err);
            return Ok(());
        }
    };

    for url in lines {
        if url.starts_with("#") {
            continue;
        }

        if url == "" {
            continue;
        }

        let full_url = match build_url(base_url, url) {
            Some(value) => value,
            None => continue,
        };

        println!("downloading {}", url);
        let bytes = get(&client, &full_url).await?;
        if let Err(err) = fs.write(&bytes[..]) {
            eprintln!("{}", err);
        }
    }

    if let Err(err) = fs::rename(tmp_name, file_name) {
        eprintln!("{}", err);
    }
    println!("{} downladed", file_name);
    Ok(())
}

#[tokio::test]
async fn test_download() -> std::result::Result<(), Box<dyn std::error::Error>> {
    let url=" https://cdn77-vid.xvideos-cdn.com/F6G0ZcMr3MRQykiqZ9Y1HA==,1651514262/videos/hls/94/ca/f1/94caf1872c3c28214ec2be535243eead/hls-360p.m3u8";

    let map: HashMap<String, String> = HashMap::new();
    download_m3u8(url, "result/1.ts", &map).await
}

fn build_client(
    headers: &HashMap<String, String>,
) -> std::result::Result<reqwest::Client, reqwest::Error> {
    let proxy = reqwest::Proxy::https("127.0.0.1:1087")?;
    let client = reqwest::Client::builder().proxy(proxy).build()?;
    for it in headers {
        let _ = client.head(format!("{}:{}", it.0, it.1));
    }
    Ok(client)
}

async fn get(client: &reqwest::Client, url: &str) -> Result<Bytes, String> {
    let resp = match client.get(url).send().await {
        Ok(resp) => resp,
        Err(_) => return Err(String::from("send error")),
    };
    if resp.status() != reqwest::StatusCode::OK {
        return Err(format!("status code is {}", resp.status()));
    }

    match resp.bytes().await {
        Ok(ret) => Ok(ret),
        Err(err) => Err(format!("{}", err)),
    }
}

async fn get_text(client: &reqwest::Client, url: &str) -> Result<String, String> {
    let resp = client.get(url).send().await;
    let resp = match resp {
        Ok(ret) => ret,
        Err(err) => return Err(format!("{}", err)),
    };

    if resp.status() != reqwest::StatusCode::OK {
        return Err(format!("status code is {}", resp.status()));
    }
    match resp.text().await {
        Ok(ret) => Ok(ret),
        Err(err) => Err(format!("{}", err)),
    }
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

fn build_url(url: &str, name: &str) -> Option<String> {
    let idx = url.rfind("/")?;
    Some(format!("{}/{}", &url[..idx], name))
}

#[test]
fn test_build_url() {
    let str = "https://w.wallhaven.cc/full/k7/wallhaven-k7v9yq.png?f=uck";
    let res = build_url(str, "fucing.ing").unwrap();

    assert_eq!(res, "https://w.wallhaven.cc/full/k7/fucing.ing");
}
