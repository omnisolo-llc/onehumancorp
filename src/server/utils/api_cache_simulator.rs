use axum::{
    body::{Body, to_bytes},
    extract::Request,
    http::{header, Response, StatusCode},
    middleware::Next,
    response::IntoResponse,
};
use crate::builder::edge::get_edge_cache;
use std::time::Duration;

pub async fn cdn_cache_simulator_middleware(
    req: Request,
    next: Next,
) -> Result<impl IntoResponse, StatusCode> {
    if req.method() != axum::http::Method::GET {
        return Ok(next.run(req).await);
    }

    let uri = req.uri().path().to_string();
    let cache_key = format!("cdn_cache_simulator:{}", uri);
    let cache = get_edge_cache();

    // Check if we have a valid cache hit in our CDN simulator
    if let Some((cached_bytes, _is_stale)) = cache.get_with_swr(&cache_key).await {
        // Return 200 with the cached bytes, acting as an edge CDN hit
        let mut res = Response::builder()
            .status(StatusCode::OK)
            .header(header::CONTENT_TYPE, "text/html; charset=utf-8")
            .header("X-CDN-Cache-Status", "HIT")
            .body(Body::from(cached_bytes))
            .unwrap();
        return Ok(res);
    }

    // Cache miss, process the request
    let response = next.run(req).await;
    let (mut parts, body) = response.into_parts();

    let bytes = match to_bytes(body, 1024 * 1024 * 10).await {
        Ok(b) => b,
        Err(_) => return Err(StatusCode::INTERNAL_SERVER_ERROR),
    };

    if parts.status == StatusCode::OK {
        let mut tags = vec![];

        if let Some(surrogate_key) = parts.headers.get("Surrogate-Key") {
            if let Ok(key_str) = surrogate_key.to_str() {
                for tag in key_str.split_whitespace() {
                    tags.push(tag.to_string());
                }
            }
        }

        if let Some(cache_tag) = parts.headers.get("Cache-Tag") {
            if let Ok(tag_str) = cache_tag.to_str() {
                for tag in tag_str.split(',') {
                    tags.push(tag.trim().to_string());
                }
            }
        }

        // Ensure no duplicate tags
        tags.sort();
        tags.dedup();

        // Store the response in the CDN cache simulator
        // We use String to store bytes directly since HybridCache works with it.
        // Convert bytes to string (HTML) assuming it's valid UTF-8 HTML response
        if let Ok(html_str) = String::from_utf8(bytes.to_vec()) {
            cache.set_with_tags(&cache_key, html_str, tags, Duration::from_secs(3600)).await;
        }
    }

    parts.headers.insert("X-CDN-Cache-Status", "MISS".parse().unwrap());

    let new_body = Body::from(bytes.to_vec());
    let new_response = Response::from_parts(parts, new_body);
    Ok(new_response)
}
