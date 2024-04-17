mod commons;
mod handlers;

use std::ptr::null;

use serde_derive::{Deserialize, Serialize};
use warp::http::Method;
use warp::{Filter, Reply};

#[derive(Debug, Serialize, Deserialize)]
struct CustomLog {
    active: bool,
    username: String,
}

pub async fn build_routes() -> impl Filter<Extract = impl Reply> + Clone {
    let log = warp::log::custom(move |info| {
        commons::logger::info(
            "".to_string(),
            info.method().to_string(),
            info.path().to_string(),
            info.status().as_u16(),
        )
    });

    let cors = warp::cors().allow_any_origin().allow_methods(&[
        Method::PUT,
        Method::DELETE,
        Method::POST,
        Method::GET,
    ]);

    let get_messages = warp::get()
        .and(warp::path("flatEntries"))
        .and(warp::path::end())
        .and_then(handlers::flat_entries_handlers::get_flat_entries);
    //.recover(return_error)

    get_messages.with(cors).with(log)
}
pub async fn run() {
    let port = 3030;

    let log = CustomLog {
        active: true,
        username: "aveiga".to_string(),
    };

    commons::logger::info("cenas".to_string(), "", "".to_string(), 0);

    //slog::info!(logger, "Logging a complex object"; "field1" => log.active, "field2" => log.username);

    let routes = build_routes().await;
    warp::serve(routes).run(([0, 0, 0, 0], port)).await;
}
