use std::{collections::HashMap, io::Write};

use crate::{http::http_client::HttpClient, m3u8::parse};

const SHOW_LOG: bool = true;

pub fn download(
    url: &str,
    file_name: &str,
    proxy: Option<&str>,
    headers: HashMap<String, String>,
) -> Result<(), String> {
    let mut req = HttpClient::new();
    if let Some(prox) = proxy {
        req.proxy(prox);
    }

    let res = req.get(url)?;

    let lines = parse(url, &res.body_str());

    let tmp_file_name = format!("{file_name}.downloading");
    println!("create file {tmp_file_name}");
    let mut fs = match std::fs::File::create(&tmp_file_name) {
        Ok(fs) => fs,
        Err(err) => return Err(err.to_string()),
    };

    let mut cnt = 0;
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
            println!("downloading [{}/{}] {line}", cnt, lines.len());
        };
        if let Err(err) = fs.write(&res.body) {
            return Err(err.to_string());
        }
    }

    std::fs::rename(&tmp_file_name, &file_name);
    Ok(())
}
