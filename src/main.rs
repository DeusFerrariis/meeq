use std::sync::Arc;
use tokio::sync::RwLock;
use axum::{
    http,
    Router,
    Json,
    routing::{get, post},
    response::IntoResponse,
    extract::{State, Query},
};
use std::net::SocketAddr;

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

#[derive(PartialEq, Debug, serde::Deserialize, serde::Serialize, Clone)]
struct Message {
    channel: String,
    headers: Option<serde_json::Value>,
    body: serde_json::Value,
}

#[derive(Debug, Clone)]
struct LockedMessageQueue(Arc<RwLock<Vec<Message>>>);


impl LockedMessageQueue {
    fn new() -> Self {
        LockedMessageQueue(
            Arc::new(
                RwLock::new(Vec::new())
            )
        )
    }
}

async fn publish_message(
    State(queue_db): State<LockedMessageQueue>,
    Json(message): Json<Message>
) -> Result<impl IntoResponse, (http::StatusCode, &'static str)> {
    let mut queue = queue_db.0.write().await;
    queue.push(message);
    Ok((http::StatusCode::OK, "message published"))
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

    let messages = {
        let queue = queue_db.0.read().await;
        queue.iter()
            .filter(|message| message.channel == consume.channel)
            .cloned()
            .take(amount)
            .collect::<Vec<_>>()
    };

    queue_db.0.write().await.retain(|message| {
        !messages.contains(message)
    });

    Ok(Json(messages))
}
