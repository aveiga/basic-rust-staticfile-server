use std::convert::Infallible;
use warp::http::StatusCode;

pub async fn get_flat_entries() -> Result<impl warp::Reply, Infallible> {
    Ok(StatusCode::OK)
}
