#[macro_use]
extern crate serde_derive;

extern crate serde;
extern crate serde_json;
extern crate futures;
extern crate hyper;
extern crate hyper_tls;
extern crate tokio_core;

//use std::io::{self, Write};
//use futures::{Future, Stream};
//use tokio_core::reactor::Core;

mod bot;
mod methods;
mod types;

use bot::Bot;

fn main() {
    const TOKEN: &'static str = "123786433:AAFCmpVP5h491DK2Rjz5ofW5cTO-N2iFmhU";

    let b: Bot = Bot::new(TOKEN);
    println!("{:?}", b);
//    let u = b.get_me();
//    println!("{:?}", u);
    let u = b.get_updates(methods::getUpdatesParams::new().json());
    println!("{:?}", u);
}
