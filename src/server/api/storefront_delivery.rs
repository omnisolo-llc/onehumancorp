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
        if let Ok((mut html, tags)) = regenerate_cache(state.pool.clone(), tenant_id, site_id, cache_key.clone(), cache.clone()).await {
            #[derive(sqlx::FromRow)]
            struct ProductSeoRow {
                seo_title: Option<String>,
                seo_description: Option<String>,
                seo_schema_json: Option<sqlx::types::Json<serde_json::Value>>,
            }

            // Check if we have SEO metadata for this product
            let seo_res = sqlx::query_as::<_, ProductSeoRow>(
                "SELECT seo_title, seo_description, seo_schema_json FROM products WHERE id = $1 AND tenant_id = $2",
            )
            .bind(product_id.to_string())
            .bind(tenant_id.to_string())
            .fetch_optional(&state.pool)
            .await;

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

            let mut response = Html(html).into_response();
            if !tags.is_empty() {
                if let Ok(cache_tag) = tags.join(", ").parse() {
                    response.headers_mut().insert("Cache-Tag", cache_tag);
                }
            }
            response.headers_mut().insert(
                axum::http::header::CACHE_CONTROL,
                "public, s-maxage=60, stale-while-revalidate=86400".parse().unwrap(),
            );
            return Ok(response);
        }
    }

    // Fallback simple HTML
    let mut response = Html(format!("<!DOCTYPE html><html><body>Product {} not found</body></html>", product_id)).into_response();
    response.headers_mut().insert(
        axum::http::header::CACHE_CONTROL,
        "public, max-age=10".parse().unwrap(),
    );
    Ok(response)
}
