use std::sync::Arc;
use tokio::sync::RwLock;
use crate::Message;
// NOTE: use this until support for trait objects is stabilized for async traits
use axum::async_trait;

#[async_trait]
pub trait MessageBroker {
    async fn publish_message(&self, channel: String, message: Message) -> Result<(), ()>;
    async fn consume_messages(&self, channel: String, amount: usize) -> Result<Vec<Message>, ()>;
}

pub type DynMessageBroker = Arc<dyn MessageBroker + Send + Sync>;

#[derive(Debug, Clone)]
pub struct LockedMessageQueue(Arc<RwLock<Vec<Message>>>);

impl LockedMessageQueue {
    pub fn new() -> Self {
        LockedMessageQueue(
            Arc::new(
                RwLock::new(Vec::new())
            )
        )
    }
}

#[async_trait]
impl MessageBroker for LockedMessageQueue {
    async fn publish_message(&self, _channel: String, message: Message) -> Result<(), ()> {
        let mut queue = self.0.write().await;
        queue.push(message);
        Ok(())
    }

    async fn consume_messages(&self, channel: String, amount: usize) -> Result<Vec<Message>, ()> {
        let messages = {
            let queue = self.0.read().await;
            queue.iter()
                .filter(|message| message.channel == channel)
                .cloned()
                .take(amount)
                .collect::<Vec<_>>()
        };

        self.0.write().await.retain(|message| {
            !messages.contains(message)
        });

        Ok(messages)
    }
}
