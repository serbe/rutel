extern crate rutel;

use rutel::bot;
use std::env;

#[test]
fn get_token() {
    let token = env::var("TOKEN");
    assert!(token.is_ok());
}

#[test]
fn test_get_me() {
    let token: String = env::var("TOKEN").unwrap();
    let mut b = bot::Bot::new(&token, "127.0.0.1:9050");
    let u = b.get_me();
    println!("{:?}", u);
}
