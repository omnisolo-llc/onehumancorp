use crate::ohc::common::OmniMessage;
use std::sync::Arc;
use tokio::sync::broadcast;

pub struct RealTimeEngine {
    sender: broadcast::Sender<OmniMessage>,
}

impl RealTimeEngine {
    pub fn new() -> Self {
        let (sender, _) = broadcast::channel(100);
        Self { sender }
    }

    pub fn push(&self, message: OmniMessage) -> Result<(), String> {
        self.sender.send(message).map_err(|e| e.to_string())?;
        Ok(())
    }

    pub fn subscribe(&self) -> broadcast::Receiver<OmniMessage> {
        self.sender.subscribe()
    }
}
