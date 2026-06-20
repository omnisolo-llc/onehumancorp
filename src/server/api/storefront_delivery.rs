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
use crate::builder::edge::{get_edge_cache, regenerate_cache};

#[derive(Clone)]
pub struct DeliveryState {
    pub pool: PgPool,
}

pub fn router() -> Router<DeliveryState> {
    Router::new()
        .route("/{tenant_id}/{product_id}", get(get_storefront_product))
        .route("/webhook/invalidate", post(invalidate_cache_webhook))
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
                axum::http::header::CACHE_CONTROL,
                "public, s-maxage=60, stale-while-revalidate=86400".parse().unwrap(),
            );
            return Ok(response);
        }
    }

    // Fetch product details for edge rendering
    let tenant_id_str_q = tenant_id.to_string();
    let product_id_str_q = product_id.to_string();
    let product_res = sqlx::query(
        "SELECT name, description, price, seo_title, seo_description, metadata FROM products WHERE tenant_id = $1 AND id = $2"
    )
    .bind(tenant_id_str_q)
    .bind(product_id_str_q)
    .fetch_one(&state.pool)
    .await;

    if let Ok(product) = product_res {
        use sqlx::Row;
        let p_name: Option<String> = product.try_get("name").unwrap_or(None);
        let p_desc: Option<String> = product.try_get("description").unwrap_or(None);
        let p_price: Option<f64> = product.try_get("price").unwrap_or(None);
        let p_seo_title: Option<String> = product.try_get("seo_title").unwrap_or(None);
        let p_seo_desc: Option<String> = product.try_get("seo_description").unwrap_or(None);
        let p_metadata: Option<serde_json::Value> = product.try_get("metadata").unwrap_or(None);

        let fallback_name = p_name.unwrap_or_else(|| "Unknown Product".to_string());
        let fallback_desc = p_desc.unwrap_or_else(|| "".to_string());
        let title = p_seo_title.unwrap_or(fallback_name);
        let description = p_seo_desc.unwrap_or(fallback_desc);
        let price = p_price.unwrap_or(0.0);

        let seo_schema = if let Some(metadata) = p_metadata {
            metadata.get("seo_schema").cloned().unwrap_or_else(|| serde_json::json!({}))
        } else {
            serde_json::json!({})
        };

        let html = format!(
            r#"<!DOCTYPE html>
<html>
<head>
    <title>{}</title>
    <meta name="description" content="{}">
    <script type="application/ld+json">
{}
    </script>
</head>
<body>
    <h1>{}</h1>
    <p>{}</p>
    <p>Price: ${}</p>
</body>
</html>"#,
            crate::builder::edge::escape_html(&title),
            crate::builder::edge::escape_html(&description),
            seo_schema,
            crate::builder::edge::escape_html(&title),
            crate::builder::edge::escape_html(&description),
            price
        );

        cache.set_with_tags(&cache_key, html.clone(), vec![format!("tenant-id:{}", tenant_id), format!("entity:product:{}", product_id)], std::time::Duration::from_secs(3600)).await;

        let mut response = Html(html).into_response();
        response.headers_mut().insert(
            axum::http::header::CACHE_CONTROL,
            "public, s-maxage=60, stale-while-revalidate=86400".parse().unwrap(),
        );
        return Ok(response);
    }

    // Fallback simple HTML
    let mut response = Html(format!("<!DOCTYPE html><html><body>Product {} not found</body></html>", product_id)).into_response();
    response.headers_mut().insert(
        axum::http::header::CACHE_CONTROL,
        "public, max-age=10".parse().unwrap(),
    );
    Ok(response)
}
