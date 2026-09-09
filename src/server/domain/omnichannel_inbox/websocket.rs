use super::data::Message;
use serde::Serialize;
use tokio::sync::broadcast;

#[derive(Debug, Clone, Serialize)]
pub enum InboxEvent {
    MessageReceived(Message),
    MessageSent(Message),
    ConversationUpdated(uuid::Uuid),
}

pub struct InboxEventBroadcaster {
    sender: broadcast::Sender<InboxEvent>,
}

impl InboxEventBroadcaster {
    pub fn new() -> Self {
        let (sender, _) = broadcast::channel(100);
        Self { sender }
    }

    pub fn subscribe(&self) -> broadcast::Receiver<InboxEvent> {
        self.sender.subscribe()
    }

    pub fn broadcast(&self, event: InboxEvent) -> Result<usize, broadcast::error::SendError<InboxEvent>> {
        self.sender.send(event)
    }
}
