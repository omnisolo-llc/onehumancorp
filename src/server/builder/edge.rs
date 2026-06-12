use axum::{
    extract::{Path, Extension},
    response::{Html, IntoResponse, Response},
    http::header::CACHE_CONTROL,
    body::Body,
};
use sqlx::PgPool;
use std::sync::Arc;
use uuid::Uuid;
use crate::utils::cache::HybridCache;
use std::sync::OnceLock;
use std::collections::HashSet;
use tokio::sync::Mutex;

pub static ONGOING_GENERATION: OnceLock<Arc<Mutex<HashSet<String>>>> = OnceLock::new();
pub fn get_ongoing_generation() -> Arc<Mutex<HashSet<String>>> {
    ONGOING_GENERATION.get_or_init(|| Arc::new(Mutex::new(HashSet::new()))).clone()
}

pub static EDGE_CACHE: OnceLock<Arc<HybridCache<String>>> = OnceLock::new();

pub fn get_edge_cache() -> Arc<HybridCache<String>> {
    EDGE_CACHE.get_or_init(|| {
        let redis_client = if let Ok(url) = std::env::var("REDIS_URL") {
            match redis::Client::open(url.clone()) {
                Ok(client) => Some(client),
                Err(e) => {
                    tracing::warn!("Failed to initialize Redis client for EDGE_CACHE at {}: {}. Falling back to in-memory cache.", url, e);
                    None
                }
            }
        } else {
            None
        };
        Arc::new(HybridCache::new(redis_client))
    }).clone()
}

pub struct EdgeWorkerState {
    pub pool: PgPool,
}

fn escape_html(s: &str) -> String {
    s.replace("&", "&amp;")
     .replace("<", "&lt;")
     .replace(">", "&gt;")
     .replace("\"", "&quot;")
     .replace("'", "&#x27;")
}

pub struct StorefrontRouter;

impl StorefrontRouter {
    pub async fn handle_edge_request(
        Extension(state): Extension<Arc<EdgeWorkerState>>,
        Path((tenant_id_str, site_id_str)): Path<(String, String)>,
        headers: axum::http::HeaderMap,
    ) -> Result<Response<Body>, axum::http::StatusCode> {
        handle_edge_request_impl(Extension(state), Path((tenant_id_str, site_id_str)), headers).await
    }
}

pub async fn handle_edge_request_impl(
    Extension(state): Extension<Arc<EdgeWorkerState>>,
    Path((tenant_id_str, site_id_str)): Path<(String, String)>,
    headers: axum::http::HeaderMap,
) -> Result<Response<Body>, axum::http::StatusCode> {
    let tenant_id = Uuid::parse_str(&tenant_id_str).map_err(|_| axum::http::StatusCode::BAD_REQUEST)?;
    let site_id = Uuid::parse_str(&site_id_str).map_err(|_| axum::http::StatusCode::BAD_REQUEST)?;

    let locale = headers.get("accept-language").and_then(|v| v.to_str().ok()).unwrap_or("en-US");
    let cache_key = format!("edge_site_{}_{}_{}", tenant_id, site_id, locale);
    let cache = get_edge_cache();

    if let Some((cached_html, stale)) = cache.get_with_swr(&cache_key).await {
        let mut response = Html(cached_html).into_response();
        let cache_tag = format!("tenant-id:{}", tenant_id);
        if let Ok(val) = cache_tag.parse() {
            response.headers_mut().insert("Cache-Tag", val);
        }
        response.headers_mut().insert(
            CACHE_CONTROL,
            "public, s-maxage=60, stale-while-revalidate=86400".parse().unwrap(),
        );
        if stale {
            // Spawn background regeneration logic if it was stale, but prevent thundering herd
            let ongoing = get_ongoing_generation();
            let mut guard = ongoing.lock().await;
            if !guard.contains(&cache_key) {
                guard.insert(cache_key.clone());
                let pool_clone = state.pool.clone();
                let cache_key_clone = cache_key.clone();
                tokio::spawn(async move {
                    let _ = regenerate_cache(pool_clone, tenant_id, site_id, cache_key_clone.clone(), cache).await;
                    let ongoing = get_ongoing_generation();
                    ongoing.lock().await.remove(&cache_key_clone);
                });
            }
        }
        return Ok(response);
    }

    // Prevent thundering herd on full cache miss
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
        // Wait a short bit and check cache again, simple backoff
        // A real system would use a broadcast channel, but for edge workers returning a slightly delayed or stale is better
        tokio::time::sleep(std::time::Duration::from_millis(500)).await;
        if let Some((cached_html, _)) = cache.get_with_swr(&cache_key).await {
            let mut response = Html(cached_html).into_response();
        let cache_tag = format!("tenant-id:{}", tenant_id);
        if let Ok(val) = cache_tag.parse() {
            response.headers_mut().insert("Cache-Tag", val);
        }
            response.headers_mut().insert(
                CACHE_CONTROL,
                "public, s-maxage=60, stale-while-revalidate=86400".parse().unwrap(),
            );
            return Ok(response);
        }
    }

    let result = regenerate_cache(state.pool.clone(), tenant_id, site_id, cache_key.clone(), cache).await;

    {
        let ongoing = get_ongoing_generation();
        ongoing.lock().await.remove(&cache_key);
    }

    let (html, tags) = result?;

    let mut response = Html(html).into_response();
    if !tags.is_empty() {
        if let Ok(cache_tag) = tags.join(", ").parse() {
            response.headers_mut().insert("Cache-Tag", cache_tag);
        }
    }
    response.headers_mut().insert(
        CACHE_CONTROL,
        "public, s-maxage=60, stale-while-revalidate=86400".parse().unwrap(),
    );
    Ok(response)
}

