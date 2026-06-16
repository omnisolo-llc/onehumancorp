use axum::{
    extract::{Extension, Path, State},
    response::{Html, IntoResponse},
    routing::{get, post},
    Json, Router,
};
use http::StatusCode;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use uuid::Uuid;
use crate::builder::edge::{get_edge_cache, regenerate_cache};

#[derive(Clone)]
pub struct DeliveryState {
    pub pool: PgPool,
}

pub fn router() -> Router<DeliveryState> {
    Router::new()
        .route("/:tenant_id/:product_id", get(get_storefront_product))
        .route("/webhook/invalidate", post(invalidate_cache_webhook))
}

#[derive(Deserialize)]
pub struct InvalidateRequest {
    pub tags: Vec<String>,
}

async fn invalidate_cache_webhook(
    State(state): State<DeliveryState>,
    Json(payload): Json<InvalidateRequest>,
) -> impl IntoResponse {
    let cache = get_edge_cache();
    for tag in payload.tags {
        cache.invalidate_by_tag(&tag).await;
    }
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
        if !is_stale {
            let mut response = Html(cached_html).into_response();
            response.headers_mut().insert(
                http::header::CACHE_CONTROL,
                "public, s-maxage=31536000, stale-while-revalidate=86400, max-age=0".parse().unwrap(),
            );
            return Ok(response);
        }
    }

    // In a real scenario we might render directly here if it's missing,
    // but the builder edge regenerate_cache logic expects a site_id.
    // Let's find the primary site for this tenant.
    let site_id_res = sqlx::query_scalar::<_, Uuid>(
        "SELECT id FROM builder_sites WHERE tenant_id = $1 ORDER BY created_at ASC LIMIT 1"
    )
    .bind(tenant_id)
    .fetch_one(&state.pool)
    .await;

    if let Ok(site_id) = site_id_res {
        // Just call regenerate_cache from builder edge
        if let Ok((html, tags)) = regenerate_cache(state.pool.clone(), tenant_id, site_id, cache_key.clone(), cache.clone()).await {
            let mut response = Html(html).into_response();
            if !tags.is_empty() {
                if let Ok(cache_tag) = tags.join(", ").parse() {
                    response.headers_mut().insert("Cache-Tag", cache_tag);
                }
            }
            response.headers_mut().insert(
                http::header::CACHE_CONTROL,
                "public, s-maxage=31536000, stale-while-revalidate=86400, max-age=0".parse().unwrap(),
            );
            return Ok(response);
        }
    }

    // Fallback simple HTML
    let mut response = Html(format!("<!DOCTYPE html><html><body>Product {} not found</body></html>", product_id)).into_response();
    response.headers_mut().insert(
        http::header::CACHE_CONTROL,
        "public, max-age=10".parse().unwrap(),
    );
    Ok(response)
}
