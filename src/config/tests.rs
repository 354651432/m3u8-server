use super::*;

#[test]
fn test_get_conf() {
    let conf = get_config().unwrap();
    println!("{:?}", conf)
}
