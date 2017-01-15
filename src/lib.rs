#![deny(missing_docs, warnings)]

//! Request logging middleware for Iron

extern crate iron;
#[macro_use] extern crate log;
extern crate time;

use iron::{AroundMiddleware, Handler, IronResult, Request, Response};
use iron::typemap::Key;

use format::FormatText::{Str, Method, URI, Status, ResponseTime, RemoteAddr, RequestTime};
use format::{Format, FormatText};

pub mod format;

/// `Middleware` for logging request and response info to the terminal.
pub struct Logger {
    format: Option<Format>
}

impl Logger {
    /// Create a  `Logger` middleware with the specified `format`. If a `None` is passed in, uses the default format:
    ///
    /// ```ignore
    /// {method} {uri} -> {status} ({response-time} ms)
    /// ```
    ///
    /// This should be wrapped around other middlewares, by doing something like this:
    ///
    /// ```ignore
    /// let mut chain = Chain::new(handler);
    /// // link other middlewares here...
    /// let handler = Logger::new(None).around(chain);
    /// ```
    pub fn new(format: Option<Format>) -> Logger {
        Logger { format: format }
    }
}

struct LoggerHandler {
    format: Option<Format>,
    handler: Box<Handler>,
}

impl AroundMiddleware for Logger {
    fn around(self, handler: Box<Handler>) -> Box<Handler> {
        Box::new(LoggerHandler {
            format: self.format,
            handler: handler,
        })
    }
}

impl Handler for LoggerHandler {
    fn handle(&self, req: &mut Request) -> IronResult<Response> {
        self.initialise(req);
        match self.handler.handle(req) {
            Ok(res) => {
                try!(self.log(req, &res));
                Ok(res)
            }
            Err(err) => {
                try!(self.log(req, &err.response));
                Err(err)
            }
        }
    }
}

struct StartTime;
impl Key for StartTime { type Value = time::Tm; }

impl LoggerHandler {
    fn initialise(&self, req: &mut Request) {
        req.extensions.insert::<StartTime>(time::now());
    }

    fn log(&self, req: &mut Request, res: &Response) -> IronResult<()> {
        let entry_time = *req.extensions.get::<StartTime>().unwrap();

        let response_time = time::now() - entry_time;
        let response_time_ms = (response_time.num_seconds() * 1000) as f64 + (response_time.num_nanoseconds().unwrap_or(0) as f64) / 1000000.0;
        let Format(format) = self.format.clone().unwrap_or_default();

        {
            let render = |text: &FormatText| {
                match *text {
                    Str(ref string) => string.clone(),
                    Method => format!("{}", req.method),
                    URI => format!("{}", req.url),
                    Status => res.status
                        .map(|status| format!("{}", status))
                        .unwrap_or("<missing status code>".to_owned()),
                    ResponseTime => format!("{} ms", response_time_ms),
                    RemoteAddr => format!("{}", req.remote_addr),
                    RequestTime => format!("{}", entry_time.strftime("%Y-%m-%dT%H:%M:%S.%fZ%z").unwrap()),
                }
            };

            let lg = format.iter().map(|unit| render(&unit.text)).collect::<Vec<String>>().join("");
            info!("{}", lg);
        }

        Ok(())
    }
}
