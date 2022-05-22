use std::cmp::min;
use std::fs::File;
use std::sync::Arc;
use std::{collections::HashMap, fmt::Write, io::Write as IoWrite};
use std::{thread, time};

use crate::{http::http_client::HttpClient, m3u8::parse};
extern crate colorful;

use colorful::Color;
use colorful::Colorful;

const SHOW_LOG: bool = true;

pub fn download(
    url: &str,
    file_name: &str,
    proxy: Option<&str>,
    headers: HashMap<String, String>,
) -> Result<usize, String> {
    let mut req = HttpClient::new();
    if let Some(prox) = proxy {
        req.proxy(prox);
    }

    let res = req.get(url)?;

    let lines = parse(url, &res.body_str());

    let md5 = md5::compute(url);
    let mut md5_str = String::new();
    for c in md5.0 {
        write!(&mut md5_str, "{:x}", c);
    }
    let md5_str = &md5_str[..9];

    let tmp_file_name = format!("{file_name}.downloading.{}", md5_str);
    println!(
        "create file {}",
        String::from(&tmp_file_name).as_str().light_blue()
    );

    let mut fs = match std::fs::File::create(&tmp_file_name) {
        Ok(fs) => fs,
        Err(err) => return Err(err.to_string()),
    };

    let mut cnt = 0;
    let mut size = 0;
    for line in &lines {
        let res = req.get(line)?;

        if SHOW_LOG {
            let mut line = String::from(line);

            if line.len() > 20 {
                let left = &line[..20];
                let right = &line[line.len() - 20..];
                line = format!("{left}....{right}");
            }
            cnt += 1;
            println!(
                "downloading [{} {:3}/{:3}] {line}",
                md5_str.light_green(),
                cnt.to_string().light_yellow(),
                lines.len().to_string().light_yellow()
            );
        };
        match fs.write(&res.body) {
            Err(err) => return Err(err.to_string()),
            Ok(size1) => size += size1,
        }
    }

    std::fs::rename(&tmp_file_name, &file_name);
    Ok(size)
}

pub fn threadify_download_vec(
    client: Arc<HttpClient>,
    urls: &[String],
    file: &mut File,
) -> Result<usize, String> {
    let mut threads = Vec::new();

    for url in urls {
        let client = Arc::clone(&client);
        let url = url.clone();
        threads.push(thread::spawn(move || -> Result<Vec<u8>, String> {
            let req = client.get(&url)?;
            Ok(req.body)
        }));
    }

    let mut cnt = 0;
    for thread in threads {
        let body = match thread.join() {
            Ok(body) => body?,
            Err(err) => return Err("thread join failed".to_string()),
        };
        cnt += match file.write(&body) {
            Ok(size) => size,
            Err(err) => return Err(err.to_string()),
        };
    }

    Ok(cnt)
}

pub fn threadify_download(
    url: &str,
    file_name: &str,
    threads: usize,
    proxy: Option<&str>,
    headers: HashMap<String, String>,
) -> Result<usize, String> {
    let mut client = HttpClient::new();
    if let Some(proxy) = proxy {
        client.proxy(proxy);
    }
    for (key, value) in headers {
        client.header(key, value);
    }

    let req = client.get(url)?;

    let urls = parse(url, String::from_utf8_lossy(&req.body).to_string().as_str());

    let mut begin = 0;

    let begin_time = time::Instant::now();
    println!("begin download all size {}", urls.len());

    let tmp_file_name = format!("{file_name}.downloading");
    let mut file = File::create(&tmp_file_name).unwrap();

    let client = Arc::new(client);
    let mut cnt = 0;
    while begin < urls.len() {
        let end = min(begin + threads, urls.len());
        let part = urls[begin..end].to_vec();
        let msg = format!("[{} .. {}]", begin, end);
        println!("downloading {}", msg.clone().light_green());
        begin += threads;
        cnt += threadify_download_vec(Arc::clone(&client), &part, &mut file)?;

        let span = time::Instant::now() - begin_time;
        println!(
            "downloaded  {} time used {} secs",
            msg.light_green(),
            span.as_secs().to_string().light_green()
        );
    }

    std::fs::rename(&tmp_file_name, &file_name);
    let span = time::Instant::now() - begin_time;
    println!(
        "download complete {} bytes use time {} secs",
        cnt,
        span.as_secs().to_string().light_green()
    );
    Ok(cnt)
}
