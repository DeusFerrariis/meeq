#![feature(async_fn_in_trait)]

mod data;
mod model;

use axum::{
    http,
    Router,
    Json,
    routing::{get, post},
    response::IntoResponse,
    extract::{State, Query},
};
use std::net::SocketAddr;
use data::{MessageBroker, LockedMessageQueue};
use model::Message;

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

async fn publish_message(
    State(queue_db): State<LockedMessageQueue>,
    Json(message): Json<Message>
) -> Result<impl IntoResponse, (http::StatusCode, &'static str)> {
    queue_db.publish_message(message.channel.clone(), message).await
        .map_err(|_| (http::StatusCode::INTERNAL_SERVER_ERROR, "Failed to publish message"))?;

    Ok((http::StatusCode::OK, "Message published"))
}

#[derive(Debug, serde::Deserialize, serde::Serialize, Clone)]
struct Consume {
    channel: String,
    amount: Option<usize>,
}

// consume amount (url query) of messages from channel
async fn consume_message(
    State(queue_db): State<LockedMessageQueue>,
    Query(consume): Query<Consume>
) -> Result<impl IntoResponse, (http::StatusCode, Json<Vec<Message>>)> {
    let amount = consume.amount.unwrap_or(1);
    let channel = consume.channel.clone();

    let messages = queue_db.consume_messages(channel, amount).await
        .map_err(|_| (http::StatusCode::INTERNAL_SERVER_ERROR, Json(Vec::new())))?;

    Ok((http::StatusCode::OK, Json(messages)))
}
