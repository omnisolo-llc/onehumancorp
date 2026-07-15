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

pub async fn inject_dynamic_inventory(
    html: &str,
    tenant_id_opt: Option<String>,
) -> String {
    let mut html_str = html.to_string();
    let mut offset = 0;
    while let Some(start) = html_str[offset..].find("<!-- INVENTORY_STATUS_") {
        let actual_start = offset + start;
        let prefix_len = "<!-- INVENTORY_STATUS_".len();
        if let Some(end) = html_str[actual_start + prefix_len..].find(" -->") {
            let actual_end = actual_start + prefix_len + end;
            let pid = &html_str[actual_start + prefix_len..actual_end];
            let pid_str = pid.to_string();

            let mut inventory_count: i32 = 0;

            if let Some(ref tenant_id) = tenant_id_opt {
                let kv_key = format!("tenant:{}:product:{}:inventory", tenant_id, pid_str);

                // Fetch from Edge KV if redis_url is available
                if let Ok(url) = std::env::var("REDIS_URL") {
                    if let Ok(client) = redis::Client::open(url) {
                        if let Ok(mut conn) = client.get_multiplexed_async_connection().await {
                            let val_res: Result<Option<String>, _> = redis::cmd("GET").arg(&kv_key).query_async(&mut conn).await;
                            if let Ok(Some(val)) = val_res {
                                if let Ok(parsed_val) = val.parse::<i32>() {
                                    inventory_count = parsed_val;
                                }
                            }
                        }
                    }
                }
            }

            let replacement = if inventory_count <= 0 {
                "<span class=\"sold-out\" style=\"color: #E30000; font-weight: 600; font-size: 14px;\">Sold Out</span>"
            } else {
                ""
            };

            html_str.replace_range(actual_start..(actual_end + 4), replacement);

            offset = actual_start + replacement.len();
        } else {
            break;
        }
    }
    html_str
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
            let mut tenant_id_opt = None;
            for (k, v) in &cached.headers {
                if k.eq_ignore_ascii_case("surrogate-key") || k.eq_ignore_ascii_case("cache-tag") {
                    for tag in v.split(&[' ', ','][..]) {
                        if tag.starts_with("tenant-id:") {
                            tenant_id_opt = Some(tag.trim_start_matches("tenant-id:").to_string());
                            break;
                        }
                    }
                }
            }

            let body_str = String::from_utf8_lossy(&cached.body);
            let injected_html = inject_dynamic_inventory(&body_str, tenant_id_opt).await;
            let injected_bytes = injected_html.into_bytes();

            let mut response = Response::builder()
                .status(cached.status)
                .body(Body::from(injected_bytes.clone()))
                .unwrap();

            for (k, v) in cached.headers {
                if let (Ok(hk), Ok(hv)) = (axum::http::HeaderName::try_from(k), axum::http::HeaderValue::try_from(v)) {
                    if hk == axum::http::header::CONTENT_LENGTH {
                        continue;
                    }
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

    let mut tenant_id_opt = None;
    if let Some(surrogate) = parts.headers.get("Surrogate-Key") {
        if let Ok(s) = surrogate.to_str() {
            for tag in s.split(&[' ', ','][..]) {
                if tag.starts_with("tenant-id:") {
                    tenant_id_opt = Some(tag.trim_start_matches("tenant-id:").to_string());
                    break;
                }
            }
        }
    }
    let body_str = String::from_utf8_lossy(&bytes);
    let injected_html = inject_dynamic_inventory(&body_str, tenant_id_opt).await;
    let new_body = Body::from(injected_html.into_bytes());
    parts.headers.remove(axum::http::header::CONTENT_LENGTH);
    let new_response = Response::from_parts(parts, new_body);
    Ok(new_response.into_response())
}

