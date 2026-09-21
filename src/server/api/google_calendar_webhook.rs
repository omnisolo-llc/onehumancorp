use axum::{http::HeaderMap, response::IntoResponse, routing::post, Router};
use std::sync::Arc;
use crate::db::DB;

pub fn router<S: Clone + Send + Sync + 'static>(_db: Arc<DB>) -> Router<S> {
    Router::new().route("/google_calendar", post(webhook_handler))
}

async fn webhook_handler(
    headers: HeaderMap,
) -> impl IntoResponse {
    if let Some(state) = headers.get("X-Goog-Resource-State") {
        if state == "sync" || state == "exists" {
            if let Some(channel_token) = headers.get("X-Goog-Channel-Token") {
                if let Ok(tenant_id) = channel_token.to_str() {
                    let tenant_id = tenant_id.to_string();
                    tokio::spawn(async move {
                        let _ = crate::workers::calendar_sync::trigger_tenant_calendar_sync(&tenant_id).await;
                    });
                }
            }
        }
    }
    (axum::http::StatusCode::OK, "OK")
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::http::{HeaderValue, StatusCode};

    #[tokio::test]
    async fn test_webhook_returns_200() {
        let mut headers = HeaderMap::new();
        headers.insert("X-Goog-Resource-State", HeaderValue::from_static("sync"));
        let res = webhook_handler(headers).await.into_response();
        assert_eq!(res.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn test_webhook_with_invalid_state_returns_200() {
        let mut headers = HeaderMap::new();
        headers.insert("X-Goog-Resource-State", HeaderValue::from_static("expired"));
        let res = webhook_handler(headers).await.into_response();
        assert_eq!(res.status(), StatusCode::OK);
    }
}
