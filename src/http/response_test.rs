use serde_derive::Serialize;

use super::*;

fn def_res() -> Response {
    Response::new(
        ResLine {
            code: 200,
            code_line: "OK".to_string(),
            version: "HTTP/1.1".to_string(),
        },
        HashMap::new(),
        Vec::new(),
    )
}

#[test]
fn test_res_default() {
    let res = def_res();
    let str1: String = res.to_string();
    assert_eq!("HTTP/1.1 200 OK\r\n\r\n", str1);
}

#[test]
fn test_res_with_body() {
    let mut res = def_res();
    res.body("i don't know what should be there!");
    let str1: String = res.to_string();
    assert!(str1.contains("HTTP/1.1 200 OK\r\n"));
    assert!(str1.contains("\r\n\r\ni don't know what should be there!"));
}

#[test]
fn test_res_with_json() {
    #[derive(Serialize)]
    struct A {
        name: String,
        value: String,
    }

    let mut res = def_res();
    res.body_json(A {
        name: String::from("aniki"),
        value: String::from("what is value!."),
    });
    let str1: String = res.to_string();
    assert!(str1.contains("\r\n\r\n{\"name\":\"aniki\",\"value\":\"what is value!.\"}"));
}
