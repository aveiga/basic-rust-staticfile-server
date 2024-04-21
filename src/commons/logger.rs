use slog::{o, Drain, Logger};
use slog_async;
use slog_json;
use std::io;

pub fn info(message: String, method: Option<String>, path: Option<String>, status: Option<u16>) {
    higher_log("info".to_string(), message, method, path, status);
}

pub fn error(message: String, method: Option<String>, path: Option<String>, status: Option<u16>) {
    higher_log("error".to_string(), message, method, path, status);
}

pub fn debug(message: String, method: Option<String>, path: Option<String>, status: Option<u16>) {
    higher_log("debug".to_string(), message, method, path, status);
}

fn higher_log(
    level: String,
    message: String,
    method: Option<String>,
    path: Option<String>,
    status: Option<u16>,
) {
    let drain = slog_json::Json::new(io::stdout()).build().fuse();
    let drain = slog_async::Async::new(drain).build().fuse();

    let logger = Logger::root(drain, o!());

    match level.as_str() {
        "info" => {
            slog::info!(logger, "Request"; "message" => message, "method" => method.unwrap_or(String::from("")), "path" => path.unwrap_or(String::from("")), "status" => status.unwrap_or(0))
        }
        "error" => {
            slog::error!(logger, "Request"; "message" => message, "method" => method.unwrap_or(String::from("")), "path" => path.unwrap_or(String::from("")), "status" => status.unwrap_or(0))
        }
        "debug" => {
            slog::debug!(logger, "Request"; "message" => message, "method" => method.unwrap_or(String::from("")), "path" => path.unwrap_or(String::from("")), "status" => status.unwrap_or(0))
        }
        _ => info(message, method, path, status),
    }
}
