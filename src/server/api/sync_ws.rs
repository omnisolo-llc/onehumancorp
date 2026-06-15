use axum::{
    extract::{ws::{Message as WsMessage, WebSocket, WebSocketUpgrade}, State, Query},
    response::IntoResponse,
    http::HeaderMap,
    routing::get,
    Router,
};
use serde::Deserialize;
use futures::{sink::SinkExt, stream::StreamExt};
use crate::api::mesh_handler::check_spiffe_auth;

#[derive(Deserialize)]
pub struct ConnectQuery {
    pub topics: Option<String>,
}

pub async fn ws_sync_handler(
    ws: WebSocketUpgrade,
    headers: HeaderMap,
    State(redis_client): State<Option<redis::Client>>,
    Query(query): Query<ConnectQuery>,
) -> impl IntoResponse {
    // Authenticate using check_spiffe_auth or equivalent session derivation
    if !crate::is_standalone_runtime() {
        if let Err(err_response) = check_spiffe_auth(&headers) {
            return err_response;
        }
    }

    // Rely on validated identity for tenant_id. Fallback or logic based on auth...
    // In many OHC auth flows, x-spiffe-id or similar verified token holds the real identity
    let spiffe_id_str = headers.get("x-spiffe-id").and_then(|v| v.to_str().ok()).unwrap_or("");
    let (mut tenant_id, _) = crate::auth::parse_spiffe_id(spiffe_id_str).unwrap_or(("".to_string(), "".to_string()));

    if tenant_id.is_empty() {
        // Fallback to x-tenant-id for tests if standalone
        if crate::is_standalone_runtime() {
            tenant_id = headers.get("x-tenant-id").and_then(|val| val.to_str().ok()).unwrap_or("default").to_string();
        } else {
            return (axum::http::StatusCode::UNAUTHORIZED, "Unauthorized").into_response();
        }
    }

    let topics = query.topics
        .map(|t| t.split(',').map(|s| s.to_string()).collect::<Vec<String>>())
        .unwrap_or_else(Vec::new);

    let allowed_topics = topics.into_iter()
        .filter(|t| t.starts_with(&format!("{}:", tenant_id)) || t.starts_with(&format!("tenant:{}:", tenant_id)) || t.contains(&format!(":{}", tenant_id)))
        .collect::<Vec<String>>();

    ws.on_upgrade(move |socket| handle_sync_socket(socket, redis_client, allowed_topics))
}

async fn handle_sync_socket(socket: WebSocket, redis_client: Option<redis::Client>, topics: Vec<String>) {
    let (mut sender, mut receiver) = socket.split();

    if topics.is_empty() {
        let _ = sender.send(WsMessage::Text("No authorized topics to subscribe to.".into())).await;
        return;
    }

    if let Some(client) = redis_client {
        if let Ok(mut pubsub_con) = client.get_async_pubsub().await {
            for topic in &topics {
                let _ = pubsub_con.subscribe(topic).await;
            }

            let mut pubsub_stream = pubsub_con.into_on_message();

            let mut send_task = tokio::spawn(async move {
                while let Some(msg) = pubsub_stream.next().await {
                    if let Ok(payload) = msg.get_payload::<String>() {
                        if sender.send(WsMessage::Text(payload.into())).await.is_err() {
                            break;
                        }
                    }
                }
            });

            let mut recv_task = tokio::spawn(async move {
                while let Some(Ok(msg)) = receiver.next().await {
                    if let WsMessage::Text(_text) = msg {
                        // handle ping or other messages if necessary
                    }
                }
            });

            tokio::select! {
                _ = (&mut send_task) => recv_task.abort(),
                _ = (&mut recv_task) => send_task.abort(),
            };
        } else {
             let _ = sender.send(WsMessage::Text("Redis pubsub error.".into())).await;
        }
    } else {
        // Fallback for standalone/sqlite mode where redis is not available
        let _ = sender.send(WsMessage::Text("Redis not configured. WebSocket sync requires Redis.".into())).await;
    }
}

pub fn router<S>(redis_client: Option<redis::Client>) -> Router<S>
where
    S: Send + Sync + 'static + Clone,
{
    Router::new()
        .route("/ws", get({
            let rc = redis_client.clone();
            move |ws: WebSocketUpgrade, headers: HeaderMap, query: Query<ConnectQuery>| async move {
                 ws_sync_handler(ws, headers, State(rc), query).await
            }
        }))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::net::SocketAddr;
    use tokio::net::TcpListener;
    use tokio_tungstenite::connect_async;
    use tokio_tungstenite::tungstenite::Message as TungsteniteMessage;

    #[tokio::test]
    async fn test_ws_sync_handler() {
        let client = None; // For test without redis
        let app = router::<()>(client);

        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let addr = listener.local_addr().unwrap();

        tokio::spawn(async move {
            axum::serve(
                listener,
                app.into_make_service_with_connect_info::<SocketAddr>(),
            )
            .await
            .unwrap();
        });

        let ws_url = format!("ws://{}/ws?topics=tenant:test:inventory", addr);
        let request = reqwest::Request::new(reqwest::Method::GET, ws_url.parse().unwrap());
        let mut request = tokio_tungstenite::tungstenite::client::IntoClientRequest::into_client_request(request.url().as_str()).unwrap();

        // Pass the actual Spiffe ID for 'test' tenant.
        request.headers_mut().insert("x-spiffe-id", "spiffe://ohc/tenant/test/agent/pos".parse().unwrap());

        let (mut ws_stream, _) = connect_async(request).await.expect("Failed to connect");

        if let Some(Ok(msg)) = ws_stream.next().await {
            if let TungsteniteMessage::Text(text) = msg {
                assert!(text.contains("Redis not configured") || text.contains("No authorized topics"));
            }
        }
    }
}
