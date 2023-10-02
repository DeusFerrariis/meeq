use axum::{
    http,
    Json,
    response::IntoResponse,
    extract::{State, Query},
};

use crate::model::{Message};
use crate::data::DynMessageBroker;
use crate::request::ConsumeQuery;

#[axum::debug_handler]
pub async fn publish_message(
    State(queue_db): State<DynMessageBroker>,
    Json(message): Json<Message>
) -> Result<impl IntoResponse, (http::StatusCode, &'static str)> {
    queue_db.publish_message(message.channel.clone(), message).await
        .map_err(|_| (http::StatusCode::INTERNAL_SERVER_ERROR, "Failed to publish message"))?;

    Ok((http::StatusCode::OK, "Message published"))
}

// consume amount (url query) of messages from channel
#[axum::debug_handler]
pub async fn consume_message(
    State(queue_db): State<DynMessageBroker>,
    Query(ConsumeQuery {amount, channel}): Query<ConsumeQuery>
) -> Result<impl IntoResponse, (http::StatusCode, Json<Vec<Message>>)> {
    let amount_or = amount.unwrap_or(1);
    let messages = queue_db.consume_messages(channel, amount_or).await
        .map_err(|_| (http::StatusCode::INTERNAL_SERVER_ERROR, Json(Vec::new())))?;

    Ok((http::StatusCode::OK, Json(messages)))
}
