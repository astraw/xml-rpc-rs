#![recursion_limit = "1024"]

extern crate base64;
#[macro_use]
extern crate error_chain;
extern crate futures;
extern crate hyper;
#[macro_use]
extern crate lazy_static;
extern crate regex;
#[macro_use]
extern crate serde;
#[cfg(test)]
extern crate serde_bytes;
#[macro_use]
extern crate serde_derive;
extern crate serde_xml_rs;
extern crate tokio_core;
extern crate xml;

pub mod client;
pub mod error;
pub mod server;
mod xmlfmt;

pub use client::{call, call_value, Client};
pub use server::Server;
pub use xmlfmt::{from_params, into_params, Call, Fault, Params, Response, Value};
