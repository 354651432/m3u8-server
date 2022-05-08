#[allow(unused)]
use std::borrow::BorrowMut;
use std::collections::HashMap;
#[allow(unused)]
use std::fmt::Display;
#[allow(unused)]
use std::ops::Deref;
#[allow(unused)]
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

#[allow(unused)]
fn main() {
    let arc = Arc::new(Mutex::new(0));
    let arc1 = Arc::clone(&arc);
    let arc2 = Arc::clone(&arc);

    let t1 = thread::spawn(move || {
        println!("thread 1 beginning");
        for i in 0..10 {
            let mut mobj = arc1.lock().unwrap();
            println!("thread 1: {} {}", i, mobj);
            drop(mobj);
            thread::sleep(Duration::from_secs(1));
        }
    });

    let t2 = thread::spawn(move || {
        println!("thread 2 beginning");
        for i in 10..20 {
            let mut mobj = arc2.lock().unwrap();
            println!("thread 2: {} {}", i, mobj);
            drop(mobj);
            thread::sleep(Duration::from_secs(1));
        }
    });

    t1.join().unwrap();
    t2.join().unwrap();
}

#[allow(unused)]
fn show(str1: &str, vec: &HashMap<i32, bool>) {
    println!("begin->");
    for it in vec.iter() {
        println!("{}:{}", str1, it.0)
    }
    println!("end->\n");
}

#[allow(unused)]
fn insert(vec: &mut HashMap<i32, bool>, i: i32) {
    vec.insert(i, true);
}
