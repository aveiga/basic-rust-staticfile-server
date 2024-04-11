use rust_microservice_archetype::run;

mod dtos;
mod handlers;

#[tokio::main]
async fn main() {
    run().await;
}
