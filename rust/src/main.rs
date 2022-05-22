use std::{collections::HashMap, error::Error, net::SocketAddr};

use axum::{
    routing::{get, post},
    Json, Router,
};

use serde::Deserialize;
use serde_json::{json, Value};
use tracing::Level;
use tracing_subscriber::FmtSubscriber;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::DEBUG)
        .finish();

    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");
    let app = Router::new()
        .route("/", get(home))
        .route("/m3u8", post(m3u8));

    let addr = SocketAddr::from(([0, 0, 0, 0], 2022));

    tracing::debug!("listening on {}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();

    Ok(())
}

async fn home() -> &'static str {
    "it works"
}

#[derive(Deserialize, Debug)]
struct ReqJson {
    url: String,
    headers: HashMap<String, String>,
    title: String,
}

async fn m3u8(
    Json(ReqJson {
        url,
        headers,
        title,
    }): Json<ReqJson>,
) -> Json<Value> {
    match web::download(&url, Some(headers), Some(title)).await {
        web::DownloadStatus::Downloding => Json(json! ({"code":1})),
        _ => Json(json! ({"code":0})),
    }
}
