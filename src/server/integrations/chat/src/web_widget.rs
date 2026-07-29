use axum::{
    extract::{
        ws::{Message, WebSocket, WebSocketUpgrade},
        Query, State,
    },
    response::IntoResponse,
    routing::get,
    Router,
};
use futures_util::{stream::StreamExt, SinkExt};
use serde::Deserialize;
use sqlx::PgPool;

#[derive(Deserialize)]
pub struct AuthQuery {
    pub website_token: String,
}

#[derive(Clone)]
pub struct AppState {
    pub db: PgPool,
}

pub fn router(state: AppState) -> Router {
    Router::new()
        .route("/ws", get(ws_handler))
        .with_state(state)
}

async fn ws_handler(
    ws: WebSocketUpgrade,
    Query(auth): Query<AuthQuery>,
    State(state): State<AppState>,
) -> impl IntoResponse {
    ws.on_upgrade(move |socket| handle_socket(socket, state, auth.website_token))
}

async fn handle_socket(socket: WebSocket, _state: AppState, _token: String) {
    let (mut sender, mut receiver) = socket.split();

    while let Some(Ok(msg)) = receiver.next().await {
        if let Message::Text(text) = msg {
            let response = format!("Received: {}", text);
            if sender.send(Message::Text(response.into())).await.is_err() {
                break;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_auth_query() {
        let auth = AuthQuery {
            website_token: "token123".to_string(),
        };
        assert_eq!(auth.website_token, "token123");
    }
}
