mod data;
mod model;
mod handle;
mod request;
mod route;

use std::net::SocketAddr;
use std::sync::Arc;
use data::LockedMessageQueue;
use model::Message;

#[tokio::main]
async fn main() {
    let app = route::new_router(Arc::new(LockedMessageQueue::new()));
    let addr = SocketAddr::from(([127, 0, 0, 1], 3001));

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
