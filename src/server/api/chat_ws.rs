use axum::{
    extract::ws::WebSocketUpgrade,
    response::IntoResponse,
};
use ::server_integrations_chat::chat_ws_handler;

pub async fn handler(ws: WebSocketUpgrade) -> impl IntoResponse {
    chat_ws_handler(ws).await
}
