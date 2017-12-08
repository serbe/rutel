#![allow(non_camel_case_types)]
#![allow(dead_code)]

#[macro_use]
extern crate serde_derive;

extern crate serde;
extern crate serde_json;
extern crate futures;
extern crate hyper;
extern crate hyper_tls;
extern crate tokio_core;

mod bot;
pub mod types;
pub mod methods;

pub use self::methods::*;

use bot::Bot;

fn main() {
    const TOKEN: &'static str = "123786433:AAFCmpVP5h491DK2Rjz5ofW5cTO-N2iFmhU";

    let b: Bot = Bot::new(TOKEN);
//    println!("{:?}", b);
//    let u = b.get_me();
//    println!("{:?}", u);
    let mut params = getUpdatesParams::new();
    let u = b.get_updates(params.json());
    println!("{:?}", u);
}