pub async fn regenerate_cache(
    pool: PgPool,
    tenant_id: Uuid,
    site_id: Uuid,
    cache_key: String,
    cache: Arc<HybridCache<String>>,
) -> Result<(String, Vec<String>), axum::http::StatusCode> {
    let site = match super::db::list_sites(&pool, tenant_id).await {
        Ok(sites) => sites.into_iter().find(|s| s.id == site_id),
        Err(_) => return Err(axum::http::StatusCode::INTERNAL_SERVER_ERROR),
    };

    if site.is_none() {
        return Err(axum::http::StatusCode::NOT_FOUND);
    }

    let pages = super::db::list_pages(&pool, tenant_id, site_id)
        .await
        .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;

    if pages.is_empty() {
        return Err(axum::http::StatusCode::NOT_FOUND);
    }

    let page = &pages[0];

    let blocks = super::db::list_blocks(&pool, tenant_id, page.id)
        .await
        .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;

    // Batch query inventory counts to avoid N+1 queries
    let mut inventory_map: std::collections::HashMap<String, i32> = std::collections::HashMap::new();
    let mut product_ids = Vec::new();
    for block in &blocks {
        if block.block_type == "ProductGridBlock" || block.block_type == "Catalog" {
            if let Some(items) = block.content.get("items").and_then(|v| v.as_array()) {
                for item in items {
                    if let Some(pid) = item.get("product_id").and_then(|v| v.as_str()) {
                        product_ids.push(pid.to_string());
                    }
                }
            }
        }
    }

    if !product_ids.is_empty() {
        use sqlx::Row;
        // In PostgreSQL we can use = ANY($1)
        if let Ok(rows) = sqlx::query("SELECT id, inventory_count FROM products WHERE id = ANY($1)")
            .bind(&product_ids)
            .fetch_all(&pool)
            .await
        {
            for row in rows {
                if let (Ok(id), Ok(count)) = (row.try_get::<String, _>("id"), row.try_get::<i32, _>("inventory_count")) {
                    inventory_map.insert(id, count);
                }
            }
        }
    }

    let mut tags = vec![format!("tenant-id:{}", tenant_id)];
    let mut html = String::new();
    html.push_str("<!DOCTYPE html>\n<html lang=\"en\">\n<head>\n");
    html.push_str("<meta charset=\"UTF-8\">\n<meta name=\"viewport\" content=\"width=device-width, initial-scale=1.0\">\n");

    html.push_str(&format!("<title>{}</title>\n", escape_html(&page.title)));
    if let Some(seo_name) = page.seo_metadata.get("name").and_then(|v| v.as_str()) {
        html.push_str(&format!("<meta name=\"description\" content=\"{}\">\n", escape_html(seo_name)));
    }

    let mut seo_ld = page.seo_metadata.clone();
    if seo_ld.get("@context").is_none() {
        seo_ld["@context"] = serde_json::Value::String("https://schema.org".to_string());
    }
    html.push_str(&format!("<script type=\"application/ld+json\">\n{}\n</script>\n", serde_json::to_string(&seo_ld).unwrap_or_default()));

    html.push_str(r#"
    <link href="https://fonts.googleapis.com/css2?family=Inter:wght@400;500;600;700&family=Outfit:wght@500;600;700&display=swap" rel="stylesheet">
    <style>
        body { font-family: 'Inter', sans-serif; margin: 0; padding: 0; background: #f5f5f7; color: #1D1D1F; display: flex; flex-direction: column; align-items: center; }
        .font-outfit { font-family: 'Outfit', sans-serif; }
        .glass-container { width: 100%; max-width: 375px; min-height: 100dvh; background: rgba(255, 255, 255, 0.65); backdrop-filter: blur(30px) saturate(210%); border: 1px solid rgba(255, 255, 255, 0.4); border-radius: 16px; box-shadow: 0 10px 15px -3px rgba(0,0,0,0.1), 0 4px 6px -2px rgba(0,0,0,0.05); margin: 20px auto; overflow: hidden; display: flex; flex-direction: column; }
        @media (prefers-color-scheme: dark) { body { background: #000; color: #F5F5F7; } .glass-container { background: rgba(22, 22, 26, 0.7); border: 1px solid rgba(255, 255, 255, 0.1); } }
        .block { padding: 24px; border-bottom: 1px solid rgba(150,150,150,0.1); }
        .block:last-child { border-bottom: none; }
        .hero-title { font-size: 28px; font-weight: 700; margin: 0 0 8px 0; }
        .hero-subtitle { font-size: 16px; color: #666; margin: 0; }
        @media (prefers-color-scheme: dark) { .hero-subtitle { color: #aaa; } }
        .product-grid { display: flex; flex-direction: column; gap: 16px; margin-top: 16px; }
        .product-card { background: rgba(255, 255, 255, 0.5); border-radius: 12px; display: flex; flex-direction: column; overflow: hidden; position: relative; }
        @media (prefers-color-scheme: dark) { .product-card { background: rgba(50, 50, 55, 0.5); } }
        .product-image { width: 100%; height: 240px; object-fit: cover; background: #e0e0e0; }
        .product-info { padding: 16px; padding-bottom: 76px; }
        .product-name { font-weight: 600; font-size: 18px; margin: 0; }
        .product-price { font-weight: 700; color: #0071E3; font-size: 18px; margin-top: 4px; }
        .product-desc { font-size: 14px; color: #555; margin-top: 8px; line-height: 1.4; }
        @media (prefers-color-scheme: dark) { .product-desc { color: #999; } }
        .badge { position: absolute; top: 12px; right: 12px; padding: 4px 8px; border-radius: 6px; font-size: 12px; font-weight: 700; text-transform: uppercase; letter-spacing: 0.5px; }
        .badge.sold-out { background: rgba(255,59,48,0.9); color: white; backdrop-filter: blur(4px); }
        .badge.low-stock { background: rgba(255,149,0,0.9); color: white; backdrop-filter: blur(4px); }
        .sticky-bottom-bar { position: absolute; bottom: 0; left: 0; right: 0; padding: 16px; background: rgba(255,255,255,0.85); backdrop-filter: blur(10px); border-top: 1px solid rgba(0,0,0,0.05); display: flex; align-items: center; }
        @media (prefers-color-scheme: dark) { .sticky-bottom-bar { background: rgba(30,30,35,0.85); border-top: 1px solid rgba(255,255,255,0.05); } }
        .btn { background: #0071E3; color: white; border: none; padding: 10px 16px; border-radius: 8px; font-weight: 600; font-size: 14px; cursor: pointer; transition: all 0.2s; min-height: 44px; display: flex; align-items: center; justify-content: center; width: 100%; }
        .btn:disabled { background: #999; cursor: not-allowed; }
        .btn:not(:disabled):active { transform: scale(0.96); }
        .service-block h3 { margin: 0 0 16px 0; }
        .testimonial { font-style: italic; color: #555; margin-bottom: 8px; }
        @media (prefers-color-scheme: dark) { .testimonial { color: #bbb; } }
        .author { font-weight: 600; font-size: 14px; }
    </style>
    </head>
    <body>
    <div class="glass-container">
    "#);

    for block in blocks {
        html.push_str("<div class=\"block\">\n");
        match block.block_type.as_str() {
            "HeroBlock" | "Hero" => {
                let title = block.content.get("headline").and_then(|v| v.as_str()).unwrap_or("Welcome");
                let subtitle = block.content.get("subtitle").and_then(|v| v.as_str()).unwrap_or("");
                html.push_str(&format!("<h1 class=\"hero-title font-outfit\">{}</h1>\n", escape_html(title)));
                html.push_str(&format!("<p class=\"hero-subtitle\">{}</p>\n", escape_html(subtitle)));
            }
            "ProductGridBlock" | "Catalog" => {
                html.push_str("<h2 class=\"font-outfit\">Our Products</h2>\n");
                html.push_str("<div class=\"product-grid\">\n");
                if let Some(items) = block.content.get("items").and_then(|v| v.as_array()) {
                    for item in items {
                        let name = item.get("name").and_then(|v| v.as_str()).unwrap_or("Product");
                        let price = item.get("price").and_then(|v| v.as_str()).unwrap_or("$0.00");
                        let desc = item.get("description").and_then(|v| v.as_str()).unwrap_or("");
                        let image_url = item.get("image").and_then(|v| v.as_str()).unwrap_or("https://images.unsplash.com/photo-1555041469-a586c61ea9bc?auto=format&fit=crop&w=600&q=80"); // fallback placeholder

                        let mut stock_badge = String::new();
                        let mut btn_text = "Add to Cart";
                        let mut btn_disabled = "";

                        if let Some(pid) = item.get("product_id").and_then(|v| v.as_str()) {
                            tags.push(format!("entity:product:{}", pid));

                            if let Some(&inventory_count) = inventory_map.get(pid) {
                                if inventory_count <= 0 {
                                    stock_badge = "<div class=\"badge sold-out\">Sold Out</div>".to_string();
                                    btn_text = "Sold Out";
                                    btn_disabled = "disabled";
                                } else if inventory_count <= 5 {
                                    stock_badge = "<div class=\"badge low-stock\">Low Stock</div>".to_string();
                                }
                            }
                        }
                        html.push_str(&format!(
                            "<div class=\"product-card\">\n{}<img src=\"{}\" class=\"product-image\" alt=\"{}\" loading=\"lazy\" />\n<div class=\"product-info\">\n<p class=\"product-name font-outfit\">{}</p>\n<p class=\"product-price font-outfit\">{}</p>\n<p class=\"product-desc\">{}</p>\n</div>\n<div class=\"sticky-bottom-bar\">\n<button class=\"btn\" {}>{}</button>\n</div>\n</div>\n",
                            stock_badge, escape_html(image_url), escape_html(name), escape_html(name), escape_html(price), escape_html(desc), btn_disabled, btn_text
                        ));
                    }
                }
                html.push_str("</div>\n");
            }
            "ServiceBookingBlock" | "Booking" => {
                let title = block.content.get("title").and_then(|v| v.as_str()).unwrap_or("Book a Service");
                let avail = block.content.get("availability").and_then(|v| v.as_str()).unwrap_or("Available now");
                html.push_str("<div class=\"service-block\">\n");
                html.push_str(&format!("<h3 class=\"font-outfit\">{}</h3>\n", escape_html(title)));
                html.push_str(&format!("<p>{}</p>\n", escape_html(avail)));
                html.push_str("<button class=\"btn\">Book Now</button>\n");
                html.push_str("</div>\n");
            }
            "TestimonialBlock" | "Testimonials" => {
                html.push_str("<h2 class=\"font-outfit\">What People Say</h2>\n");
                if let Some(quotes) = block.content.get("quotes").and_then(|v| v.as_array()) {
                    for quote in quotes {
                        let text = quote.get("text").and_then(|v| v.as_str()).unwrap_or("");
                        let author = quote.get("author").and_then(|v| v.as_str()).unwrap_or("");
                        html.push_str(&format!(
                            "<div><p class=\"testimonial\">\"{}\"</p><p class=\"author\">- {}</p></div>\n",
                            escape_html(text), escape_html(author)
                        ));
                    }
                }
            }
            _ => {
                if let Some(text) = block.content.get("text").and_then(|v| v.as_str()) {
                    html.push_str(&format!("<p>{}</p>\n", escape_html(text)));
                }
            }
        }
        html.push_str("</div>\n");
    }

    html.push_str(r#"
        <div class="block" style="text-align: center; font-size: 12px; color: #888;">
            ⚡ Powered by OHC
        </div>
    </div>
    </body>
    </html>
    "#);

    cache.set_with_tags(&cache_key, html.clone(), tags.clone(), std::time::Duration::from_secs(3600)).await;

    Ok((html, tags))
}
