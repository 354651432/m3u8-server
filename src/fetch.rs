use std::collections::HashMap;

use regex::Regex;
use serde_derive::Deserialize;

#[cfg(test)]
#[path = "fetch_test.rs"]
mod test;

#[derive(Deserialize, Debug)]
pub struct FetchOpt {
    pub headers: Option<HashMap<String, String>>,
    pub referrer: Option<String>,
    pub method: String,
    pub mode: String,
    pub credentials: String,
}

#[derive(Deserialize, Debug)]
pub struct FetchObj {
    pub url: String,
    pub option: FetchOpt,
}

impl FetchObj {
    pub fn new(str1: &str) -> Self {
        let mut obj: Self = serde_json::from_str(str1).unwrap();

        let mut opt = obj.option;
        let mut headers = match opt.headers {
            Some(mut headers) if opt.referrer.is_some() => {
                headers.insert(
                    String::from("referrer"),
                    String::from(opt.referrer.as_ref().unwrap()),
                );
                headers
            }
            _ if opt.referrer.is_some() => {
                let mut headers = HashMap::new();
                headers.insert(
                    String::from("referrer"),
                    String::from(opt.referrer.as_ref().unwrap()),
                );
                headers
            }
            _ => HashMap::new(),
        };

        Self {
            url: obj.url,
            option: FetchOpt {
                headers: Some(headers),
                method: opt.method.clone(),
                mode: opt.mode.clone(),
                credentials: opt.credentials.clone(),
                referrer: opt.referrer.as_ref().cloned(),
            },
        }
    }

    pub fn from_fetch_string(content: &str) -> Self {
        let content = Regex::new(r"^\s*fetch\(")
            .unwrap()
            .replace(content, "[")
            .to_string();

        let content = Regex::new(r"\);\s*$")
            .unwrap()
            .replace(&content, "]")
            .to_string();

        // println!("\n\ncontent is {content}");
        Self::new(&content)
    }
}
