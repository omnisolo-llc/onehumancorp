use axum::{
    extract::Extension,
    response::{sse::{Event, KeepAlive, Sse}, IntoResponse},
    http::StatusCode,
    routing::get,
    Router,
};
use std::{convert::Infallible, time::Duration};
use tokio_stream::StreamExt;
use ::server_common::Claims;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct StreamEvent {
    pub event_type: String,
    pub payload: serde_json::Value,
    pub timestamp: i64,
}

pub fn router<S>() -> Router<S> where S: Clone + Send + Sync + 'static, {
    Router::new()
        .route("/", get(sse_handler))
}

async fn sse_handler(
    Extension(claims): Extension<Claims>,
) -> impl IntoResponse {
    let tenant_id = match claims.organization_id.as_deref() {
        Some(org_id) => org_id.to_string(),
        None => return (StatusCode::UNAUTHORIZED, "Unauthorized").into_response(),
    };

    let channel_name = format!("ohc:stream:{}", tenant_id);

    let redis_client_opt = crate::get_redis_client();

    if let Some(redis_client) = redis_client_opt {
        match redis_client.get_async_pubsub().await {
            Ok(mut pubsub) => {
                if let Err(e) = pubsub.subscribe(&channel_name).await {
                    tracing::error!("Failed to subscribe to redis pubsub: {}", e);
                    return (StatusCode::INTERNAL_SERVER_ERROR, "Internal Error").into_response();
                }

                let stream = pubsub.into_on_message().map(|msg| {
                    match msg.get_payload::<String>() {
                        Ok(payload_str) => {
                            Event::default().data(payload_str)
                        }
                        Err(_) => Event::default().data(r#"{"event_type":"error","payload":{},"timestamp":0}"#),
                    }
                }).map(Ok::<_, Infallible>);

                Sse::new(stream)
                    .keep_alive(KeepAlive::new().interval(Duration::from_secs(15)).text("keep-alive-text"))
                    .into_response()
            }
            Err(e) => {
                tracing::error!("Failed to get redis pubsub connection: {}", e);
                return (StatusCode::INTERNAL_SERVER_ERROR, "Internal Error").into_response();
            }
        }
    } else {
         let stream = futures::stream::unfold((), |()| async {
             tokio::time::sleep(Duration::from_secs(30)).await;
             let event = StreamEvent {
                 event_type: "ping".to_string(),
                 payload: serde_json::json!({}),
                 timestamp: chrono::Utc::now().timestamp_millis(),
             };
             Some((Ok::<_, Infallible>(Event::default().data(serde_json::to_string(&event).unwrap())), ()))
         });

         Sse::new(stream)
             .keep_alive(KeepAlive::new().interval(Duration::from_secs(15)).text("keep-alive-text"))
             .into_response()
    }
}
