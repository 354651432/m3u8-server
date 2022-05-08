use std::{convert::From, fmt::Display};

#[derive(Clone)]
struct A;
impl From<A> for String {
    fn from(_: A) -> String {
        "this is a".to_owned()
    }
}

// impl Into<String> for A {
//     fn into(self) -> String {
//         "Into this is a".to_owned()
//     }
// }

impl Display for A {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", <String as From<A>>::from(self.clone()))
    }
}

fn f(obj: &A) {}

fn main() {
    // let a = A;
    // let str1 = a.to_string();
    // let str2: String = a.into();
    // println!("str1 {}", str1);
    // println!("str2 {}", str2);

    // let str1 = "123";
    // let i1: Option<usize> = str1.parse().ok();

    // println!("{}", i1.unwrap());
    let mut obj = A;
    let obj = &mut obj;
    f(obj);
}
