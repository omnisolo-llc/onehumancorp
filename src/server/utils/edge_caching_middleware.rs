use crate::cache::HybridCache;
use axum::{
    body::{Body, to_bytes},
    extract::Request,
    http::{Response, header},
    middleware::Next,
    response::IntoResponse,
};
use serde::{Deserialize, Serialize};
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::sync::OnceLock;

#[derive(Clone, Serialize, Deserialize)]
pub struct CachedResponse {
    pub status: u16,
    pub headers: Vec<(String, String)>,
    pub body: Vec<u8>,
}

pub static CDN_CACHE: OnceLock<std::sync::Arc<HybridCache<CachedResponse>>> = OnceLock::new();

pub fn get_cdn_cache() -> std::sync::Arc<HybridCache<CachedResponse>> {
    CDN_CACHE
        .get_or_init(|| std::sync::Arc::new(HybridCache::new(None)))
        .clone()
}

pub async fn edge_caching_middleware(
    req: Request,
    next: Next,
) -> Result<impl IntoResponse, axum::http::StatusCode> {
    // Authenticated/private documents cannot use the shared public URI cache.
    // This check precedes lookup, so old cache entries cannot bypass authority.
    if req.headers().contains_key(header::AUTHORIZATION)
        || req.headers().contains_key(header::COOKIE)
        || req.extensions().get::<server_common::Claims>().is_some()
    {
        let mut response = next.run(req).await;
        response.headers_mut().insert(
            header::CACHE_CONTROL,
            axum::http::HeaderValue::from_static("private, no-store"),
        );
        return Ok(response.into_response());
    }
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

    if is_get
        && !bypass_cache
        && let Some((cached, _is_stale)) = cdn_cache.get_with_swr(&cache_key).await
    {
        let body = Body::from(cached.body);
        let mut response = Response::builder()
            .status(cached.status)
            .body(body)
            .unwrap();

        for (k, v) in cached.headers {
            if let (Ok(hk), Ok(hv)) = (
                axum::http::HeaderName::try_from(k),
                axum::http::HeaderValue::try_from(v),
            ) {
                response.headers_mut().insert(hk, hv);
            }
        }
        response
            .headers_mut()
            .insert("X-Cache", "HIT".parse().unwrap());
        return Ok(response.into_response());
    }

    let response = next.run(req).await;

    let (mut parts, body) = response.into_parts();

    // Set Surrogate-Key from Cache-Tag if present
    if let Some(cache_tag) = parts.headers.get("Cache-Tag")
        && let Ok(tag_str) = cache_tag.to_str()
    {
        // Fastly uses space-separated keys, replace ", " with " "
        let surrogate_val = tag_str.replace(", ", " ");
        if let Ok(val) = surrogate_val.parse() {
            parts.headers.insert("Surrogate-Key", val);
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

    let private_response = parts.headers.contains_key(header::SET_COOKIE)
        || parts
            .headers
            .get_all(header::CACHE_CONTROL)
            .iter()
            .any(|value| {
                let Ok(value) = value.to_str() else {
                    return true;
                };
                value.split(',').any(|directive| {
                    matches!(
                        directive
                            .split('=')
                            .next()
                            .unwrap_or_default()
                            .trim()
                            .to_ascii_lowercase()
                            .as_str(),
                        "private" | "no-store" | "no-cache"
                    )
                })
            });
    if private_response {
        parts.headers.insert(
            header::CACHE_CONTROL,
            axum::http::HeaderValue::from_static("private, no-store"),
        );
    } else if !parts.headers.contains_key(header::CACHE_CONTROL) {
        parts.headers.insert(
            header::CACHE_CONTROL,
            axum::http::HeaderValue::from_static(
                "public, s-maxage=60, stale-while-revalidate=86400",
            ),
        );
    }

    parts.headers.insert("X-Cache", "MISS".parse().unwrap());

    if is_get && parts.status.is_success() && !private_response {
        let mut tags_vec = Vec::new();
        if let Some(surrogate) = parts.headers.get("Surrogate-Key")
            && let Ok(s) = surrogate.to_str()
        {
            for t in s.split(' ') {
                if !t.is_empty() {
                    tags_vec.push(t.to_string());
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

        cdn_cache
            .set_with_tags(
                &cache_key,
                cached_response,
                tags_vec,
                std::time::Duration::from_secs(60),
            )
            .await;
    }

    let new_body = Body::from(bytes.to_vec());
    let new_response = Response::from_parts(parts, new_body);
    Ok(new_response.into_response())
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::{
        Router,
        body::{Body, to_bytes},
        http::{Request, Response, StatusCode},
        middleware::from_fn,
        routing::get,
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
        let req1 = Request::builder()
            .uri("/bypass")
            .body(Body::empty())
            .unwrap();
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
            .route(
                "/surrogate",
                get(|| async {
                    let mut res = Response::new(Body::from("Surrogate Content"));
                    res.headers_mut()
                        .insert("Cache-Tag", "tag1, tag2".parse().unwrap());
                    res
                }),
            )
            .layer(from_fn(edge_caching_middleware));

        let req = Request::builder()
            .uri("/surrogate")
            .body(Body::empty())
            .unwrap();
        let res = app.oneshot(req).await.unwrap();

        assert_eq!(res.headers().get("Surrogate-Key").unwrap(), "tag1 tag2");
        assert_eq!(res.headers().get("X-Cache").unwrap(), "MISS");
    }
    #[tokio::test]
    async fn authenticated_requests_never_reuse_a_public_uri_cache_entry() {
        let path = format!("/private-{}", uuid::Uuid::new_v4());
        get_cdn_cache()
            .set(
                &format!("cdn:{path}"),
                CachedResponse {
                    status: 200,
                    headers: vec![],
                    body: b"Old public response".to_vec(),
                },
                std::time::Duration::from_secs(60),
            )
            .await;
        let app = Router::new()
            .route(&path, get(|| async { "Current private response" }))
            .layer(from_fn(edge_caching_middleware));
        for name in [header::AUTHORIZATION, header::COOKIE] {
            let response = app
                .clone()
                .oneshot(
                    Request::builder()
                        .uri(&path)
                        .header(name, "explicit-fixture-value")
                        .body(Body::empty())
                        .unwrap(),
                )
                .await
                .unwrap();
            assert_eq!(
                response.headers()[header::CACHE_CONTROL],
                "private, no-store"
            );
            assert_eq!(
                to_bytes(response.into_body(), 1024).await.unwrap(),
                "Current private response"
            );
        }
    }

    #[tokio::test]
    async fn private_and_cookie_responses_never_enter_the_shared_cache() {
        use std::sync::{
            Arc,
            atomic::{AtomicUsize, Ordering},
        };
        for (name, value) in [
            (header::CACHE_CONTROL, "private=\"user\", max-age=60"),
            (header::SET_COOKIE, "local-fixture=value; HttpOnly"),
        ] {
            let path = format!("/private-response-{}", uuid::Uuid::new_v4());
            let hits = Arc::new(AtomicUsize::new(0));
            let observed = hits.clone();
            let app = Router::new()
                .route(
                    &path,
                    get(move || {
                        let hits = hits.clone();
                        let name = name.clone();
                        async move {
                            let mut response = Response::new(Body::from(format!(
                                "Private {}",
                                hits.fetch_add(1, Ordering::SeqCst)
                            )));
                            response.headers_mut().insert(name, value.parse().unwrap());
                            response
                        }
                    }),
                )
                .layer(from_fn(edge_caching_middleware));
            for expected in ["Private 0", "Private 1"] {
                let response = app
                    .clone()
                    .oneshot(Request::builder().uri(&path).body(Body::empty()).unwrap())
                    .await
                    .unwrap();
                assert_eq!(
                    response.headers()[header::CACHE_CONTROL],
                    "private, no-store"
                );
                assert_eq!(
                    to_bytes(response.into_body(), 1024).await.unwrap(),
                    expected
                );
            }
            assert_eq!(observed.load(Ordering::SeqCst), 2);
        }
    }
}
