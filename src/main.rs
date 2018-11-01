#![allow(dead_code)]
//#![allow(doc_markdown)]
//#![allow(empty_line_after_outer_attr)]
//#![allow(unused_attributes)]

//#![feature(plugin)]
//#![plugin(clippy)]

#[macro_use]
extern crate rutel_derive;

#[macro_use]
extern crate serde_derive;

extern crate tsocks;
extern crate serde;
extern crate serde_json;

pub mod bot;
pub mod types;
pub mod params;

use bot::Bot;
use params::*;
// use crate::types;

fn main() {
    let token: String = String::from("260914752:AAHOFm8itRPcIg_i14ZvhduuLyXaF1dTXqA");

    let mut b: Bot = Bot::new(&token, "127.0.0.1:9050");
    println!("bot {:?}", b);
    let u = b.get_me();
    println!("get me {:?}", u);
    let mut params = GetUpdatesParams::new();
    //    // ;
    println!("GetUpdatesParams {:?}", params);
    let u = b.get_updates(params.limit(Some(2)));
    //        //    println!("{:?}", params.json());
    println!("get_updates {:?}", u);
    let mut params = SendMessageParams::new(
        types::ChatID::from(94_983_903),
        String::from("Привет"),
    );
    let u = b.send_message(&mut params);
    println!("send_message {:?}", u);
}