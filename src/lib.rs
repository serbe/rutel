#![allow(dead_code)]
//#![allow(doc_markdown)]
//#![allow(empty_line_after_outer_attr)]
//#![allow(unused_attributes)]

#[macro_use]
extern crate rutel_derive;

#[macro_use]
extern crate serde_derive;

extern crate serde;
extern crate serde_json;
extern crate tsocks;

pub mod bot;
pub mod types;
