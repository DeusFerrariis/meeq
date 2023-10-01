#![feature(async_fn_in_trait)]

mod data;
mod model;
mod handle;

use axum::{
    Router,
    routing::{get, post},
};
use std::net::SocketAddr;
use data::LockedMessageQueue;
use model::Message;
use handle::{publish_message, consume_message};

#[tokio::main]
async fn main() {
    let queue_db = LockedMessageQueue::new();
    let app = Router::new()
        .route("/publish", post(publish_message))
        .route("/consume", get(consume_message))
        .with_state(queue_db.clone());
    let addr = SocketAddr::from(([127, 0, 0, 1], 3001));

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
