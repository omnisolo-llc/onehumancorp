use axum::{
    body::{Body, to_bytes},
    extract::Request,
    http::{header, Response},
    middleware::Next,
    response::IntoResponse,
};
use std::hash::{Hash, Hasher};
use std::collections::hash_map::DefaultHasher;
use std::sync::OnceLock;
use crate::cache::HybridCache;
use serde::{Serialize, Deserialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct CachedResponse {
    pub status: u16,
    pub headers: Vec<(String, String)>,
    pub body: Vec<u8>,
}

pub static CDN_CACHE: OnceLock<std::sync::Arc<HybridCache<CachedResponse>>> = OnceLock::new();

pub fn get_cdn_cache() -> std::sync::Arc<HybridCache<CachedResponse>> {
    CDN_CACHE.get_or_init(|| std::sync::Arc::new(HybridCache::new(None))).clone()
}

pub async fn edge_caching_middleware(
    req: Request,
    next: Next,
) -> Result<impl IntoResponse, axum::http::StatusCode> {
    let method = req.method().clone();
    let uri = req.uri().to_string();
    let is_get = method == axum::http::Method::GET;

    let bypass_cache = req
        .headers()
        .get(header::CACHE_CONTROL)
        .and_then(|val| val.to_str().ok())
        .map(|val| val.contains("no-cache"))
        .unwrap_or(false);

    let cdn_cache = get_cdn_cache();
    let cache_key = format!("cdn:{}", uri);

    if is_get && !bypass_cache {
        if let Some((cached, _is_stale)) = cdn_cache.get_with_swr(&cache_key).await {
            let body = Body::from(cached.body);
            let mut response = Response::builder()
                .status(cached.status)
                .body(body)
                .unwrap();

            for (k, v) in cached.headers {
                if let (Ok(hk), Ok(hv)) = (axum::http::HeaderName::try_from(k), axum::http::HeaderValue::try_from(v)) {
                    response.headers_mut().insert(hk, hv);
                }
            }
            response.headers_mut().insert("X-Cache", "HIT".parse().unwrap());
            return Ok(response.into_response());
        }
    }

    let response = next.run(req).await;

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

    if !bytes.is_empty() && !parts.headers.contains_key(header::ETAG) {
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

    parts.headers.insert("X-Cache", "MISS".parse().unwrap());

    if is_get && parts.status.is_success() {
        let mut tags_vec = Vec::new();
        if let Some(surrogate) = parts.headers.get("Surrogate-Key") {
            if let Ok(s) = surrogate.to_str() {
                for t in s.split(' ') {
                    if !t.is_empty() {
                        tags_vec.push(t.to_string());
                    }
                }
            }
        }

        let mut headers_vec = Vec::new();
        for (k, v) in parts.headers.iter() {
            if let Ok(v_str) = v.to_str() {
                headers_vec.push((k.as_str().to_string(), v_str.to_string()));
            }
        }

        let cached_response = CachedResponse {
            status: parts.status.as_u16(),
            headers: headers_vec,
            body: bytes.to_vec(),
        };

        cdn_cache.set_with_tags(&cache_key, cached_response, tags_vec, std::time::Duration::from_secs(60)).await;
    }

    let new_body = Body::from(bytes.to_vec());
    let new_response = Response::from_parts(parts, new_body);
    Ok(new_response.into_response())
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::{
        body::{Body, to_bytes},
        http::{Request, Response, StatusCode},
        middleware::from_fn,
        routing::get,
        Router,
    };
    use tower::ServiceExt;

    #[tokio::test]
    async fn test_edge_caching_middleware_hit_miss() {
        let app = Router::new()
            .route("/", get(|| async { "Hello, World!" }))
            .layer(from_fn(edge_caching_middleware));

        let req1 = Request::builder().uri("/").body(Body::empty()).unwrap();
        let res1 = app.clone().oneshot(req1).await.unwrap();
        assert_eq!(res1.status(), StatusCode::OK);
        assert_eq!(res1.headers().get("X-Cache").unwrap(), "MISS");

        // Allow cache to be saved asynchronously
        tokio::time::sleep(std::time::Duration::from_millis(50)).await;

        let req2 = Request::builder().uri("/").body(Body::empty()).unwrap();
        let res2 = app.clone().oneshot(req2).await.unwrap();
        assert_eq!(res2.status(), StatusCode::OK);
        assert_eq!(res2.headers().get("X-Cache").unwrap(), "HIT");

        let body_bytes = to_bytes(res2.into_body(), 1024).await.unwrap();
        assert_eq!(body_bytes, "Hello, World!");
    }

    #[tokio::test]
    async fn test_edge_caching_middleware_bypass_no_cache() {
        let app = Router::new()
            .route("/bypass", get(|| async { "Hello, Bypass!" }))
            .layer(from_fn(edge_caching_middleware));

        // Initial request -> MISS
        let req1 = Request::builder().uri("/bypass").body(Body::empty()).unwrap();
        let res1 = app.clone().oneshot(req1).await.unwrap();
        assert_eq!(res1.headers().get("X-Cache").unwrap(), "MISS");

        tokio::time::sleep(std::time::Duration::from_millis(50)).await;

        // Second request with no-cache -> MISS (bypassed)
        let req2 = Request::builder()
            .uri("/bypass")
            .header(header::CACHE_CONTROL, "no-cache")
            .body(Body::empty())
            .unwrap();
        let res2 = app.clone().oneshot(req2).await.unwrap();
        assert_eq!(res2.headers().get("X-Cache").unwrap(), "MISS");
    }

    #[tokio::test]
    async fn test_edge_caching_middleware_surrogate_key() {
        let app = Router::new()
            .route("/surrogate", get(|| async {
                let mut res = Response::new(Body::from("Surrogate Content"));
                res.headers_mut().insert("Cache-Tag", "tag1, tag2".parse().unwrap());
                res
            }))
            .layer(from_fn(edge_caching_middleware));

        let req = Request::builder().uri("/surrogate").body(Body::empty()).unwrap();
        let res = app.oneshot(req).await.unwrap();

        assert_eq!(res.headers().get("Surrogate-Key").unwrap(), "tag1 tag2");
        assert_eq!(res.headers().get("X-Cache").unwrap(), "MISS");
    }
}
