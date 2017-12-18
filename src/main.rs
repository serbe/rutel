#![allow(non_camel_case_types)]
#![allow(dead_code)]
#![allow(unused_attributes)]
#![cfg_attr(feature = "clippy", feature(plugin))]
#![cfg_attr(feature = "clippy", plugin(clippy))]

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
    const TOKEN: &str = "123786433:AAFCmpVP5h491DK2Rjz5ofW5cTO-N2iFmhU";

    let mut b: Bot = Bot::new(TOKEN);
    println!("{:?}", b);
    let u = b.get_me();
    println!("{:?}", u);
    let mut params = GetUpdatesParams::new();
    // ;
    // println!("{:?}", params);
    let u = b.get_updates(params.limit(Some(2)).json());
    //    println!("{:?}", params.json());
    println!("{:?}", u);
    //    let params = SendMessageParams::new(types::ChatID::from(94_983_903), String::from("Привет"));
    //    let u = b.send_message(params.json());
    //    println!("{:?}", u);
    //    println!("{:?}", params.json());
    //    let mut params = ForwardMessageParams::new(types::ChatID::from(94_983_903), types::ChatID::from(94_983_903), 68);
    //    let u = b.forward_message(params.disable_notification(true).json());
    //    println!("{:?}", u);
    //    println!("{:?}", params.json());
}
