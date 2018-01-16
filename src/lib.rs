#![allow(dead_code)]
#![allow(doc_markdown)]
//#![allow(unused_attributes)]

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

pub mod bot;
pub mod types;
pub mod params;
