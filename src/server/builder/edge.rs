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

pub fn escape_html(s: &str) -> String {
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


pub async fn inject_dynamic_inventory(
    mut html: String,
    tenant_id: Uuid,
    pool: &PgPool,
    cache: Arc<HybridCache<String>>,
) -> String {
    let mut offset = 0;
    while let Some(start) = html[offset..].find("<!-- INVENTORY_STATUS_") {
        let actual_start = offset + start;
        let prefix_len = "<!-- INVENTORY_STATUS_".len();
        if let Some(end) = html[actual_start + prefix_len..].find(" -->") {
            let actual_end = actual_start + prefix_len + end;
            let pid = &html[actual_start + prefix_len..actual_end];
            let pid_str = pid.to_string();

            let kv_key = format!("tenant:{}:product:{}:inventory", tenant_id, pid_str);

            let mut inventory_count: i32 = 0;
            if let Some(cached_val) = cache.get(&kv_key).await {
                if let Ok(val) = cached_val.parse::<i32>() {
                    inventory_count = val;
                }
            } else {
                let db_res: Result<Option<i32>, _> = sqlx::query_scalar(
                    "SELECT inventory_count FROM products WHERE tenant_id = $1 AND id = $2"
                )
                .bind(tenant_id.to_string())
                .bind(&pid_str)
                .fetch_optional(pool)
                .await;

                if let Ok(Some(count)) = db_res {
                    inventory_count = count;
                    cache.set(&kv_key, count.to_string(), std::time::Duration::from_secs(60)).await;
                }
            }

            let replacement = if inventory_count <= 0 {
                "<span class=\"sold-out\" style=\"color: #E30000; font-weight: 600; font-size: 14px;\">Sold Out</span>"
            } else {
                ""
            };

            html.replace_range(actual_start..(actual_end + 4), replacement);

            offset = actual_start + replacement.len();
        } else {
            break;
        }
    }
    html
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

    if let Some((mut cached_html, stale)) = cache.get_with_swr(&cache_key).await {
        cached_html = inject_dynamic_inventory(cached_html, tenant_id, &state.pool, cache.clone()).await;
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        std::hash::Hash::hash(&cached_html, &mut hasher);
        let etag = format!("\"{:x}\"", std::hash::Hasher::finish(&hasher));

        let mut response = Html(cached_html).into_response();
        let cache_tag = format!("tenant-id:{}", tenant_id);
        if let Ok(val) = cache_tag.parse::<axum::http::HeaderValue>() {
            response.headers_mut().insert("Cache-Tag", val.clone());
            response.headers_mut().insert("Surrogate-Key", val);
        }
        if let Ok(etag_val) = etag.parse::<axum::http::HeaderValue>() {
            response.headers_mut().insert(axum::http::header::ETAG, etag_val);
        }
        response.headers_mut().insert(
            CACHE_CONTROL,
            axum::http::HeaderValue::from_static("public, s-maxage=60, stale-while-revalidate=86400"),
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
            let mut hasher = std::collections::hash_map::DefaultHasher::new();
            std::hash::Hash::hash(&cached_html, &mut hasher);
            let etag = format!("\"{:x}\"", std::hash::Hasher::finish(&hasher));

            let mut response = Html(cached_html).into_response();
            let cache_tag = format!("tenant-id:{}", tenant_id);
            if let Ok(val) = cache_tag.parse::<axum::http::HeaderValue>() {
                response.headers_mut().insert("Cache-Tag", val.clone());
                response.headers_mut().insert("Surrogate-Key", val);
            }
            if let Ok(etag_val) = etag.parse::<axum::http::HeaderValue>() {
                response.headers_mut().insert(axum::http::header::ETAG, etag_val);
            }
            response.headers_mut().insert(
                CACHE_CONTROL,
                axum::http::HeaderValue::from_static("public, s-maxage=60, stale-while-revalidate=86400"),
            );
            return Ok(response);
        }
    }

    let result = regenerate_cache(state.pool.clone(), tenant_id, site_id, cache_key.clone(), cache.clone()).await;

    {
        let ongoing = get_ongoing_generation();
        ongoing.lock().await.remove(&cache_key);
    }

    let (mut html, tags) = result?;
    html = inject_dynamic_inventory(html, tenant_id, &state.pool, cache.clone()).await;

    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    std::hash::Hash::hash(&html, &mut hasher);
    let etag = format!("\"{:x}\"", std::hash::Hasher::finish(&hasher));

    let mut response = Html(html).into_response();
    if !tags.is_empty() {
        if let Ok(cache_tag) = tags.join(", ").parse::<axum::http::HeaderValue>() {
            response.headers_mut().insert("Cache-Tag", cache_tag.clone());
            response.headers_mut().insert("Surrogate-Key", cache_tag);
        }
    }
    if let Ok(etag_val) = etag.parse::<axum::http::HeaderValue>() {
        response.headers_mut().insert(axum::http::header::ETAG, etag_val);
    }
    response.headers_mut().insert(
        CACHE_CONTROL,
        axum::http::HeaderValue::from_static("public, s-maxage=60, stale-while-revalidate=86400"),
    );
    Ok(response)
}

pub async fn regenerate_product_cache(
    pool: PgPool,
    tenant_id: Uuid,
    product_id: Uuid,
    cache_key: String,
    cache: std::sync::Arc<crate::utils::cache::HybridCache<String>>,
) -> Result<(String, Vec<String>), axum::http::StatusCode> {
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
                            let safe_title = seo_title.replace("&", "&amp;").replace("<", "&lt;").replace(">", "&gt;");
                            html.replace_range(start..end, &format!("<title>{}</title>\n<meta name=\"title\" content=\"{}\">\n<meta property=\"og:title\" content=\"{}\">", safe_title, safe_title, safe_title));
                        }
                    }
                }

                if let Some(seo_desc) = row.seo_description {
                    let safe_desc = seo_desc.replace("&", "&amp;").replace("<", "&lt;").replace(">", "&gt;");
                    if let Some(start) = html.find("<meta name=\"description\"") {
                        if let Some(end) = html[start..].find(">") {
                            let end = start + end + ">".len();
                            html.replace_range(start..end, &format!("<meta name=\"description\" content=\"{}\">\n<meta property=\"og:description\" content=\"{}\">", safe_desc, safe_desc));
                        }
                    } else if let Some(head_end) = html.find("</head>") {
                        html.insert_str(head_end, &format!("<meta name=\"description\" content=\"{}\">\n<meta property=\"og:description\" content=\"{}\">\n", safe_desc, safe_desc));
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
    Err(axum::http::StatusCode::NOT_FOUND)
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

    let mut tags = vec![format!("tenant-id:{}", tenant_id)];
    let mut html = String::new();
    html.push_str("<!DOCTYPE html>\n<html lang=\"en\">\n<head>\n");
    html.push_str("<meta charset=\"UTF-8\">\n<meta name=\"viewport\" content=\"width=device-width, initial-scale=1.0\">\n");

    html.push_str(&format!("<title>{}</title>\n", escape_html(&page.title)));
    if let Some(seo_name) = page.seo_metadata.get("name").and_then(|v| v.as_str()) {
        html.push_str(&format!("<meta name=\"title\" content=\"{}\">\n", escape_html(seo_name)));
    }
    if let Some(seo_description) = page.seo_metadata.get("description").and_then(|v| v.as_str()) {
        html.push_str(&format!("<meta name=\"description\" content=\"{}\">\n", escape_html(seo_description)));
    }

    let mut seo_ld = page.seo_metadata.clone();
    if seo_ld.get("@context").is_none() {
        seo_ld["@context"] = serde_json::Value::String("https://schema.org".to_string());
    }
    html.push_str(&format!("<script type=\"application/ld+json\">\n{}\n</script>\n", serde_json::to_string(&seo_ld).unwrap_or_default()));

    html.push_str(r#"
    <link href="https://fonts.googleapis.com/css2?family=Inter:wght@400;500;600;700&family=Outfit:wght@500;600;700&display=swap" rel="stylesheet">
    <style>
        @keyframes fadeIn { from { opacity: 0; transform: translateY(10px); } to { opacity: 1; transform: translateY(0); } }
        @keyframes shimmer { 0% { background-position: -200% 0; } 100% { background-position: 200% 0; } }
        body { font-family: 'Inter', sans-serif; margin: 0; padding: 0; background: #f5f5f7; color: #1D1D1F; display: flex; flex-direction: column; align-items: center; }
        .font-outfit { font-family: 'Outfit', sans-serif; }
        .glass-container { width: 100%; max-width: 375px; min-height: 100dvh; background: rgba(255, 255, 255, 0.65); backdrop-filter: blur(30px) saturate(210%); border: 1px solid rgba(255, 255, 255, 0.4); border-radius: 16px; box-shadow: 0 10px 15px -3px rgba(0,0,0,0.1), 0 4px 6px -2px rgba(0,0,0,0.05); margin: 20px auto; overflow: hidden; display: flex; flex-direction: column; animation: fadeIn 0.4s ease-out forwards; }
        @media (prefers-color-scheme: dark) { body { background: #000; color: #F5F5F7; } .glass-container { background: rgba(22, 22, 26, 0.7); border: 1px solid rgba(255, 255, 255, 0.1); } }
        .block { padding: 24px; border-bottom: 1px solid rgba(150,150,150,0.1); animation: fadeIn 0.5s ease-out both; opacity: 0; }
        .block:nth-child(1) { animation-delay: 0.1s; }
        .block:nth-child(2) { animation-delay: 0.2s; }
        .block:nth-child(3) { animation-delay: 0.3s; }
        .block:nth-child(4) { animation-delay: 0.4s; }
        .block:nth-child(5) { animation-delay: 0.5s; }
        .block:last-child { border-bottom: none; }
        .hero-title { font-size: 28px; font-weight: 700; margin: 0 0 8px 0; }
        .hero-subtitle { font-size: 16px; color: #666; margin: 0; }
        @media (prefers-color-scheme: dark) { .hero-subtitle { color: #aaa; } }
        .product-grid { display: flex; flex-direction: column; gap: 16px; margin-top: 16px; }
        .product-card { background: rgba(255, 255, 255, 0.5); border-radius: 12px; padding: 16px; display: flex; justify-content: space-between; align-items: center; transition: all 0.3s cubic-bezier(0.25, 0.8, 0.25, 1); cursor: pointer; border: 1px solid transparent; }
        .product-card:hover { transform: translateY(-2px) scale(1.01); box-shadow: 0 10px 20px rgba(0,0,0,0.08); border-color: rgba(0,102,255,0.3); }
        .product-card:active { transform: translateY(0) scale(0.98); box-shadow: 0 4px 10px rgba(0,0,0,0.05); }
        @media (prefers-color-scheme: dark) { .product-card { background: rgba(50, 50, 55, 0.5); } .product-card:hover { border-color: rgba(0,102,255,0.5); } }
        .product-name { font-weight: 600; font-size: 16px; margin: 0; transition: color 0.2s; }
        .product-card:hover .product-name { color: #0066FF; }
        .product-price { font-weight: 700; color: #0071E3; font-size: 16px; }
        .product-desc { font-size: 14px; color: #555; margin-top: 4px; }
        @media (prefers-color-scheme: dark) { .product-desc { color: #999; } }
        .btn { background: #0071E3; color: white; border: none; padding: 10px 16px; border-radius: 8px; font-weight: 600; font-size: 14px; cursor: pointer; transition: all 0.2s cubic-bezier(0.25, 0.8, 0.25, 1); box-shadow: 0 4px 6px rgba(0, 113, 227, 0.2); position: relative; overflow: hidden; }
        .btn::after { content: ''; position: absolute; top: 0; left: -100%; width: 50%; height: 100%; background: linear-gradient(to right, transparent, rgba(255,255,255,0.3), transparent); transform: skewX(-20deg); }
        .btn:hover::after { animation: shimmer 1.5s infinite; }
        .btn:hover { transform: translateY(-1px); box-shadow: 0 6px 12px rgba(0, 113, 227, 0.3); background: #0066FF; }
        .btn:active { transform: scale(0.96); box-shadow: 0 2px 4px rgba(0, 113, 227, 0.2); }
        .service-block h3 { margin: 0 0 16px 0; }
        .testimonial { font-style: italic; color: #555; margin-bottom: 8px; transition: transform 0.3s ease; }
        .testimonial-card:hover .testimonial { transform: translateX(4px); }
        @media (prefers-color-scheme: dark) { .testimonial { color: #bbb; } }
        .author { font-weight: 600; font-size: 14px; color: #0066FF; }
        .loading-skeleton { background: linear-gradient(90deg, #f0f0f0 25%, #e0e0e0 50%, #f0f0f0 75%); background-size: 200% 100%; animation: shimmer 1.5s infinite; border-radius: 4px; height: 1em; width: 100%; margin-bottom: 8px; }
        @media (prefers-color-scheme: dark) { .loading-skeleton { background: linear-gradient(90deg, #333 25%, #444 50%, #333 75%); } }
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
                        if let Some(pid) = item.get("product_id").and_then(|v| v.as_str()) {
                            tags.push(format!("entity:product:{}", pid));
                            html.push_str(&format!(
                                "<div class=\"product-card\">
<div><p class=\"product-name font-outfit\">{}</p><p class=\"product-desc\">{}</p></div><div><div class=\"product-price font-outfit\">{}</div><div class=\"inventory-status\"><!-- INVENTORY_STATUS_{} --></div></div>
</div>
",
                                escape_html(name), escape_html(desc), escape_html(price), pid
                            ));
                        } else {
                            html.push_str(&format!(
                                "<div class=\"product-card\">
<div><p class=\"product-name font-outfit\">{}</p><p class=\"product-desc\">{}</p></div><div class=\"product-price font-outfit\">{}</div>
</div>
",
                                escape_html(name), escape_html(desc), escape_html(price)
                            ));
                        }
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
    <script>
        // Hydrate cart from localStorage to keep edge HTML generic
        document.addEventListener('DOMContentLoaded', () => {
            const cartItems = JSON.parse(localStorage.getItem('cart') || '[]');
            const cartCount = cartItems.reduce((acc, item) => acc + item.quantity, 0);
            const cartBadge = document.getElementById('cart-badge');
            if (cartBadge) {
                cartBadge.innerText = cartCount;
                cartBadge.style.display = cartCount > 0 ? 'inline-block' : 'none';
            }
        });
    </script>
    </body>
    </html>
    "#);

    cache.set_with_tags(&cache_key, html.clone(), tags.clone(), std::time::Duration::from_secs(3600)).await;

    Ok((html, tags))
}
