#![allow(dead_code)]
#![allow(doc_markdown)]
//#![allow(unused_attributes)]
//#![cfg_attr(feature = "clippy", feature(plugin))]
//#![cfg_attr(feature = "clippy", plugin(clippy))]

#![feature(plugin)]
#![plugin(clippy)]

#[macro_use]
extern crate rutel_derive;

#[macro_use]
extern crate serde_derive;

extern crate futures;
extern crate hyper;
extern crate hyper_tls;
extern crate serde;
extern crate serde_json;
extern crate tokio_core;

mod bot;
pub mod types;
pub mod params;

pub use self::params::*;

use bot::Bot;

fn main() {
    const TOKEN: &str = "123786483:AAFCmpVP5h491DK2Rjz50fW5cTO-N2iFmhU";

    let mut b: Bot = Bot::new(TOKEN);
    println!("{:?}", b);
    let u = b.get_me();
    println!("{:?}", u);
    let mut params = GetUpdatesParams::new();
    let u = b.get_updates(params.limit(Some(2)));
    println!("{:?}", u);
    let mut params = SendMessageParams::new(types::ChatID::from(94_983_903), String::from("Привет"));
    let u = b.send_message(&mut params);
    println!("{:?}", u);
}
