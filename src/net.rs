////use hyper::client::Client;
////use hyper::Client::{HttpConnector, Body};
//use tokio_core::reactor::Core;
//
//const API_URL: &'static str = "https://api.telegram.org/bot";
//
//pub fn new_core() -> Core {
//    Core::new().unwrap()
//}
//
////pub fn new_client(c: Core) -> Client<HttpConnector, Body> {
////    Client::new(&c.handle())
////}
//
//extern crate reqwest;
//extern crate serde_json;
//
//use std::io::Read;
//use serde_json::{Value};
//
//fn main() {
////    let uri: Uri = format!("https://api.telegram.org/bot{}/{}", "123786433:AAFCmpVP5h491DK2Rjz5ofW5cTO-N2iFmhU", "getMe").parse().unwrap();
//
//    let mut resp = reqwest::get("https://api.telegram.org/bot123786433:AAFCmpVP5h491DK2Rjz5ofW5cTO-N2iFmhU/getMe").unwrap();
//    assert!(resp.status().is_success());
//
//    let mut content = String::new();
//    resp.read_to_string(&mut content);
//    let v: Value = serde_json::from_str(&content).unwrap();
//    println!("{:?}", v);
//}
