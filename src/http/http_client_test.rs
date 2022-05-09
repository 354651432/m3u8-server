use super::*;

#[test]
fn test_url() {
    let url = Url::new("http://www.baidy.com/fuck?time=9999");
    let url = url.unwrap();

    assert_eq!(url.host, "www.baidy.com");
    assert_eq!(url.proto, "http");
    assert_eq!(url.path, "/fuck?time=9999");
}

#[test]
fn test_get() {
    // let url = "http://127.0.0.1:8080/env.aniki";
    let url = "http://127.0.0.1:2022/fuck";
    let res = HttpClient::new().get(url).unwrap();

    // panic!("{:?}", res.headers);
}

#[test]
fn https_get_test() {
    let url = "https://stackoverflow.com/questions/42503296/value-does-not-live-long-enough";
    let res = HttpClient::new().get(url).unwrap();
}
