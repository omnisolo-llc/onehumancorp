use axum::{
    body::{Body, to_bytes},
    extract::Request,
    http::{header, Response},
    middleware::Next,
    response::IntoResponse,
};
use std::hash::{Hash, Hasher};
use std::collections::hash_map::DefaultHasher;
use server_auth::orchestration::AuthInfo;

pub async fn edge_caching_middleware(
    req: Request,
    next: Next,
) -> Result<impl IntoResponse, axum::http::StatusCode> {
    // Extract tenant info early if available to automatically add tenant tag
    let tenant_id = req.extensions().get::<AuthInfo>().map(|c| c.org_id.clone());

    let mut response = next.run(req).await.into_response();

    if let Some(tid) = tenant_id {
        if let Ok(val) = format!("tenant-id:{}", tid).parse() {
            response.headers_mut().append("Cache-Tag", val);
        }
    }

    let (mut parts, body) = response.into_parts();

    // Set Surrogate-Key from Cache-Tag if present
    if let Some(cache_tag) = parts.headers.get("Cache-Tag") {
        if let Ok(tag_str) = cache_tag.to_str() {
            // Fastly uses space-separated keys, replace ", " with " "
            let surrogate_val = tag_str.replace(", ", " ");
            if let Ok(val) = surrogate_val.parse() {
                parts.headers.insert("Surrogate-Key", val);
            }
        }
    }

    // Buffer body to compute ETag (limit 10MB)
    let bytes = match to_bytes(body, 1024 * 1024 * 10).await {
        Ok(b) => b,
        Err(_) => return Err(axum::http::StatusCode::INTERNAL_SERVER_ERROR),
    };

    if !bytes.is_empty() {
        let mut hasher = DefaultHasher::new();
        bytes.hash(&mut hasher);
        let etag = format!("W/\"{:x}\"", hasher.finish());
        if let Ok(etag_val) = etag.parse() {
            parts.headers.insert(header::ETAG, etag_val);
        }
    }

    if !parts.headers.contains_key(header::CACHE_CONTROL) {
        if let Ok(val) = "public, s-maxage=60, stale-while-revalidate=86400".parse() {
            parts.headers.insert(header::CACHE_CONTROL, val);
        }
    }

    let new_body = Body::from(bytes.to_vec());
    let new_response = Response::from_parts(parts, new_body);
    Ok(new_response)
}
