use std::{collections::HashMap, fmt::Write, io::Write as IoWrite};

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
