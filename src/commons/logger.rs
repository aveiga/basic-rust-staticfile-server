use slog::{o, Drain, Logger};
use slog_async;
use slog_json;
use std::io;

pub fn info(message: String, method: Option<String>, path: Option<String>, status: Option<u16>) {
    let drain = slog_json::Json::new(io::stdout()).build().fuse();
    let drain = slog_async::Async::new(drain).build().fuse();

    let logger = Logger::root(drain, o!());

    slog::info!(logger, "Request"; "message" => message, "method" => method.unwrap_or(String::from("")), "path" => path.unwrap_or(String::from("")), "status" => status)
}
