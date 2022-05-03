use crate::download::*;
use actix_web::{get, web, App, HttpServer, Responder};
use serde_derive::{Deserialize, Serialize};
use std::collections::hash_map::DefaultHasher;
use std::collections::HashMap;
use std::error::Error;
use std::hash::{Hash, Hasher};

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

async fn m3u8(post: web::Json<Post>) -> impl Responder {
    let file_name = gen_file_name(&post.url);
    download_m3u8(&post.url, &file_name, &post.headers).await;
    "download finished"
}

pub async fn run() -> Result<(), std::io::Error> {
    println!("server started in :2000");
    HttpServer::new(|| {
        App::new()
            .service(web::resource("/").route(web::get().to(index)))
            .service(web::resource("m3u8").route(web::post().to(m3u8)))
    })
    .bind(("0.0.0.0", 2000))?
    .run()
    .await
}
