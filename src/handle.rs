use axum::{
    http,
    Json,
    response::IntoResponse,
    extract::{State, Query},
};

use crate::model::{Message};
use crate::data::{MessageBroker, LockedMessageQueue};

pub async fn publish_message(
    State(queue_db): State<LockedMessageQueue>,
    Json(message): Json<Message>
) -> Result<impl IntoResponse, (http::StatusCode, &'static str)> {
    queue_db.publish_message(message.channel.clone(), message).await
        .map_err(|_| (http::StatusCode::INTERNAL_SERVER_ERROR, "Failed to publish message"))?;

    Ok((http::StatusCode::OK, "Message published"))
}

#[derive(Debug, serde::Deserialize, serde::Serialize, Clone)]
pub struct Consume {
    channel: String,
    amount: Option<usize>,
}

// consume amount (url query) of messages from channel
pub async fn consume_message(
    State(queue_db): State<LockedMessageQueue>,
    Query(consume): Query<Consume>
) -> Result<impl IntoResponse, (http::StatusCode, Json<Vec<Message>>)> {
    let amount = consume.amount.unwrap_or(1);
    let channel = consume.channel.clone();

    let messages = queue_db.consume_messages(channel, amount).await
        .map_err(|_| (http::StatusCode::INTERNAL_SERVER_ERROR, Json(Vec::new())))?;

    Ok((http::StatusCode::OK, Json(messages)))
}
