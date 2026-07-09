
use axum::{
    extract::{Extension, ws::{Message as WsMessage, WebSocket, WebSocketUpgrade}},
    response::IntoResponse,
    http::StatusCode,
};
use ::server_common::Claims;
use futures::{stream::StreamExt, sink::SinkExt};
use std::sync::Arc;
use tokio::sync::Mutex;
use std::sync::OnceLock;
use tokio::sync::broadcast;
use dashmap::DashMap;

static TENANT_CHANNELS: OnceLock<DashMap<String, broadcast::Sender<String>>> = OnceLock::new();
static REDIS_SUBSCRIBED: OnceLock<Arc<Mutex<bool>>> = OnceLock::new();

fn get_redis_client() -> redis::Client {
    let redis_url = std::env::var("REDIS_URL").unwrap_or_else(|_| "redis://127.0.0.1:6379".to_string());
    redis::Client::open(redis_url).expect("Invalid Redis URL")
}

fn get_tenant_channel(tenant_id: &str) -> broadcast::Sender<String> {
    let channels = TENANT_CHANNELS.get_or_init(DashMap::new);
    channels.entry(tenant_id.to_string()).or_insert_with(|| {
        let (tx, _) = broadcast::channel(100);
        tx
    }).clone()
}

async fn ensure_redis_subscription() {
    let subscribed = REDIS_SUBSCRIBED.get_or_init(|| Arc::new(Mutex::new(false)));
    let mut is_sub = subscribed.lock().await;
    if *is_sub {
        return;
    }
    *is_sub = true;

    tokio::spawn(async move {
        loop {
            let client = get_redis_client();
            let pubsub_conn = client.get_async_pubsub().await;

            if let Ok(mut pubsub_conn) = pubsub_conn {
                if let Err(e) = pubsub_conn.psubscribe("tenant_events:*").await {
                    tracing::error!("Failed to psubscribe to tenant_events: {}", e);
                } else {
                    let mut pubsub_stream = pubsub_conn.on_message();

                    while let Some(msg) = pubsub_stream.next().await {
                        let channel_name = msg.get_channel_name().to_string();
                        if let Some(tenant_id) = channel_name.strip_prefix("tenant_events:") {
                            if let Ok(payload) = msg.get_payload::<String>() {
                                let channels = TENANT_CHANNELS.get_or_init(DashMap::new);
                                if let Some(tx) = channels.get(tenant_id) {
                                    let _ = tx.send(payload);
                                }
                            }
                        }
                    }
                }
            }
            tracing::error!("Redis connection for tenant_events dropped, reconnecting in 5s");
            tokio::time::sleep(std::time::Duration::from_secs(5)).await;
        }
    });
}

pub async fn ws_handler(
    ws: WebSocketUpgrade,
    Extension(claims): Extension<Claims>,
) -> impl IntoResponse {
    let tenant_id = match claims.organization_id.as_deref() {
        Some(org_id) => org_id.to_string(),
        None => return StatusCode::UNAUTHORIZED.into_response(),
    };

    ws.on_upgrade(move |socket| handle_socket(socket, tenant_id))
}

async fn handle_socket(socket: WebSocket, tenant_id: String) {
    ensure_redis_subscription().await;

    let (mut sender, mut receiver) = socket.split();

    let tx = get_tenant_channel(&tenant_id);
    let mut rx = tx.subscribe();

    loop {
        tokio::select! {
            msg_res = rx.recv() => {
                match msg_res {
                    Ok(payload) => {
                        if let Err(e) = sender.send(WsMessage::Text(payload.into())).await {
                            tracing::error!("Failed to send tenant event message to client: {}", e);
                            break;
                        }
                    }
                    Err(broadcast::error::RecvError::Lagged(_)) => {}
                    Err(broadcast::error::RecvError::Closed) => {
                        break;
                    }
                }
            }
            client_msg = receiver.next() => {
                match client_msg {
                    Some(Ok(msg)) => {
                        if let WsMessage::Close(_) = msg {
                            break;
                        }
                        if let WsMessage::Ping(data) = msg {
                            if let Err(e) = sender.send(WsMessage::Pong(data)).await {
                                tracing::error!("Failed to send pong: {}", e);
                                break;
                            }
                        }
                    }
                    Some(Err(e)) => {
                        tracing::error!("Error receiving from client ws: {}", e);
                        break;
                    }
                    None => {
                        break;
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::{routing::get, Router};
    use std::net::SocketAddr;
    use tokio::net::TcpListener;
    use tokio_tungstenite::connect_async;

    #[tokio::test]
    async fn test_tenant_events_websocket() {
        let mock_claims = Claims {
            sub: "user-123".to_string(),
            organization_id: Some("test_ws_tenant".to_string()),
            roles: vec!["ADMIN".to_string()],
            iat: 0,
            username: "test".to_string(),
            email: "test@test.com".to_string(),
            exp: 9999999999,
            jti: "test_jti".to_string(),
            session_id: Some("test_session_id".to_string()),
        };

        let app = Router::new()
            .route("/api/v1/tenant/events/ws", get(ws_handler))
            .layer(Extension(mock_claims));

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

        let redis_url = std::env::var("REDIS_URL").unwrap_or_else(|_| "redis://127.0.0.1:6379".to_string());
        if let Ok(client) = redis::Client::open(redis_url) {
            if client.get_connection().is_ok() {
                let ws_url = format!("ws://{}/api/v1/tenant/events/ws", addr);
                let (mut ws_stream, _) = connect_async(ws_url).await.expect("Failed to connect");

                tokio::time::sleep(std::time::Duration::from_millis(200)).await;

                let mut conn = client.get_multiplexed_async_connection().await.unwrap();
                let topic = "tenant_events:test_ws_tenant";
                let payload = "{\"event_type\":\"agent.task.completed\"}";
                let _: () = redis::cmd("PUBLISH").arg(topic).arg(payload).query_async(&mut conn).await.unwrap();

                let msg = tokio::time::timeout(std::time::Duration::from_secs(2), ws_stream.next())
                    .await
                    .expect("Timeout waiting for websocket message")
                    .expect("Stream closed early")
                    .expect("Error receiving message");

                assert!(msg.is_text());
                assert_eq!(msg.to_text().unwrap(), payload);
            }
        }
    }
}
