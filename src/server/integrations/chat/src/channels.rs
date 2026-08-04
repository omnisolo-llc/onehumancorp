use axum::{
    extract::{ws::{WebSocket, WebSocketUpgrade}},
    response::IntoResponse,
};

pub mod whatsapp {
    pub async fn handle_webhook(_payload: String) -> Result<String, String> {
        Ok("OK".to_string())
    }
}

pub mod web_widget {
    use super::*;

    pub async fn handle_socket(_socket: WebSocket) {
        // Echo stub
    }

    pub async fn ws_handler(ws: WebSocketUpgrade) -> impl IntoResponse {
        ws.on_upgrade(handle_socket)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_whatsapp_webhook() {
        let payload = String::from("test");
        let result = whatsapp::handle_webhook(payload).await;
        assert_eq!(result.unwrap(), "OK");
    }
}
