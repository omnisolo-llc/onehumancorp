use axum::{
    extract::{Path, State},
    response::{Html, IntoResponse},
    routing::{get, post},
    Json, Router,
};
use axum::http::StatusCode;
use serde::Deserialize;
use sqlx::PgPool;
use uuid::Uuid;
use sha2::{Digest, Sha256};
use std::sync::Arc;
use crate::utils::cache::HybridCache;
use crate::builder::edge::{get_edge_cache, regenerate_cache, get_ongoing_generation, inject_dynamic_inventory};

#[derive(Clone)]
pub struct DeliveryState {
    pub pool: PgPool,
}

pub fn router() -> Router<DeliveryState> {
    Router::new()
        .route("/{tenant_id}/{product_id}", get(get_storefront_product).layer(axum::middleware::from_fn(crate::utils::edge_caching_middleware::edge_caching_middleware)))
        .route("/webhook/invalidate", post(invalidate_cache_webhook))
        .route("/resolve_domain", get(resolve_domain).layer(axum::middleware::from_fn(crate::utils::edge_caching_middleware::edge_caching_middleware)))
}

async fn resolve_domain(
    axum::extract::State(state): axum::extract::State<DeliveryState>,
    req: axum::extract::Request,
) -> Result<impl IntoResponse, StatusCode> {
    let host = req.headers().get("X-Forwarded-Host")
        .or_else(|| req.headers().get("Host"))
        .and_then(|h| h.to_str().ok())
        .ok_or(StatusCode::BAD_REQUEST)?;

    // Remove port if present
    let domain = host.split(':').next().unwrap_or(host).to_string();

    let (site_id, tenant_id) = match sqlx::query_as::<_, (Uuid, Uuid)>(
        "SELECT id, tenant_id FROM builder_sites WHERE domain = $1"
    )
    .bind(&domain)
    .fetch_optional(&state.pool)
    .await {
        Ok(Some(res)) => res,
        _ => return Err(StatusCode::NOT_FOUND),
    };

    let cache = get_edge_cache();
    let cache_key = format!("edge_site_{}_{}_{}", tenant_id, site_id, "en-US");

    if let Some((cached_html, is_stale)) = cache.get_with_swr(&cache_key).await {
        let mut response = Html(cached_html.clone()).into_response();
        set_storefront_headers(&mut response, &cached_html, tenant_id, None);
        if !is_stale {
            return Ok(response);
        } else {
            let pool_clone = state.pool.clone();
            let cache_key_clone = cache_key.clone();
            let cache_clone = cache.clone();
            tokio::spawn(async move {
                let _ = regenerate_cache(pool_clone, tenant_id, site_id, cache_key_clone, cache_clone).await;
            });
            return Ok(response);
        }
    }

    if let Ok((html, tags)) = regenerate_cache(state.pool.clone(), tenant_id, site_id, cache_key.clone(), cache.clone()).await {
        let mut response = Html(html.clone()).into_response();
        set_storefront_headers(&mut response, &html, tenant_id, Some(tags));
        return Ok(response);
    }

    Err(StatusCode::NOT_FOUND)
}

pub struct CacheInvalidationService {
    cache: Arc<HybridCache<String>>,
}

impl CacheInvalidationService {
    pub fn new(cache: Arc<HybridCache<String>>) -> Self {
        Self { cache }
    }

    pub async fn invalidate(&self, tags: Vec<String>) {
        let futures = tags.into_iter().map(|tag| async move {
            self.cache.invalidate_by_tag(&tag).await;
        });
        futures::future::join_all(futures).await;
    }
}

#[derive(Deserialize)]
pub struct InvalidateRequest {
    pub tags: Vec<String>,
}

async fn invalidate_cache_webhook(
    State(_state): State<DeliveryState>,
    Json(payload): Json<InvalidateRequest>,
) -> impl IntoResponse {
    let cache = get_edge_cache();
    let service = CacheInvalidationService::new(cache);
    service.invalidate(payload.tags.clone()).await;

    let tags_to_invalidate = payload.tags;
    tokio::spawn(async move {
        let cdn = crate::utils::edge_caching_middleware::get_cdn_cache();
        let client = reqwest::Client::new();
        let futures = tags_to_invalidate.into_iter().map(|tag| {
            let cdn_clone = cdn.clone();
            let client_clone = client.clone();
            async move {
                cdn_clone.invalidate_by_tag(&tag).await;

                // Send purge request to NGINX Edge Cache
                if let Err(e) = client_clone.post("http://edge-cache/purge")
                    .body(tag.clone())
                    .send()
                    .await
                {
                    tracing::warn!("Failed to send purge request to NGINX for tag {}: {}", tag, e);
                } else {
                    tracing::info!("Successfully sent purge request to NGINX for tag {}", tag);
                }
            }
        });
        futures::future::join_all(futures).await;
    });

    StatusCode::OK
}


