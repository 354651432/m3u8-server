#![allow(warnings, unused)]
use super::*;

#[test]
#[no_mangle]
fn test_parse_req() {
    let src = "get /api HTTP/1.1
    Host: ducl.cc
    Accept: application/json
    ";
    let mut stream = BufReader::new(src.as_bytes());

    let req = Request::read_from_stream(stream).unwrap();
    assert_eq!(req.body, None);
    assert_eq!(req.version, "HTTP/1.1");
    assert_eq!(req.method, "GET");
    assert_eq!(req.path, "/api");
    assert_eq!(req.headers["Host"], "ducl.cc");
    assert_eq!(req.headers["Accept"], "application/json");
}
