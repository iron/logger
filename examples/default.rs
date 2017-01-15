//! Example of a simple logger
extern crate iron;
extern crate logger;
extern crate env_logger;

use iron::prelude::*;
use logger::Logger;

// Logger has a default formatting of the strings printed
// to console.
fn main() {
    env_logger::init().unwrap();

    let chain = Chain::new(no_op_handler);

    // Wrap logger around the rest of your middleware.
    let logger = Logger::new(None).around(chain);

    println!("Run `RUST_LOG=logger=info cargo run --example default` to see logs.");
    match Iron::new(logger).http("127.0.0.1:3000") {
        Result::Ok(listening) => println!("{:?}", listening),
        Result::Err(err) => panic!("{:?}", err),
    }
}

fn no_op_handler(_: &mut Request) -> IronResult<Response> {
    Ok(Response::with(iron::status::Ok))
}
