#![allow(unused)]

use std::{
    collections::HashMap,
    ffi::OsString,
    fmt::Write,
    fs,
    io::Read,
    sync::{Arc, Mutex},
    thread,
    time::Instant,
};

use clap::{Args, Command, Parser};
use colorful::Colorful;
use config::Config;
use serde_derive::{Deserialize, Serialize};

use crate::{
    fetch::FetchObj,
    http::http_listener::HttpListener,
    m3u8::{download::threadify_download, gen_file_name},
};
pub mod config;
pub mod fetch;
pub mod http;
pub mod m3u8;
// pub mod server;

fn bind(config: Config) {
    #[derive(Deserialize)]
    struct ReqData {
        url: String,
        headers: HashMap<String, String>,
        title: String,
    }

    #[derive(Serialize)]
    struct ResData {
        code: usize,
    }

    let dirname = "results";
    if let Err(err) = fs::create_dir(dirname) {}

    let map = Arc::new(Mutex::new(HashMap::new()));

    let bind = format!("http://{}/", config.bind.as_ref().unwrap());
    println!("server started at {}", bind.light_green().bold());
    let config = Arc::new(config);
    for mut ctx in HttpListener::bind(config.bind.as_ref().unwrap().as_str()) {
        let map = Arc::clone(&map);
        let config = Arc::clone(&config);
        thread::spawn(move || {
            // println!("{:?}", ctx.req.req);
            if &ctx.req.req.method == "POST" && &ctx.req.req.path == "/m3u8" {
                thread::spawn(move || {
                    let mut mutex = map.lock().unwrap();
                    let req_data: ReqData = serde_json::from_slice(&ctx.req.body).unwrap();
                    if mutex.get(&req_data.url).is_some() {
                        ctx.res.body_json(ResData { code: 1 });
                        return;
                    }

                    mutex.insert(String::from(&req_data.url), false);
                    ctx.res.body_json(ResData { code: 0 });
                    drop(ctx);
                    drop(mutex);

                    let file_name = format!("{dirname}/{}.ts", &req_data.title);
                    let proxy = match config.proxy.as_ref() {
                        Some(proxy) => Some(proxy.as_str()),
                        None => None,
                    };
                    let result = threadify_download(
                        &req_data.url,
                        &file_name,
                        config.threads,
                        proxy,
                        req_data.headers,
                    );
                    let mut mutex = map.lock().unwrap();
                    if let Ok(_) = result {
                        mutex.insert(String::from(&req_data.url), true);
                    } else {
                        mutex.remove(&req_data.url);
                    }
                });
            } else {
                ctx.res.body("<h1> it works</h1>");
                let map = map.lock().unwrap();
                ctx.res.body(
                    format!(
                        r"<ul><li>tasks {}</li><li>downlading {}</li><li>complete {}</li>",
                        map.len(),
                        map.iter().filter(|(key, value)| **value == true).count(),
                        map.iter().filter(|(key, value)| **value == false).count(),
                    )
                    .as_str(),
                );
            }
        });
    }
}

fn stdin(config: Config) {
    println!(
        "{}",
        "type fetch code copied from chrome dev bar and enter return and ctrl-d ->".light_blue()
    );
    let mut buf = Vec::new();
    std::io::stdin().read_to_end(&mut buf).unwrap();
    println!("{}", "code readed".light_blue());

    let obj = FetchObj::from_fetch_string(String::from_utf8_lossy(&buf).to_string().as_str());

    let time = Instant::now();
    let file_name = gen_file_name(&obj.url);

    let mut headers = match obj.option.headers {
        Some(headers) => headers,
        None => HashMap::default(),
    };

    let proxy = match config.proxy.as_ref() {
        Some(proxy) => Some(proxy.as_str()),
        None => None,
    };
    match threadify_download(&obj.url, &file_name, config.threads, proxy, headers) {
        Err(err) => {
            eprintln!("{}", err.light_yellow().bold());
            return;
        }
        Ok(size) => {
            let span = Instant::now() - time;

            let msg = format!(
                "complete downloaded {} size:{} secs:{}",
                file_name,
                size,
                span.as_secs(),
            );
            println!("{}", msg.light_green().bold())
        }
    }
}

fn make_filename(config: &Config) -> String {
    if let Some(file) = &config.file {
        return file.clone();
    }

    let md5 = md5::compute(config.url.as_ref().unwrap());
    let mut filename = String::new();
    for c in md5.0 {
        write!(&mut filename, "{:x}", c);
    }
    filename + ".ts"
}

fn download(config: Config) {
    let time = Instant::now();
    let proxy = match config.proxy.as_ref() {
        Some(proxy) => Some(proxy.as_str()),
        None => None,
    };

    let file_name = make_filename(&config);
    match threadify_download(
        &config.url.unwrap(),
        &file_name,
        config.threads,
        proxy,
        HashMap::default(),
    ) {
        Err(err) => {
            eprintln!("{}", err.light_yellow().bold());
            return;
        }
        Ok(size) => {
            let span = Instant::now() - time;

            let msg = format!(
                "complete downloaded {} size:{} secs:{}",
                file_name,
                size,
                span.as_secs(),
            );
            println!("{}", msg.light_green().bold())
        }
    }
}

pub fn run() {
    let mut config = Config::parse();
    if let Some(_) = config.bind {
        bind(config);
    } else if config.stdin {
        stdin(config);
    } else if let Some(_) = config.url {
        download(config);
    } else {
        // println!("--help for a help")
    }
}
