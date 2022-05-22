pub enum Proxy {
    Socks5(String),
    Http(String),
    None,
}

impl Proxy {
    pub fn new(proxy: &str) -> Self {
        let proxy = proxy.trim();
        if !proxy.contains("://") {
            return Self::Http(proxy.to_string());
        }

        let mut proxy = proxy.split("://");
        let proxy_type = proxy.next().unwrap();
        let addr = proxy.next().unwrap().to_string();

        if proxy_type.to_lowercase() == "http" {
            Self::Http(addr)
        } else {
            Self::Socks5(addr)
        }
    }
}

impl Default for Proxy {
    fn default() -> Self {
        Proxy::None
    }
}
