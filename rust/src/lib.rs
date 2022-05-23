use std::{
    collections::{hash_map::DefaultHasher, HashMap},
    fs::{self, File},
    hash::{Hash, Hasher},
    io::Write,
    sync::Mutex,
};

use hls_m3u8::MediaPlaylist;
use reqwest::Client;
use tokio::spawn;

struct Global {
    cache: Mutex<Vec<u64>>,
    client: Client,
}

const PROXY: &str = "socks5://127.0.0.1:10808";

lazy_static::lazy_static! {
    static ref GLOBAL: Global = Global::new();
}

impl Global {
    fn new() -> Self {
        let proxy = reqwest::Proxy::all(PROXY).unwrap();
        Self {
            cache: Default::default(),
            client: reqwest::Client::builder().proxy(proxy).build().unwrap(),
        }
    }

    fn hash(s: &str) -> u64 {
        let mut hasher = DefaultHasher::default();
        s.hash(&mut hasher);
        hasher.finish()
    }

    fn exists(&self, hash: u64) -> bool {
        self.cache.lock().unwrap().contains(&hash)
    }

    fn add(&self, hash: u64) {
        self.cache.lock().unwrap().push(hash)
    }
}

pub enum DownloadStatus {
    Downloding,
    Success,
    Err(Box<dyn std::error::Error>),
}

pub async fn download(
    url: &str,
    headers: Option<HashMap<String, String>>,
    title: Option<String>,
) -> DownloadStatus {
    let hash = Global::hash(url);
    if GLOBAL.exists(hash) {
        return DownloadStatus::Downloding;
    }
    GLOBAL.add(hash);

    let url = String::from(url);
    spawn(async move {
        macro_rules! eh {
            ($ex:expr) => {
                match $ex {
                    Ok(ok) => ok,
                    Err(err) => {
                        eprintln!("{err}");
                        return;
                    }
                }
            };
        }

        let client = &GLOBAL.client;

        let mut get_url = client.get(&url);
        if let Some(headers) = headers.as_ref() {
            for (key, value) in headers.iter() {
                get_url = get_url.header(key, value);
            }
        }

        let res = eh!(get_url.send().await);

        let m3u8 = eh!(res.text().await);
        let m3u8: MediaPlaylist = eh!(m3u8.parse());

        let parsed_url = eh!(url::Url::parse(&url));

        let hash = title.unwrap();
        let file_name = format!("download/{hash}.ts");
        let tmp_file_name = format!("{file_name}_downloading");
        let mut file = eh!(File::create(&tmp_file_name));

        let mut results = Vec::new();
        for (_, seg) in m3u8.segments.iter() {
            let mut get_url = client.get(eh!(parsed_url.join(seg.uri())));
            if let Some(headers) = headers.as_ref() {
                for (key, value) in headers.iter() {
                    get_url = get_url.header(key, value);
                }
            }

            let res = get_url.send();
            results.push(res);
        }

        for it in results {
            let res = eh!(it.await);
            eh!(file.write_all(eh!(&res.bytes().await)))
        }

        fs::rename(tmp_file_name, file_name).unwrap();
    });

    DownloadStatus::Success
}
