use serde_derive::Serialize;

use super::*;

#[test]
fn test_res_default() {
    let res = Response::default();
    let str1: String = res.to_string();
    assert_eq!("HTTP/1.1 200 OK\r\n\r\n", str1);
}

#[test]
fn test_res_with_body() {
    let mut res = Response::default();
    res.body("i don't know what should be there!");
    let str1: String = res.to_string();
    assert_eq!("HTTP/1.1 200 OK\r\nContent-Type: text/html\r\nContent-Length: 34\r\n\r\ni don't know what should be there!", str1);
}

#[test]
fn test_res_with_json() {
    #[derive(Serialize)]
    struct A {
        name: String,
        value: String,
    }

    let mut res = Response::default();
    res.body_json(A {
        name: String::from("aniki"),
        value: String::from("what is value!."),
    });
    let str1: String = res.to_string();
    assert!(str1.contains("\r\n{\"name\":\"aniki\",\"value\":\"what is value!.\"}"));
}
