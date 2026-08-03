// Temporarily mock WebSocket routing as we adjust Bazel dependencies
use std::sync::Arc;
use tokio::sync::broadcast;

pub struct WebWidgetState {
    pub tx: broadcast::Sender<String>,
}

impl Default for WebWidgetState {
    fn default() -> Self {
        Self::new()
    }
}

impl WebWidgetState {
    pub fn new() -> Self {
        let (tx, _) = broadcast::channel(100);
        Self { tx }
    }
}

pub async fn handle_socket_message(state: Arc<WebWidgetState>, message: String) {
    let _ = state.tx.send(message);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_web_widget_state() {
        let state = WebWidgetState::new();
        let mut _rx = state.tx.subscribe();
        assert!(state.tx.send("test message".to_string()).is_ok());
    }
}
