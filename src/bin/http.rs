#![allow(unused)]

use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
    thread::{self, JoinHandle},
};

use colorful::Colorful;
use serde_derive::{Deserialize, Serialize};
use work::{config::get_config, http::http_listener::HttpListener};

fn main() {
    let config = get_config();
    let map = Arc::new(Mutex::new(HashMap::new()));

    let bind = format!("http://{}/", &config.bind);
    println!("server started at {}", bind.light_green().bold());
    for mut ctx in HttpListener::bind(&config.bind) {
        let map = Arc::clone(&map);
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

                    let result = download(
                        String::from(&req_data.url),
                        req_data.headers,
                        req_data.title,
                    )
                    .join()
                    .unwrap();
                    let mut mutex = map.lock().unwrap();
                    if result {
                        mutex.insert(String::from(&req_data.url), true);
                    } else {
                        mutex.remove(&req_data.url);
                    }
                });
            } else {
                ctx.res.body("<h1> it works");
            }
        });
    }
}

fn download(url: String, headers: HashMap<String, String>, title: String) -> JoinHandle<bool> {
    let config = get_config();
    let title = title.replace(" ", "_");
    thread::spawn(move || {
        match work::download::download(
            &url,
            format!("result/{title}.rs.ts").as_str(),
            Some(&config.proxy),
            headers,
        ) {
            Ok(_) => true,
            Err(err) => {
                println!("download err: {}", err.light_yellow().bold());
                false
            }
        }
    })
}

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
