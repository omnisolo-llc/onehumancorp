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
}


pub struct CacheInvalidationService {
    cache: Arc<HybridCache<String>>,
}

impl CacheInvalidationService {
    pub fn new(cache: Arc<HybridCache<String>>) -> Self {
        Self { cache }
    }

    pub async fn invalidate(&self, tags: Vec<String>) {
        for tag in tags {
            self.cache.invalidate_by_tag(&tag).await;
        }
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
        for tag in tags_to_invalidate {
            cdn.invalidate_by_tag(&tag).await;
        }
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
                    let _ = regenerate_storefront_product(pool_clone, tenant_id, product_id, cache_key_clone.clone(), cache_clone).await;
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

    let result = regenerate_storefront_product(state.pool.clone(), tenant_id, product_id, cache_key.clone(), cache.clone()).await;

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

async fn regenerate_storefront_product(
    pool: PgPool,
    tenant_id: Uuid,
    product_id: Uuid,
    cache_key: String,
    cache: std::sync::Arc<crate::utils::cache::HybridCache<String>>,
) -> Result<(String, Vec<String>), StatusCode> {
    #[derive(sqlx::FromRow)]
    struct ProductSeoRow {
        seo_title: Option<String>,
        seo_description: Option<String>,
        seo_schema_json: Option<sqlx::types::Json<serde_json::Value>>,
    }

    let pool1 = pool.clone();
    let pool2 = pool.clone();
    let (site_id_res, seo_res) = tokio::join!(
        async move {
            sqlx::query_scalar::<_, Uuid>(
                "SELECT id FROM builder_sites WHERE tenant_id = $1 ORDER BY created_at ASC LIMIT 1"
            )
            .bind(tenant_id)
            .fetch_one(&pool1)
            .await
        },
        async move {
            sqlx::query_as::<_, ProductSeoRow>(
                "SELECT seo_title, seo_description, seo_schema_json FROM products WHERE id = $1 AND tenant_id = $2"
            )
            .bind(product_id.to_string())
            .bind(tenant_id.to_string())
            .fetch_optional(&pool2)
            .await
        }
    );

    if let Ok(site_id) = site_id_res {
        // Just call regenerate_cache from builder edge
        if let Ok((mut html, tags)) = regenerate_cache(pool.clone(), tenant_id, site_id, cache_key.clone(), cache.clone()).await {

            if let Ok(Some(row)) = seo_res {
                if let Some(seo_title) = row.seo_title {
                    if let Some(start) = html.find("<title>") {
                        if let Some(end) = html[start..].find("</title>") {
                            let end = start + end + "</title>".len();
                            html.replace_range(start..end, &format!("<title>{}</title>\n<meta name=\"title\" content=\"{}\">", seo_title.replace("&", "&amp;").replace("<", "&lt;").replace(">", "&gt;"), seo_title.replace("&", "&amp;").replace("<", "&lt;").replace(">", "&gt;")));
                        }
                    }
                }

                if let Some(seo_desc) = row.seo_description {
                    if let Some(start) = html.find("<meta name=\"description\"") {
                        if let Some(end) = html[start..].find(">") {
                            let end = start + end + ">".len();
                            html.replace_range(start..end, &format!("<meta name=\"description\" content=\"{}\">", seo_desc.replace("&", "&amp;").replace("<", "&lt;").replace(">", "&gt;")));
                        }
                    } else if let Some(head_end) = html.find("</head>") {
                        html.insert_str(head_end, &format!("<meta name=\"description\" content=\"{}\">\n", seo_desc.replace("&", "&amp;").replace("<", "&lt;").replace(">", "&gt;")));
                    }
                }

                if let Some(seo_schema) = row.seo_schema_json {
                    if let Some(start) = html.find("<script type=\"application/ld+json\">") {
                        if let Some(end) = html[start..].find("</script>") {
                            let end = start + end + "</script>".len();
                            html.replace_range(start..end, &format!("<script type=\"application/ld+json\">\n{}\n</script>", serde_json::to_string(&seo_schema.0).unwrap_or_default()));
                        }
                    } else if let Some(head_end) = html.find("</head>") {
                        html.insert_str(head_end, &format!("<script type=\"application/ld+json\">\n{}\n</script>\n", serde_json::to_string(&seo_schema.0).unwrap_or_default()));
                    }
                }
            }

            // Pre-warm the cache since SWR or cache miss just resolved
            cache.set_with_tags(&cache_key, html.clone(), tags.clone(), std::time::Duration::from_secs(3600)).await;

            return Ok((html, tags));
        }
    }
    Err(StatusCode::NOT_FOUND)
}


fn set_storefront_headers(response: &mut axum::response::Response, html: &str, tenant_id: Uuid, custom_tags: Option<Vec<String>>) {
    let mut hasher = Sha256::new();
    hasher.update(html.as_bytes());
    let result = hasher.finalize();
    let etag = format!("\"{:x}\"", result);

    let mut cache_tag = format!("tenant-id:{}", tenant_id);
    if let Some(tags) = custom_tags {
        if !tags.is_empty() {
            cache_tag = tags.join(", ");
        }
    }

    if let Ok(val) = cache_tag.parse() {
        response.headers_mut().insert("Cache-Tag", val);
    }

    if let Ok(val) = cache_tag.parse() {
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
