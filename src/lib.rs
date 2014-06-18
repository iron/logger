#![crate_id = "logger"]
#![license = "MIT"]

//! Request logging middleware for Iron

extern crate iron;
extern crate time;

use iron::{Ingot, Alloy, Request, Response};
use iron::ingot::{Status, Continue};

use time::precise_time_ns;

use std::io::stdio::println;

/// `Ingot` for logging request and response info to the terminal.
/// `Logger` logs the request method, request URI, response status, and response
/// time in the format:
/// ```
/// {method} {uri} -> {status} ({response_time} ms)
/// ```
#[deriving(Clone)]
pub struct Logger {
    entry_time: u64
}

impl Logger {
    /// Create a new `Logger`.
    pub fn new() -> Logger {
        Logger { entry_time: 0u64 }
    }
}

enum Color {
    Red,
    Yellow,
    Green,
    Blue
}

impl<Rq: Request, Rs: Response> Ingot<Rq, Rs> for Logger {
    fn enter(&mut self, _req: &mut Rq, _res: &mut Rs, _alloy: &mut Alloy) -> Status {
        self.entry_time = precise_time_ns();
        Continue
    }
    fn exit(&mut self, req: &mut Rq, res: &mut Rs, _al: &mut Alloy) -> Status {
        let status = res.status();
        let status_color = match status.code() {
            _n @ 200..299 => Green, // Success
            _n @ 300..399 => Yellow, // Redirection
            _n @ 400..599 => Red, // Error
            _ => Blue // Information
        };
        let response_time_ms = (precise_time_ns() - self.entry_time) as f64 / 1000000.0;

        let mut output = String::new();
        output.push_str(format!("\x1B[1m{}\x1B[0m {} \x1B[1m->\x1B[0m ",
                                req.method(), req.uri()).as_slice());
        output.push_str(colorize(format!("{}", status), status_color).as_slice());
        output.push_str(format!(" ({} ms)", response_time_ms).as_slice());
        println(output.as_slice());

        Continue
    }
}
