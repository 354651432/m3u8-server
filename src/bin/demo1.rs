use std::cmp::min;

fn main() {
    let urls: Vec<u32> = (0..70).collect();

    let size = 15usize;
    let mut begin = 0;

    while begin < urls.len() {
        let end = min(begin + size, urls.len());
        let part = urls[begin..end].to_vec();
        println!("{part:#?}");
        println!("downloading from {} to {}", begin, end);
        begin += size;
    }
}
