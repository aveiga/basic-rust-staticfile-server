// use std::str::FromStr;
// use std::io::{Error, ErrorKind};

use warp::Filter;

#[tokio::main]
async fn main() {
    let hello = warp::get()
        .map(|| format!("Hello, World!"));

    warp::serve(hello)
        .run(([0, 0, 0, 0], 3030))
        .await;
}