use axum::{Json, response::IntoResponse, http::StatusCode};
use serde::{Deserialize, Serialize};
use tracing::info;

#[derive(Deserialize, Debug)]
pub struct EdgeInvalidateRequest {
    pub tags: Vec<String>,
}

#[derive(Serialize)]
pub struct EdgeInvalidateResponse {
    pub success: bool,
    pub invalidated_tags: usize,
}

pub async fn edge_invalidate_handler(
    headers: axum::http::HeaderMap,
    Json(payload): Json<EdgeInvalidateRequest>,
) -> impl IntoResponse {
    let spiffe_id_str = headers.get("x-spiffe-id").and_then(|v| v.to_str().ok()).unwrap_or("");
    let (tenant_id, _) = crate::auth::parse_spiffe_id(spiffe_id_str).unwrap_or(("".to_string(), "".to_string()));

    if tenant_id.is_empty() {
        return (
            StatusCode::UNAUTHORIZED,
            Json(EdgeInvalidateResponse { success: false, invalidated_tags: 0 }),
        );
    }

    info!("Received edge invalidation request for tenant {} with {} tags.", tenant_id, payload.tags.len());
    let cache = crate::builder::edge::get_edge_cache();

    let mut invalidated_count = 0;
    for tag in &payload.tags {
        let safe_tag = if tag.starts_with("tenant-id:") {
            if tag == &format!("tenant-id:{}", tenant_id) {
                tag.clone()
            } else {
                continue; // Reject cross-tenant tag
            }
        } else {
            // For other tags, scope them implicitly to the tenant
            format!("tenant:{}:{}", tenant_id, tag)
        };

        cache.invalidate_by_tag(&safe_tag).await;
        invalidated_count += 1;
    }

    (
        StatusCode::OK,
        Json(EdgeInvalidateResponse { success: true, invalidated_tags: invalidated_count }),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::http::HeaderMap;

    #[tokio::test]
    async fn test_edge_invalidate_unauthorized() {
        let headers = HeaderMap::new();
        let payload = Json(EdgeInvalidateRequest { tags: vec![] });
        let result = edge_invalidate_handler(headers, payload).await.into_response();
        assert_eq!(result.status(), StatusCode::UNAUTHORIZED);
    }

    #[tokio::test]
    async fn test_edge_invalidate_success() {
        let mut headers = HeaderMap::new();
        headers.insert("x-spiffe-id", "spiffe://ohc.network/tenant/t-123/service/test".parse().unwrap());
        let payload = Json(EdgeInvalidateRequest { tags: vec!["entity:product:123".to_string(), "tenant-id:t-123".to_string()] });
        let result = edge_invalidate_handler(headers, payload).await.into_response();
        assert_eq!(result.status(), StatusCode::OK);
    }
}
