use std::sync::Arc;
use tokio::sync::broadcast;
use serde_json::Value;

pub struct ChatHub {
    sender: broadcast::Sender<Value>,
}

impl ChatHub {
    pub fn new() -> Self {
        let (sender, _) = broadcast::channel(100);
        Self { sender }
    }

    pub fn subscribe(&self) -> broadcast::Receiver<Value> {
        self.sender.subscribe()
    }

    pub fn broadcast_message(&self, msg: Value) {
        let _ = self.sender.send(msg);
    }
}
