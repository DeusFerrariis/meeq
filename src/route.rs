use axum::{
    Router,
    routing::{get, post}
};
use crate::data::DynMessageBroker;
use crate::handle::*;

pub fn new_router(message_db: DynMessageBroker) -> Router
{
    Router::new()
        .route("/publish", post(publish_message))
        .route("/consume", get(consume_message))
        .with_state(message_db)
}
