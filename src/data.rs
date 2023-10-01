use crate::Message;

pub trait MessageBroker {
    async fn publish_message(&self, channel: String, message: Message) -> Result<(), ()>;
    async fn consume_messages(&self, channel: String, amount: usize) -> Result<Vec<Message>, ()>;
}
