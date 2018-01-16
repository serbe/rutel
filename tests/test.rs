extern crate rutel;

use rutel::bot::Bot;
use rutel::types;
use rutel::params::*;

use std::env;

#[test]
fn get_token() {
    let token = env::var("TOKEN");
    assert!(token.is_ok());
}

//#[test]
//fn test_get_me() {
//    let token = env::var("TOKEN").unwrap();
//    let TOKEN = &token.clone();
//
//    let mut b: Bot = Bot::new(TOKEN);
//    println!("{:?}", b);
//    let u = b.get_me();
//    println!("{:?}", u);
//    let mut params = GetUpdatesParams::new();
//    // ;
//        // println!("{:?}", params);
//    let u = b.get_updates(params.limit(Some(2)));
//        //    println!("{:?}", params.json());
//        println!("{:?}", u);
//    let mut params = SendMessageParams::new(types::ChatID::from(94_983_903), String::from("Привет"));
//    let u = b.send_message(&mut params);
//    println!("{:?}", u);
//        //    println!("{:?}", params.json());
//        //    let mut params = ForwardMessageParams::new(types::ChatID::from(94_983_903), types::ChatID::from(94_983_903), 68);
//        //    let u = b.forward_message(params.disable_notification(true).json());
//        //    println!("{:?}", u);
//        //    println!("{:?}", params.json());
//}
