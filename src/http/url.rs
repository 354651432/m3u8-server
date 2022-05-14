use regex::Regex;

#[derive(Debug)]
pub struct Url {
    pub proto: String,
    pub host: String,
    pub port: usize,
    pub path: String,
}

impl Url {
    //r"'(?P<title>[^']+)'\s+\((?P<year>\d{4})\)"
    pub fn new(url: &str) -> Option<Self> {
        let reg = Regex::new(r"(?P<proto>https?)://(?P<host>[^/]+)(?P<port>:\w+)?(?P<path>/.+)?")
            .unwrap();
        let mch = reg.captures(url)?;

        let proto = mch.name("proto")?.as_str().to_lowercase();
        Some(Self {
            host: mch.name("host")?.as_str().to_owned(),
            port: match mch.name("port") {
                Some(port) => {
                    let port = port.as_str()[1..].parse().ok()?;
                    port
                }
                None => {
                    if &proto == "http" {
                        80
                    } else {
                        443
                    }
                }
            },
            proto,
            path: match mch.name("path") {
                Some(path) => path.as_str().to_owned(),
                None => "/".to_owned(),
            },
        })
    }

    pub fn to_host(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }
}
