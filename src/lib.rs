mod handlers;

use warp::http::Method;
use warp::{Filter, Reply};

pub async fn build_routes() -> impl Filter<Extract = impl Reply> + Clone {
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

    get_messages.with(cors)
}
pub async fn run() {
    let routes = build_routes().await;
    warp::serve(routes).run(([0, 0, 0, 0], 3030)).await;
}
