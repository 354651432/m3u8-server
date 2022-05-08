use crate::config::get_config;
use crate::download::*;
use actix_web::{web, App, HttpServer, Responder};
use serde_derive::{Deserialize, Serialize};
use std::collections::hash_map::DefaultHasher;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::sync::Mutex;

#[derive(Deserialize, Serialize, Debug)]
struct Post {
    headers: HashMap<String, String>,
    url: String,
}

fn gen_file_name(url: &str) -> String {
    let mut s = DefaultHasher::new();
    url.hash(&mut s);
    let hash = s.finish();
    format!("result/rs-{}.ts", hash)
}

async fn index() -> impl Responder {
    String::from("it works")
}

struct Cache {
    map: Mutex<Vec<String>>,
}

impl Cache {
    fn new() -> Cache {
        Cache {
            map: Mutex::new(vec![]),
        }
    }

    fn insert(&mut self, key: &str) {
        let mut vec = self.map.lock().unwrap();
        vec.push(String::from(key))
    }

    fn check(&self, key: &str) -> bool {
        for it in self.map.lock().unwrap().iter() {
            if it == key {
                return true;
            }
        }
        false
    }

    fn show(&self) {
        println!("show begin->");
        for it in self.map.lock().unwrap().iter() {
            println!("0: {}", it)
        }
    }
}

async fn m3u8(post: web::Json<Post>, cache: web::Data<Mutex<Cache>>) -> impl Responder {
    let mut cache = cache.lock().unwrap();

    cache.show();

    if cache.check(&post.url) {
        return "downloading";
    }

    cache.insert(&post.url);
    // drop(mutex);

    let file_name = gen_file_name(&post.url);

    let _ = download_m3u8(&post.url, &file_name, &post.headers).await;
    // if data.insert(post.url.clone(), true).is_none() {
    //     eprintln!("update failed");
    // }
    "download finished"
}

pub async fn run() -> Result<(), std::io::Error> {
    let config = get_config().unwrap();
    println!("server started in :{}", config.port);

    HttpServer::new(|| {
        let data = web::Data::new(Mutex::new(Cache::new()));
        App::new()
            .app_data(data.clone())
            .service(web::resource("/").route(web::get().to(index)))
            .service(web::resource("m3u8").route(web::post().to(m3u8)))
    })
    .bind((&config.host[..], config.port))?
    .run()
    .await
}