async fn get_storefront_product(
    State(state): State<DeliveryState>,
    Path((tenant_id_str, product_id_str)): Path<(String, String)>,
) -> Result<impl IntoResponse, StatusCode> {
    let tenant_id = Uuid::parse_str(&tenant_id_str).map_err(|_| StatusCode::BAD_REQUEST)?;
    let product_id = Uuid::parse_str(&product_id_str).map_err(|_| StatusCode::BAD_REQUEST)?;

    let cache = get_edge_cache();
    let cache_key = format!("storefront:product:{}:{}", tenant_id, product_id);

    if let Some((cached_html, is_stale)) = cache.get_with_swr(&cache_key).await {
        let html = inject_dynamic_inventory(cached_html, tenant_id, &state.pool, cache.clone()).await;
        let mut response = Html(html.clone()).into_response();
        set_storefront_headers(&mut response, &html, tenant_id, None);

        if !is_stale {
            return Ok(response);
        } else {
            let ongoing = get_ongoing_generation();
            let mut guard = ongoing.lock().await;
            if !guard.contains(&cache_key) {
                guard.insert(cache_key.clone());
                let pool_clone = state.pool.clone();
                let cache_key_clone = cache_key.clone();
                let cache_clone = cache.clone();
                tokio::spawn(async move {
                    let _ = crate::builder::edge::regenerate_product_cache(pool_clone, tenant_id, product_id, cache_key_clone.clone(), cache_clone).await;
                    let ongoing = get_ongoing_generation();
                    ongoing.lock().await.remove(&cache_key_clone);
                });
            }
            return Ok(response);
        }
    }

    let ongoing = get_ongoing_generation();
    let is_generating = {
        let mut guard = ongoing.lock().await;
        if guard.contains(&cache_key) {
            true
        } else {
            guard.insert(cache_key.clone());
            false
        }
    };

    if is_generating {
        tokio::time::sleep(std::time::Duration::from_millis(500)).await;
        if let Some((cached_html, _)) = cache.get_with_swr(&cache_key).await {
            let html = inject_dynamic_inventory(cached_html, tenant_id, &state.pool, cache.clone()).await;
            let mut response = Html(html.clone()).into_response();
        set_storefront_headers(&mut response, &html, tenant_id, None);
            return Ok(response);
        }
    }

    let result = crate::builder::edge::regenerate_product_cache(state.pool.clone(), tenant_id, product_id, cache_key.clone(), cache.clone()).await;

    {
        let ongoing = get_ongoing_generation();
        ongoing.lock().await.remove(&cache_key);
    }

    if let Ok((html, tags)) = result {
        let final_html = inject_dynamic_inventory(html, tenant_id, &state.pool, cache.clone()).await;
        let mut response = Html(final_html.clone()).into_response();
        set_storefront_headers(&mut response, &final_html, tenant_id, Some(tags));
        return Ok(response);
    }

    // Fallback simple HTML
    let html = format!("<!DOCTYPE html><html><body>Product {} not found</body></html>", product_id);
    let mut response = Html(html.clone()).into_response();
    set_storefront_headers(&mut response, &html, tenant_id, None);
    Ok(response)
}



fn set_storefront_headers(response: &mut axum::response::Response, html: &str, tenant_id: Uuid, custom_tags: Option<Vec<String>>) {
    let mut hasher = Sha256::new();
    hasher.update(html.as_bytes());
    let result = hasher.finalize();
    let etag = format!("\"{:x}\"", result);

    let mut tags = vec![format!("tenant-id:{}", tenant_id)];
    if let Some(mut ct) = custom_tags {
        tags.append(&mut ct);
    }

    let cache_tag_comma = tags.join(", ");
    let surrogate_key_space = tags.join(" ");

    if let Ok(val) = cache_tag_comma.parse() {
        response.headers_mut().insert("Cache-Tag", val);
    }

    if let Ok(val) = surrogate_key_space.parse() {
        response.headers_mut().insert("Surrogate-Key", val);
    }

    if let Ok(val) = etag.parse() {
        response.headers_mut().insert("ETag", val);
    }

    response.headers_mut().insert(
        axum::http::header::CACHE_CONTROL,
        "public, s-maxage=60, stale-while-revalidate=86400".parse().unwrap(),
    );
}

#[cfg(test)]

mod tests {

    #[test]
    fn test_storefront_headers_dummy() {
        assert!(true);
    }
}
