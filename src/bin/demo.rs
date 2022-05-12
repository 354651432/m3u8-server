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
    m3u8::{
        download::{self, threadify_download},
        parse,
    },
};

fn main() {
    let url = "https://rtb0.doubleverify.com/verify.js?jsCallback=__verify_callback_483108966140&jsTagObjCallback=__tagObject_callback_483108966140&num=6&ctx=589953&cmp=27004562&plc=328259177&sid=6895402&advid=&adsrv=&unit=728x90&isdvvid=&uid=483108966140&tagtype=&adID=&app=&sup=&isovv=0&gmnpo=&crt=&sfe=1&brid=1&brver=&bridua=3&dup=null&srcurlD=0&ssl=1&refD=1&htmlmsging=1&m1=15&noc=4&fcifrms=3&brh=1&vavbkt=&lvvn=28&dvp_idcerr=undefined&ver=150&eparams=DC4FC%3Dl9EEADTbpTauTauDE24%3C%40G6C7%3D%40H%5D4%40%3ETauU2%3F4r92%3A%3Fl9EEADTbpTauTauDE24%3C%40G6C7%3D%40H%5D4%40%3ETar9EEADTbpTauTau_2f666g2d5_caaa6%604bc_da5__d322d2%5DD2767C2%3E6%5D8%40%408%3D6DJ%3F5%3A42E%3A%40%3F%5D4%40%3E&dvp_exetime=10.20&callbackName=__verify_callback_483108966140";

    // let size = 17usize;

    let mut req = HttpClient::new();
    req.proxy("127.0.0.1:10808");
    let res = req.get(url).unwrap();
    println!("{:#?} {:#?}", res.res, res.headers);
}
