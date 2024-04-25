mod commons;
mod dtos;
mod handlers;

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
            Some(info.method().to_string()),
            Some(info.path().to_string()),
            Some(info.status().as_u16()),
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

    let post_messages = warp::post()
        .and(warp::path("flatEntries"))
        .and(warp::path::end())
        .and(warp::body::json())
        .and_then(handlers::flat_entries_handlers::post_flat_entries);

    get_messages
        .or(post_messages)
        .with(cors)
        .with(log)
        .recover(commons::handle_errors::return_error)
}
pub async fn run() {
    let port = 3030;

    commons::logger::info(
        format!("Server starting on port {}", port),
        None,
        None,
        None,
    );

    let routes = build_routes().await;
    warp::serve(routes).run(([0, 0, 0, 0], port)).await;
}
