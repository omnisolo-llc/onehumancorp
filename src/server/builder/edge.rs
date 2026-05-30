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
    EDGE_CACHE.get_or_init(|| Arc::new(HybridCache::new(None))).clone()
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

pub async fn handle_edge_request(
    Extension(state): Extension<Arc<EdgeWorkerState>>,
    Path((tenant_id_str, site_id_str)): Path<(String, String)>,
) -> Result<Response<Body>, axum::http::StatusCode> {
    let tenant_id = Uuid::parse_str(&tenant_id_str).map_err(|_| axum::http::StatusCode::BAD_REQUEST)?;
    let site_id = Uuid::parse_str(&site_id_str).map_err(|_| axum::http::StatusCode::BAD_REQUEST)?;

    let cache_key = format!("edge_site_{}_{}", tenant_id, site_id);
    let cache = get_edge_cache();

    if let Some((cached_html, stale)) = cache.get_with_swr(&cache_key).await {
        let mut response = Html(cached_html).into_response();
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

    let (html, _tags) = result?;

    let mut response = Html(html).into_response();
    response.headers_mut().insert(
        CACHE_CONTROL,
        "public, s-maxage=60, stale-while-revalidate=86400".parse().unwrap(),
    );
    Ok(response)
}

async fn regenerate_cache(
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
        .glass-container { width: 100%; max-width: 375px; min-height: 100dvh; background: rgba(255, 255, 255, 0.65); backdrop-filter: blur(40px) saturate(210%); -webkit-backdrop-filter: blur(40px) saturate(210%); border: 1px solid rgba(255, 255, 255, 0.4); border-radius: 16px; box-shadow: 0 20px 40px -10px rgba(0,0,0,0.1), 0 10px 15px -5px rgba(0,0,0,0.05); margin: 20px auto; overflow: hidden; display: flex; flex-direction: column; }
        @media (prefers-color-scheme: dark) { body { background: #000; color: #F5F5F7; } .glass-container { background: rgba(22, 22, 26, 0.7); border: 1px solid rgba(255, 255, 255, 0.1); } }
        .block { padding: 24px; border-bottom: 1px solid rgba(150,150,150,0.1); }
        .block:last-child { border-bottom: none; }
        .hero-title { font-size: 28px; font-weight: 700; margin: 0 0 8px 0; }
        .hero-subtitle { font-size: 16px; color: #666; margin: 0; }
        @media (prefers-color-scheme: dark) { .hero-subtitle { color: #aaa; } }
        .product-grid { display: flex; flex-direction: column; gap: 16px; margin-top: 16px; }
        .product-card { background: rgba(255, 255, 255, 0.5); backdrop-filter: blur(10px); border: 1px solid rgba(255, 255, 255, 0.3); border-radius: 16px; padding: 16px; display: flex; justify-content: space-between; align-items: center; box-shadow: 0 4px 6px -1px rgba(0, 0, 0, 0.05); }
        @media (prefers-color-scheme: dark) { .product-card { background: rgba(50, 50, 55, 0.5); border: 1px solid rgba(255, 255, 255, 0.05); } }
        .product-name { font-weight: 600; font-size: 16px; margin: 0; }
        .product-price { font-weight: 700; color: #0071E3; font-size: 16px; }
        .product-desc { font-size: 14px; color: #555; margin-top: 4px; }
        @media (prefers-color-scheme: dark) { .product-desc { color: #999; } }
        .btn { background: linear-gradient(180deg, #007aff 0%, #0056b3 100%); color: white; border: none; padding: 12px 20px; border-radius: 12px; font-weight: 600; font-size: 16px; cursor: pointer; transition: all 0.2s; width: 100%; box-shadow: 0 2px 4px rgba(0, 122, 255, 0.3); }
        .btn:active { transform: scale(0.96); box-shadow: 0 1px 2px rgba(0, 122, 255, 0.3); }
        .service-block { background: rgba(255, 255, 255, 0.4); backdrop-filter: blur(10px); border: 1px solid rgba(255, 255, 255, 0.3); border-radius: 16px; padding: 20px; text-align: center; }
        @media (prefers-color-scheme: dark) { .service-block { background: rgba(50, 50, 55, 0.4); border: 1px solid rgba(255, 255, 255, 0.05); } }
        .service-block h3 { margin: 0 0 16px 0; }
        .testimonial-card { background: rgba(255, 255, 255, 0.4); backdrop-filter: blur(10px); border: 1px solid rgba(255, 255, 255, 0.3); border-radius: 16px; padding: 16px; margin-bottom: 12px; }
        @media (prefers-color-scheme: dark) { .testimonial-card { background: rgba(50, 50, 55, 0.4); border: 1px solid rgba(255, 255, 255, 0.05); } }
        .testimonial { font-style: italic; color: #555; margin-bottom: 8px; margin-top: 0; }
        @media (prefers-color-scheme: dark) { .testimonial { color: #bbb; } }
        .author { font-weight: 600; font-size: 14px; margin: 0; }
        .form-input { width: 100%; padding: 12px; margin-bottom: 12px; border: 1px solid rgba(0,0,0,0.1); border-radius: 8px; background: rgba(255,255,255,0.7); box-sizing: border-box; font-family: 'Inter', sans-serif; }
        @media (prefers-color-scheme: dark) { .form-input { background: rgba(0,0,0,0.3); border: 1px solid rgba(255,255,255,0.1); color: #fff; } }
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
                html.push_str(&format!("<h1 class=\"hero-title font-outfit\">{}</h1>
", escape_html(title)));
                html.push_str(&format!("<p class=\"hero-subtitle\">{}</p>
", escape_html(subtitle)));
            }
            "ProductGridBlock" | "Catalog" => {
                html.push_str("<h2 class=\"font-outfit\">Our Products</h2>
");
                html.push_str("<div class=\"product-grid\">
");
                if let Some(items) = block.content.get("items").and_then(|v| v.as_array()) {
                    for item in items {
                        let name = item.get("name").and_then(|v| v.as_str()).unwrap_or("Product");
                        let price = item.get("price").and_then(|v| v.as_str()).unwrap_or("$0.00");
                        let desc = item.get("description").and_then(|v| v.as_str()).unwrap_or("");
                        if let Some(pid) = item.get("product_id").and_then(|v| v.as_str()) {
                            tags.push(format!("entity:product:{}", pid));
                        }
                        html.push_str(&format!(
                            "<div class=\"product-card\">
<div><p class=\"product-name font-outfit\">{}</p><p class=\"product-desc\">{}</p></div><div class=\"product-price font-outfit\">{}</div>
</div>
",
                            escape_html(name), escape_html(desc), escape_html(price)
                        ));
                    }
                }
                html.push_str("</div>
");
            }
            "ServiceBookingBlock" | "Booking" => {
                let title = block.content.get("title").and_then(|v| v.as_str()).unwrap_or("Book a Service");
                let avail = block.content.get("availability").and_then(|v| v.as_str()).unwrap_or("Available now");
                html.push_str("<div class=\"service-block\">
");
                html.push_str(&format!("<h3 class=\"font-outfit\">{}</h3>
", escape_html(title)));
                html.push_str(&format!("<p>{}</p>
", escape_html(avail)));
                html.push_str("<button class=\"btn\">Book Now</button>
");
                html.push_str("</div>
");
            }
            "BookingCalendarBlock" | "BookingCalendar" => {
                let title = block.content.get("title").and_then(|v| v.as_str()).unwrap_or("Schedule an Appointment");
                html.push_str("<div class=\"service-block\">
");
                html.push_str(&format!("<h3 class=\"font-outfit\">{}</h3>
", escape_html(title)));
                html.push_str("<div style=\"background: rgba(0,0,0,0.05); border-radius: 8px; padding: 16px; margin-bottom: 16px;\"><p style=\"color: #666; font-size: 14px;\">Calendar Widget</p></div>
");
                html.push_str("<button class=\"btn\">Confirm Booking</button>
");
                html.push_str("</div>
");
            }
            "TestimonialBlock" | "Testimonials" => {
                html.push_str("<h2 class=\"font-outfit\">What People Say</h2>
");
                if let Some(quotes) = block.content.get("quotes").and_then(|v| v.as_array()) {
                    for quote in quotes {
                        let text = quote.get("text").and_then(|v| v.as_str()).unwrap_or("");
                        let author = quote.get("author").and_then(|v| v.as_str()).unwrap_or("");
                        html.push_str(&format!(
                            "<div class=\"testimonial-card\"><p class=\"testimonial\">\"{}\"</p><p class=\"author\">- {}</p></div>
",
                            escape_html(text), escape_html(author)
                        ));
                    }
                }
            }
            "ContactFormBlock" | "ContactForm" => {
                html.push_str("<div class=\"service-block\">
");
                html.push_str("<h2 class=\"font-outfit\" style=\"margin-bottom: 16px;\">Contact Us</h2>
");
                html.push_str("<input class=\"form-input\" placeholder=\"Your Name\" />
");
                html.push_str("<input class=\"form-input\" placeholder=\"Your Email\" />
");
                html.push_str("<textarea class=\"form-input\" placeholder=\"Message\" rows=\"3\"></textarea>
");
                html.push_str("<button class=\"btn\" style=\"background: #111; color: #fff;\">Send Message</button>
");
                html.push_str("</div>
");
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
