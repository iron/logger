//! Example of logger with custom formatting
extern crate iron;
extern crate logger;
extern crate env_logger;

use iron::prelude::*;

use logger::Logger;
use logger::format::Format;

static FORMAT: &'static str =
    "Uri: {uri}, Method: {method}, Status: {status}, Duration: {response-time}, Time: {request-time}";

// This is an example of using a format string that can specify colors and attributes
// to specific words that are printed out to the console.
fn main() {
    env_logger::init().unwrap();

    let format = Format::new(FORMAT);
    let logger = Logger::new(Some(format.unwrap()))
    let hanlder = logger.around(no_op_handler);

    println!("Run `RUST_LOG=info cargo run --example formatstring` to see logs.");
    match Iron::new(handler).http("127.0.0.1:3000") {
        Result::Ok(listening) => println!("{:?}", listening),
        Result::Err(err) => panic!("{:?}", err),
    }
}

fn no_op_handler(_: &mut Request) -> IronResult<Response> {
    Ok(Response::with(iron::status::Ok))
}
