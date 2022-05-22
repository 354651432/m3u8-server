use std::{
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
};

fn main() {
    let str1 = "fds";
    // str1.hash(DefaultHasher)
    let mut hasher = DefaultHasher::default();
    str1.hash(&mut hasher);
    let hash = Hasher::finish(&hasher);

    println!("default hash value is {hash}")
}
