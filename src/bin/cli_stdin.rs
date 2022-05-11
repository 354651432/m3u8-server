use std::io::Read;

fn main() {
    let mut buf = Vec::new();
    std::io::stdin().read_to_end(&mut buf).unwrap();
    println!("{}", String::from_utf8_lossy(&buf).to_string());
}
