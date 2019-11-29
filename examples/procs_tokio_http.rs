#![feature(await_macro, async_await, futures_api)]

#[macro_use]
extern crate tokio;
extern crate hyper;

use tokio::prelude::*;
use hyper::Client;

use std::time::Duration;
use std::str;

fn main() {
    let client = Client::new();
    let uri = "http://httpbin.org/ip".parse().unwrap();
    let response = client.get(uri);

    let response_is_ok = response
        .and_then(|resp| {
            println!("Status: {}", resp.status());
            Ok(())
        });

    // tokio::run(response_is_ok);
}
