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

fn build_url(url: &str, name: &str) -> Option<String> {
    let idx = url.rfind("/")?;
    Some(format!("{}/{}", &url[..idx], name))
}

#[cfg(test)]
mod tests;
