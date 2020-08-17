use std::env;

use dotenv;
use rutel::bot;

#[test]
fn get_token() {
    let token = dotenv::var("TOKEN");
    assert!(token.is_ok());
}

#[test]
fn test_get_me() {
    let token: String = dotenv::var("TOKEN").unwrap();
    let mut b = bot::Bot::new(&token, "127.0.0.1:9050");
    let u = b.get_me();
    println!("{:?}", u);
}
