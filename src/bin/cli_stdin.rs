use std::{collections::HashMap, io::Read, time::Instant};

use colorful::Colorful;
use work::{
    config::get_config,
    fetch::FetchObj,
    m3u8::{download, gen_file_name},
};

fn main() {
    let config = get_config();

    println!("type fetch code copied from chrome dev bar and enter return and ctrl-d ->");
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

    headers.insert(String::from("User-Agent"), config.user_agent);
    match download::threadify_download(
        &obj.url,
        &file_name,
        config.threads,
        Some(&config.proxy),
        headers,
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
