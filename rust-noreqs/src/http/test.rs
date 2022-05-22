use core::panic;

use super::*;

#[test]
fn test_parse_headers() {
    let mut lines = Vec::new();
    lines.push("Content-Type: text/json".to_string());
    lines.push("Content-Length: 500".to_string());
    let headers = parse_headers(&lines);

    let leaders_str = header_tostr(&headers);
    println!("{}", leaders_str);
}

#[test]
fn test_parse_reqline() {
    let line = "HTTP/1.1 202 OK";
    let res = parse_resline(line).unwrap();
    // println!("{:?}", res);
    assert_eq!(res.code, 202);
    assert_eq!(res.version, "HTTP/1.1");
    assert_eq!(res.code_line, "OK");
}

#[test]
fn test_parse_resline() {
    let line = "get /fuck HTTP/2.0";
    let req = parse_reqline(line).unwrap();
    println!("{:?}", req);
    assert_eq!(req.method, "GET");
    assert_eq!(req.version, "HTTP/2.0");
    assert_eq!(req.path, "/fuck");
}

#[test]
fn test_getkey_ignorecase() {
    let mut map = HashMap::new();
    map.insert("content-length".to_string(), "200".to_string());
    map.insert("content-type".to_string(), "text/json".to_string());
    map.insert("content-encoding".to_string(), "utf9".to_string());

    let value = getkey_ignorecase("Content-Length", &map).unwrap();
    assert_eq!(value, "200");

    let value = getkey_ignorecase("Content-type", &map).unwrap();
    assert_eq!(value, "text/json");
}
