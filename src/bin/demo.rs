#![allow(unused)]
use std::{
    cmp::min,
    collections::HashMap,
    fs::{self, File},
    io::{Seek, SeekFrom, Write},
    sync::Arc,
    thread,
};

use std::time;
use work::{
    http::http_client::HttpClient,
    m3u8::{download::threadify_download, parse},
};

fn main() {
    let url = "https://cdn77-vid.xvideos-cdn.com/41ERCH4AYGYEQBDLK8mPmg==,1652249953/videos/hls/b9/09/ba/b909baf80a531a41cca52bb3c7878c33/hls-720p-fa822.m3u8";

    let size = 17usize;

    threadify_download(
        url,
        "test2.ts",
        size,
        Some("127.0.0.1:10808"),
        HashMap::new(),
    );
}
