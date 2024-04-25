use crate::dtos::flat_entries_dtos::FlatEntries;

use std::convert::Infallible;
use warp::http::StatusCode;

pub async fn get_flat_entries() -> Result<impl warp::Reply, Infallible> {
    let v = vec![
        FlatEntries {
            key: "key1".to_string(),
            value: "value1".to_string(),
        },
        FlatEntries {
            key: "key2".to_string(),
            value: "value2".to_string(),
        },
    ];

    Ok(warp::reply::with_status(
        warp::reply::json(&v),
        StatusCode::OK,
    ))
}

pub async fn post_flat_entries(
    newEntries: Vec<FlatEntries>,
) -> Result<impl warp::Reply, Infallible> {
    Ok(warp::reply::with_status(
        warp::reply::json(&newEntries),
        StatusCode::CREATED,
    ))
}
