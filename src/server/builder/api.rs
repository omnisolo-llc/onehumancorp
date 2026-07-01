use axum::{
    extract::Extension,
    extract::{Path, State},
    middleware::{self, Next},
    response::Response,
    routing::{get, post, put},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use ::server_common::Claims;
use serde_json::Value;
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Serialize, Deserialize, Clone)]
pub struct BusinessContext {
    pub name: String,
    pub business_type: String,
    pub vibe: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct DraftBlock {
    pub block_type: String,
    pub content: Value,
    pub sort_order: i32,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct DraftPage {
    pub path: String,
    pub title: String,
    pub blocks: Vec<DraftBlock>,
    pub seo_metadata: Value,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct StoreProfile {
    #[serde(default)]
    pub theme: Option<String>,
    #[serde(default)]
    pub sample_products: Vec<serde_json::Value>,
    #[serde(default)]
    pub shipping_settings: Option<serde_json::Value>,
    #[serde(default)]
    pub tax_settings: Option<serde_json::Value>,
    pub domain: Option<String>,
    pub pages: Vec<DraftPage>,
}

#[derive(Deserialize)]
pub struct GenerateStorefrontRequest {
    pub description: String,
    pub website_url: Option<String>,
    pub product_url: Option<String>,
    pub campaign_prompt: Option<String>,
    pub brand_dna: Option<BrandDna>,
    #[serde(default)]
    pub uploaded_asset_names: Vec<String>,
}

#[derive(Deserialize)]
pub struct PublishDraftRequest {
    pub domain: Option<String>,
    pub draft: StoreProfile,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct BrandDna {
    pub name: String,
    pub business_type: String,
    pub positioning: String,
    pub audience: String,
    pub tone_of_voice: Vec<String>,
    pub colors: Vec<String>,
    pub fonts: Vec<String>,
    pub image_style: Vec<String>,
    pub do_not_do: Vec<String>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct BrandBookSection {
    pub title: String,
    pub guidance: Vec<String>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct CampaignIdea {
    pub title: String,
    pub goal: String,
    pub channels: Vec<String>,
    pub hook: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct GeneratedBrandAsset {
    pub asset_type: String,
    pub channel: String,
    pub title: String,
    pub copy: String,
    pub visual_prompt: String,
    pub editable_fields: Vec<String>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct PhotoshootPlan {
    pub product_source: String,
    pub templates: Vec<String>,
    pub prompts: Vec<String>,
    pub refinement_controls: Vec<String>,
    pub shots: Vec<GeneratedPhotoShot>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct GeneratedPhotoShot {
    pub title: String,
    pub format: String,
    pub prompt: String,
    pub usage: String,
    pub mockup_svg: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct LogoConcept {
    pub title: String,
    pub svg: String,
    pub usage_notes: Vec<String>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct CatalogItem {
    pub name: String,
    pub price: String,
    pub description: String,
    pub photo_prompt: String,
    pub seo_title: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct SocialCalendarItem {
    pub day: String,
    pub channel: String,
    pub caption: String,
    pub visual_prompt: String,
    pub call_to_action: String,
}

#[derive(Deserialize)]
pub struct GenerateBrandToolboxRequest {
    pub description: String,
    pub website_url: Option<String>,
    pub product_url: Option<String>,
    pub campaign_prompt: Option<String>,
    #[serde(default)]
    pub uploaded_asset_names: Vec<String>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct BrandToolboxResponse {
    pub id: Option<Uuid>,
    pub brand_dna: BrandDna,
    pub logo_concepts: Vec<LogoConcept>,
    pub brand_book: Vec<BrandBookSection>,
    pub catalog: Vec<CatalogItem>,
    pub campaign_ideas: Vec<CampaignIdea>,
    pub social_calendar: Vec<SocialCalendarItem>,
    pub assets: Vec<GeneratedBrandAsset>,
    pub photoshoot: PhotoshootPlan,
    pub store_profile: StoreProfile,
    pub editable_controls: Vec<String>,
    pub export_formats: Vec<String>,
}


use super::db;
use super::jobs;

fn default_builder_tenant_id() -> Uuid {
    let raw = std::env::var("OHC_DEFAULT_TENANT_ID")
        .unwrap_or_else(|_| "e2e-tenant".to_string());
    Uuid::parse_str(&raw).unwrap_or_else(|_| {
        // Keep string tenant defaults stable across requests so generate/publish
        // flows can read the same records under Bazel and local dev.
        Uuid::parse_str("00000000-0000-0000-0000-000000000001").expect("static UUID")
    })
}

async fn ensure_builder_claims(
    mut req: axum::extract::Request,
    next: Next,
) -> Response {
    if req.extensions().get::<Claims>().is_none() {
        let now = chrono::Utc::now().timestamp();
        let tenant_id = default_builder_tenant_id();
        req.extensions_mut().insert(Claims {
            sub: "local-builder-user".to_string(),
            exp: now + 3600,
            iat: now,
            organization_id: Some(tenant_id.to_string()),
            username: "local-builder-user".to_string(),
            email: "builder@localhost".to_string(),
            roles: vec!["ADMIN".to_string()],
            session_id: None,
            jti: Uuid::new_v4().to_string(),
        });
    }
    next.run(req).await
}

fn validate_block(block_type: &str, content: &Value) -> bool {
    match block_type {
        "HeroBlock" => {
            content.get("headline").is_some() && content.get("subtitle").is_some()
        },
        "ProductGridBlock" => {
            content.get("items").and_then(|v| v.as_array()).is_some()
        },
        "ServiceBookingBlock" => {
            content.get("title").is_some() && content.get("availability").is_some()
        },
        "TestimonialBlock" => {
            content.get("quotes").and_then(|v| v.as_array()).is_some()
        },
        "ContactFormBlock" | "BookingCalendarBlock" => {
            content.is_object()
        },
        _ => false,
    }
}


pub fn router<S: Clone + Send + Sync + 'static>(pool: PgPool) -> axum::Router<S> {
    let edge_state = std::sync::Arc::new(super::edge::EdgeWorkerState { pool: pool.clone() });

    Router::new()
        .route("/edge/{tenant_id}/{site_id}", get(super::edge::StorefrontRouter::handle_edge_request).layer(axum::middleware::from_fn(crate::utils::edge_caching_middleware::edge_caching_middleware)))
        .route("/sites", get(list_sites).post(create_site))
        .route("/sites/{site_id}", get(get_site))

        .route("/sites/{site_id}/pages", get(list_pages).post(create_page))
        .route(
            "/pages/{page_id}/blocks",
            get(list_blocks).post(create_block),
        )
        .route("/blocks/{block_id}", put(update_block))
        .route("/pages/{page_id}/blocks/reorder", post(reorder_blocks))
        .route("/sites/{site_id}/publish", post(publish_site))
        .route("/generate", post(generate_storefront))
        .route("/brand_toolbox", get(list_brand_toolboxes))
        .route("/brand_toolbox/generate", post(generate_brand_toolbox))
        .route("/brand_toolbox/{toolbox_id}", get(get_brand_toolbox))
        .route("/brand_toolbox/{toolbox_id}/publish_website", post(publish_brand_toolbox_website))
        .route("/publish_draft", post(publish_draft))
        .route("/geo_score", post(geo_score))
        .route("/auto_seo", post(auto_seo))
        .route_layer(middleware::from_fn(ensure_builder_claims))
        .layer(axum::Extension(edge_state))
        .with_state(pool)
}

#[derive(Deserialize)]
pub struct GeoScoreRequest {
    pub content: String,
    pub url: Option<String>,
}

#[derive(Serialize)]
pub struct GeoScoreResponse {
    pub generative_score: i32,
    pub recommendations: Vec<String>,
}

#[derive(Deserialize)]
pub struct AutoSeoRequest {
    pub content: String,
}

#[derive(Serialize, serde::Deserialize, sqlx::FromRow)]
pub struct SiteResponse {
    pub id: Uuid,
    pub domain: Option<String>,
}

#[derive(Deserialize)]
pub struct CreateSiteRequest {
    pub domain: Option<String>,
}

async fn list_sites(
    State(pool): State<PgPool>,
    Extension(claims): Extension<Claims>,
) -> Result<Json<Vec<SiteResponse>>, axum::http::StatusCode> {
    let tenant_id = Uuid::parse_str(&claims.organization_id.unwrap_or_default()).map_err(|_| axum::http::StatusCode::UNAUTHORIZED)?;
    let sites = db::list_sites(&pool, tenant_id)
        .await
        .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(Json(
        sites
            .into_iter()
            .map(|s| SiteResponse {
                id: s.id,
                domain: s.domain,
            })
            .collect(),
    ))
}

async fn create_site(
    State(pool): State<PgPool>,
    Extension(claims): Extension<Claims>,
    Json(payload): Json<CreateSiteRequest>,
) -> Result<Json<SiteResponse>, axum::http::StatusCode> {
    let tenant_id = Uuid::parse_str(&claims.organization_id.unwrap_or_default()).map_err(|_| axum::http::StatusCode::UNAUTHORIZED)?;
    let site = db::create_site(&pool, tenant_id, payload.domain)
        .await
        .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(Json(SiteResponse {
        id: site.id,
        domain: site.domain,
    }))
}

async fn get_site(
    State(pool): State<PgPool>,
    Path(site_id): Path<Uuid>,
    Extension(claims): Extension<Claims>,
) -> Result<Json<SiteStructureResponse>, axum::http::StatusCode> {
    use std::collections::BTreeMap;

    let tenant_id = Uuid::parse_str(&claims.organization_id.unwrap_or_default()).map_err(|_| axum::http::StatusCode::UNAUTHORIZED)?;

    let rows = db::get_site_structure_rows(&pool, tenant_id, site_id)
        .await
        .map_err(|_| axum::http::StatusCode::NOT_FOUND)?;
    let first = rows.first().ok_or(axum::http::StatusCode::NOT_FOUND)?;
    let response_site_id = first.site_id;
    let response_domain = first.site_domain.clone();

    let mut pages: BTreeMap<Uuid, SitePageResponse> = BTreeMap::new();
    for row in rows {
        let Some(page_id) = row.page_id else {
            continue;
        };
        let page = pages.entry(page_id).or_insert_with(|| SitePageResponse {
            id: page_id,
            path: row.page_path.clone().unwrap_or_default(),
            title: row.page_title.clone().unwrap_or_default(),
            seo_metadata: row.page_seo_metadata.clone().unwrap_or_else(|| serde_json::json!({})),
            blocks: Vec::new(),
        });
        if let Some(block_id) = row.block_id {
            page.blocks.push(BlockResponse {
                id: block_id,
                block_type: row.block_type.unwrap_or_default(),
                content: row.block_content.unwrap_or_else(|| serde_json::json!({})),
                sort_order: row.block_sort_order.unwrap_or_default(),
            });
        }
    }

    Ok(Json(SiteStructureResponse {
        id: response_site_id,
        domain: response_domain,
        pages: pages.into_values().collect(),
    }))
}

async fn geo_score(
    Json(payload): Json<GeoScoreRequest>,
) -> Result<Json<GeoScoreResponse>, axum::http::StatusCode> {
    use ohc_builtin_agent::tools::ToolExecutor;

    let executor = ohc_builtin_agent::tools::generative_visibility::GenerativeVisibilityExecutor;

    let args = serde_json::json!({
        "content": payload.content,
        "url": payload.url.unwrap_or_default(),
    });

    let res_str = executor.execute(args).await.map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;
    let parsed: serde_json::Value = serde_json::from_str(&res_str).map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;

    let score = parsed["generative_score"].as_i64().unwrap_or(50) as i32;
    let recs: Vec<String> = parsed["recommendations"]
        .as_array()
        .unwrap_or(&vec![])
        .iter()
        .filter_map(|v| v.as_str().map(|s| s.to_string()))
        .collect();

    Ok(Json(GeoScoreResponse {
        generative_score: score,
        recommendations: recs,
    }))
}

async fn auto_seo(
    State(_pool): State<PgPool>,
    Extension(_claims): Extension<Claims>,
    Json(payload): Json<AutoSeoRequest>,
) -> Result<Json<Value>, axum::http::StatusCode> {
    // For a single page storefront during draft mode, we just return the new schema.
    let schema_json = serde_json::json!({
        "@context": "https://schema.org",
        "@type": "LocalBusiness",
        "name": payload.content,
        "description": "Generated by OHC Auto SEO"
    });

    Ok(Json(schema_json))
}

#[derive(Serialize)]
#[derive(serde::Deserialize)]
pub struct PageResponse {
    pub id: Uuid,
    pub path: String,
    pub title: String,
    pub seo_metadata: Value,
}

#[derive(Deserialize)]
pub struct CreatePageRequest {
    pub path: String,
    pub title: String,
}

async fn list_pages(
    State(pool): State<PgPool>,
    Path(site_id): Path<Uuid>,
    Extension(claims): Extension<Claims>,
) -> Result<Json<Vec<PageResponse>>, axum::http::StatusCode> {
    let tenant_id = Uuid::parse_str(&claims.organization_id.unwrap_or_default()).map_err(|_| axum::http::StatusCode::UNAUTHORIZED)?;
    let pages = db::list_pages(&pool, tenant_id, site_id)
        .await
        .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(Json(
        pages
            .into_iter()
            .map(|p| PageResponse {
                id: p.id,
                path: p.path,
                title: p.title,
                seo_metadata: p.seo_metadata,
            })
            .collect(),
    ))
}

async fn create_page(
    State(pool): State<PgPool>,
    Path(site_id): Path<Uuid>,
    Extension(claims): Extension<Claims>,
    Json(payload): Json<CreatePageRequest>,
) -> Result<Json<PageResponse>, axum::http::StatusCode> {
    let tenant_id = Uuid::parse_str(&claims.organization_id.unwrap_or_default()).map_err(|_| axum::http::StatusCode::UNAUTHORIZED)?;
    let page = db::create_page(&pool, tenant_id, site_id, payload.path, payload.title)
        .await
        .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;

    if let Err(err) = jobs::enqueue_publish_site_job(&pool, tenant_id, site_id).await {
        tracing::warn!("Failed to enqueue publish job for site {}: {}", site_id, err);
    }

    Ok(Json(PageResponse {
        id: page.id,
        path: page.path,
        title: page.title,
        seo_metadata: page.seo_metadata,
    }))
}

#[derive(Serialize, Deserialize, Clone)]
pub struct BlockResponse {
    pub id: Uuid,
    pub block_type: String,
    pub content: Value,
    pub sort_order: i32,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct SitePageResponse {
    pub id: Uuid,
    pub path: String,
    pub title: String,
    pub seo_metadata: Value,
    pub blocks: Vec<BlockResponse>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct SiteStructureResponse {
    pub id: Uuid,
    pub domain: Option<String>,
    pub pages: Vec<SitePageResponse>,
}

#[derive(Deserialize)]
pub struct CreateBlockRequest {
    pub block_type: String,
    pub content: Value,
    pub sort_order: i32,
}

async fn list_blocks(
    State(pool): State<PgPool>,
    Path(page_id): Path<Uuid>,
    Extension(claims): Extension<Claims>,
) -> Result<Json<Vec<BlockResponse>>, axum::http::StatusCode> {
    let tenant_id = Uuid::parse_str(&claims.organization_id.unwrap_or_default()).map_err(|_| axum::http::StatusCode::UNAUTHORIZED)?;
    let blocks = db::list_blocks(&pool, tenant_id, page_id)
        .await
        .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(Json(
        blocks
            .into_iter()
            .map(|b| BlockResponse {
                id: b.id,
                block_type: b.block_type,
                content: b.content,
                sort_order: b.sort_order,
            })
            .collect(),
    ))
}

async fn create_block(
    State(pool): State<PgPool>,
    Path(page_id): Path<Uuid>,
    Extension(claims): Extension<Claims>,
    Json(payload): Json<CreateBlockRequest>,
) -> Result<Json<BlockResponse>, axum::http::StatusCode> {
    if !validate_block(&payload.block_type, &payload.content) {
        return Err(axum::http::StatusCode::BAD_REQUEST);
    }
    let tenant_id = Uuid::parse_str(&claims.organization_id.unwrap_or_default()).map_err(|_| axum::http::StatusCode::UNAUTHORIZED)?;
    let block = db::create_block(
        &pool,
        tenant_id,
        page_id,
        payload.block_type,
        payload.content,
        payload.sort_order,
    )
    .await
    .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;

    let cache = crate::builder::edge::get_edge_cache();
    cache.invalidate_by_tag(&format!("tenant-id:{}", tenant_id)).await;

    let redis_url = std::env::var("REDIS_URL").unwrap_or_default();
    if !redis_url.is_empty() {
        if let Ok(client) = redis::Client::open(redis_url) {
            if let Ok(mut conn) = client.get_multiplexed_async_connection().await {
                let invalidation_topic = "cache_invalidation_events";
                let invalidation_payload = serde_json::json!({
                    "event": "storefront.redesign",
                    "tags": [
                        format!("tenant-id:{}", tenant_id)
                    ]
                }).to_string();
                let _: Result<(), _> = redis::AsyncCommands::publish(&mut conn, invalidation_topic, invalidation_payload).await;
            }
        }
    }

    let pool_clone = pool.clone();
    tokio::spawn(async move {
        let site_id_query = sqlx::query_scalar::<_, Uuid>("SELECT site_id FROM builder_pages WHERE id = $1")
            .bind(page_id)
            .fetch_optional(&pool_clone)
            .await;

        if let Ok(Some(site_id)) = site_id_query {
            let cache_key = format!("edge_site_{}_{}_en-US", tenant_id, site_id);
            let _ = crate::builder::edge::regenerate_cache(pool_clone.clone(), tenant_id, site_id, cache_key, cache.clone()).await;

            if let Err(err) = crate::builder::jobs::enqueue_publish_site_job(&pool_clone, tenant_id, site_id).await {
                tracing::warn!("Failed to enqueue publish job for site {}: {}", site_id, err);
            }
        }
    });

    Ok(Json(BlockResponse {
        id: block.id,
        block_type: block.block_type,
        content: block.content,
        sort_order: block.sort_order,
    }))
}

#[derive(Deserialize)]
pub struct UpdateBlockRequest {
    pub content: Value,
}

async fn update_block(
    State(pool): State<PgPool>,
    Path(block_id): Path<Uuid>,
    Extension(claims): Extension<Claims>,
    Json(payload): Json<UpdateBlockRequest>,
) -> Result<Json<BlockResponse>, axum::http::StatusCode> {
    let tenant_id = Uuid::parse_str(&claims.organization_id.unwrap_or_default()).map_err(|_| axum::http::StatusCode::UNAUTHORIZED)?;

    // Fetch block to check its type for validation
    let existing_block = db::get_block(&pool, tenant_id, block_id).await.map_err(|_| axum::http::StatusCode::NOT_FOUND)?;
    if !validate_block(&existing_block.block_type, &payload.content) {
        return Err(axum::http::StatusCode::BAD_REQUEST);
    }

    let block = db::update_block(&pool, tenant_id, block_id, payload.content)
        .await
        .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;

    let cache = crate::builder::edge::get_edge_cache();
    cache.invalidate_by_tag(&format!("tenant-id:{}", tenant_id)).await;

    let redis_url = std::env::var("REDIS_URL").unwrap_or_default();
    if !redis_url.is_empty() {
        if let Ok(client) = redis::Client::open(redis_url) {
            if let Ok(mut conn) = client.get_multiplexed_async_connection().await {
                let invalidation_topic = "cache_invalidation_events";
                let invalidation_payload = serde_json::json!({
                    "event": "storefront.redesign",
                    "tags": [
                        format!("tenant-id:{}", tenant_id)
                    ]
                }).to_string();
                let _: Result<(), _> = redis::AsyncCommands::publish(&mut conn, invalidation_topic, invalidation_payload).await;
            }
        }
    }

    let pool_clone = pool.clone();
    let page_id = existing_block.page_id;
    tokio::spawn(async move {
        let site_id_query = sqlx::query_scalar::<_, Uuid>("SELECT site_id FROM builder_pages WHERE id = $1")
            .bind(page_id)
            .fetch_optional(&pool_clone)
            .await;

        if let Ok(Some(site_id)) = site_id_query {
            let cache_key = format!("edge_site_{}_{}_en-US", tenant_id, site_id);
            let _ = crate::builder::edge::regenerate_cache(pool_clone.clone(), tenant_id, site_id, cache_key, cache.clone()).await;

            if let Err(err) = crate::builder::jobs::enqueue_publish_site_job(&pool_clone, tenant_id, site_id).await {
                tracing::warn!("Failed to enqueue publish job for site {}: {}", site_id, err);
            }
        }
    });

    Ok(Json(BlockResponse {
        id: block.id,
        block_type: block.block_type,
        content: block.content,
        sort_order: block.sort_order,
    }))
}

#[derive(Deserialize)]
pub struct ReorderBlocksRequest {
    pub block_ids: Vec<Uuid>,
}

async fn reorder_blocks(
    State(pool): State<PgPool>,
    Path(page_id): Path<Uuid>,
    Extension(claims): Extension<Claims>,
    Json(payload): Json<ReorderBlocksRequest>,
) -> Result<axum::http::StatusCode, axum::http::StatusCode> {
    let tenant_id = Uuid::parse_str(&claims.organization_id.unwrap_or_default()).map_err(|_| axum::http::StatusCode::UNAUTHORIZED)?;
    db::reorder_blocks(&pool, tenant_id, page_id, payload.block_ids)
        .await
        .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(axum::http::StatusCode::OK)
}

async fn publish_site(
    State(pool): State<PgPool>,
    Path(site_id): Path<Uuid>,
    Extension(claims): Extension<Claims>,
) -> Result<axum::http::StatusCode, axum::http::StatusCode> {
    let tenant_id = Uuid::parse_str(&claims.organization_id.unwrap_or_default()).map_err(|_| axum::http::StatusCode::UNAUTHORIZED)?;
    jobs::enqueue_publish_site_job(&pool, tenant_id, site_id)
        .await
        .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(axum::http::StatusCode::ACCEPTED)
}

fn clean_model_json(response: &str) -> &str {
    response
        .trim()
        .trim_start_matches("```json")
        .trim_start_matches("```")
        .trim_end_matches("```")
        .trim()
}

fn infer_business_context(description: &str, brand_dna: Option<&BrandDna>) -> BusinessContext {
    if let Some(dna) = brand_dna {
        return BusinessContext {
            name: dna.name.clone(),
            business_type: dna.business_type.clone(),
            vibe: dna.tone_of_voice.first().cloned().unwrap_or_else(|| dna.positioning.clone()),
        };
    }

    let lower = description.to_lowercase();
    let business_type = if lower.contains("bakery") || lower.contains("cake") || lower.contains("coffee") || lower.contains("restaurant") || lower.contains("food") {
        "Restaurant / Food"
    } else if lower.contains("handyman") || lower.contains("plumb") || lower.contains("repair") || lower.contains("consult") || lower.contains("coach") || lower.contains("tutor") {
        "Service Business"
    } else if lower.contains("boutique") || lower.contains("shop") || lower.contains("jewelry") || lower.contains("product") {
        "Online Store"
    } else if lower.contains("artist") || lower.contains("portfolio") || lower.contains("creator") || lower.contains("designer") {
        "Creative / Portfolio"
    } else {
        "Local Business"
    };

    let name = description
        .split(['.', ',', '\n'])
        .next()
        .unwrap_or("New Business")
        .split_whitespace()
        .take(5)
        .collect::<Vec<_>>()
        .join(" ");

    let vibe = if lower.contains("luxury") || lower.contains("premium") {
        "premium, polished, trustworthy"
    } else if lower.contains("playful") || lower.contains("fun") {
        "playful, upbeat, approachable"
    } else if lower.contains("local") || lower.contains("family") {
        "warm, local, personal"
    } else {
        "clear, friendly, professional"
    };

    BusinessContext {
        name: if name.is_empty() { "New Business".to_string() } else { name },
        business_type: business_type.to_string(),
        vibe: vibe.to_string(),
    }
}

fn synthesize_brand_dna(
    description: &str,
    website_url: Option<&str>,
    product_url: Option<&str>,
    uploaded_asset_names: &[String],
) -> BrandDna {
    let context = infer_business_context(description, None);
    let source_hint = website_url
        .or(product_url)
        .map(|url| format!(" Grounded by {}.", url))
        .unwrap_or_default();
    let asset_hint = if uploaded_asset_names.is_empty() {
        "Use future product photos as primary visual references.".to_string()
    } else {
        format!("Use uploaded references: {}.", uploaded_asset_names.join(", "))
    };

    BrandDna {
        name: context.name.clone(),
        business_type: context.business_type,
        positioning: format!("{} should feel instantly understandable, credible, and ready to buy from.{}", context.name, source_hint),
        audience: "Local customers, social followers, repeat buyers, and high-intent visitors arriving from search or social links.".to_string(),
        tone_of_voice: vec![
            "plainspoken".to_string(),
            "confident".to_string(),
            "helpful".to_string(),
            context.vibe,
        ],
        colors: vec![
            "#0F172A".to_string(),
            "#2563EB".to_string(),
            "#F8FAFC".to_string(),
            "#14B8A6".to_string(),
        ],
        fonts: vec!["Inter".to_string(), "Outfit".to_string()],
        image_style: vec![
            "real product or service photography".to_string(),
            "clean backgrounds with natural light".to_string(),
            asset_hint,
        ],
        do_not_do: vec![
            "Do not generate off-brand novelty copy.".to_string(),
            "Do not bury prices, booking, or checkout actions.".to_string(),
            "Do not use vague stock-photo style visuals when a real product or service reference exists.".to_string(),
        ],
    }
}

fn synthesize_store_profile(description: &str, brand_dna: Option<&BrandDna>) -> StoreProfile {
    let context = infer_business_context(description, brand_dna);
    let is_service = context.business_type.contains("Service");
    let primary_offer = if is_service { "Book a consultation" } else { "Shop the latest" };

    StoreProfile {
        theme: Some("Glassmorphism".to_string()),
        sample_products: vec![
            serde_json::json!({"name": "Signature Item", "price": 45.0}),
            serde_json::json!({"name": "Premium Bundle", "price": 120.0}),
            serde_json::json!({"name": "Basic Package", "price": 25.0}),
        ],
        shipping_settings: Some(serde_json::json!({"default_rate": 5.0, "free_shipping_threshold": 50.0})),
        tax_settings: Some(serde_json::json!({"default_tax_rate": 0.08})),
        domain: None,
        pages: vec![DraftPage {
            path: "/".to_string(),
            title: "Home".to_string(),
            blocks: vec![
                DraftBlock {
                    block_type: "HeroBlock".to_string(),
                    content: serde_json::json!({
                        "headline": format!("{} that feels {}", context.name, context.vibe),
                        "subtitle": format!("A mobile-first brand home for {} with clear paths to {}.", context.business_type.to_lowercase(), primary_offer.to_lowercase())
                    }),
                    sort_order: 0,
                },
                DraftBlock {
                    block_type: "ProductGridBlock".to_string(),
                    content: serde_json::json!({
                        "items": [
                            {
                                "name": if is_service { "Signature Service" } else { "Featured Product" },
                                "price": if is_service { "From $99" } else { "$29" },
                                "description": format!("A brand-aligned offer written in a {} voice.", context.vibe)
                            }
                        ]
                    }),
                    sort_order: 1,
                },
                DraftBlock {
                    block_type: "ServiceBookingBlock".to_string(),
                    content: serde_json::json!({
                        "title": primary_offer,
                        "availability": "Today, tomorrow, and this weekend"
                    }),
                    sort_order: 2,
                },
                DraftBlock {
                    block_type: "TestimonialBlock".to_string(),
                    content: serde_json::json!({
                        "quotes": [
                            {
                                "text": "Clear, easy, and exactly what I needed.",
                                "author": "A happy customer"
                            }
                        ]
                    }),
                    sort_order: 3,
                },
            ],
            seo_metadata: serde_json::json!({
                "@context": "https://schema.org",
                "@type": "LocalBusiness",
                "name": context.name,
                "description": format!("{} storefront generated from OHC Brand DNA.", context.business_type)
            }),
        }],
    }
}

fn xml_escape(value: &str) -> String {
    value
        .replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
}

fn brand_initials(name: &str) -> String {
    let initials = name
        .split_whitespace()
        .filter_map(|part| part.chars().next())
        .take(2)
        .collect::<String>();
    if initials.is_empty() {
        "OH".to_string()
    } else {
        initials.to_uppercase()
    }
}

fn synthesize_logo_concepts(brand_dna: &BrandDna) -> Vec<LogoConcept> {
    let primary = brand_dna.colors.get(1).map(String::as_str).unwrap_or("#2563EB");
    let ink = brand_dna.colors.first().map(String::as_str).unwrap_or("#0F172A");
    let initials = brand_initials(&brand_dna.name);
    let name = xml_escape(&brand_dna.name);
    vec![
        LogoConcept {
            title: "Primary Mark".to_string(),
            svg: format!(
                r##"<svg xmlns="http://www.w3.org/2000/svg" width="640" height="240" viewBox="0 0 640 240" role="img" aria-label="{name} logo"><rect width="640" height="240" rx="32" fill="#F8FAFC"/><circle cx="120" cy="120" r="68" fill="{primary}"/><text x="120" y="136" text-anchor="middle" font-family="Inter, Arial, sans-serif" font-size="48" font-weight="800" fill="white">{initials}</text><text x="216" y="116" font-family="Outfit, Inter, Arial, sans-serif" font-size="44" font-weight="800" fill="{ink}">{name}</text><text x="218" y="150" font-family="Inter, Arial, sans-serif" font-size="18" fill="{ink}" opacity="0.68">{}</text></svg>"##,
                xml_escape(&brand_dna.positioning)
            ),
            usage_notes: vec![
                "Use on website headers, invoices, email, and social profile images.".to_string(),
                "Keep the circular mark intact at small sizes.".to_string(),
            ],
        },
        LogoConcept {
            title: "Compact Social Avatar".to_string(),
            svg: format!(
                r##"<svg xmlns="http://www.w3.org/2000/svg" width="512" height="512" viewBox="0 0 512 512" role="img" aria-label="{name} avatar"><rect width="512" height="512" rx="112" fill="{ink}"/><circle cx="256" cy="256" r="164" fill="{primary}"/><text x="256" y="288" text-anchor="middle" font-family="Inter, Arial, sans-serif" font-size="132" font-weight="900" fill="white">{initials}</text></svg>"##
            ),
            usage_notes: vec![
                "Use anywhere the brand needs to fit into a square or circle.".to_string(),
                "Best for Instagram, TikTok, app icons, and favicon seeds.".to_string(),
            ],
        },
    ]
}

fn synthesize_catalog(brand_dna: &BrandDna) -> Vec<CatalogItem> {
    let is_service = brand_dna.business_type.contains("Service");
    vec![
        CatalogItem {
            name: if is_service { "Signature Service" } else { "Signature Product" }.to_string(),
            price: if is_service { "From $99" } else { "$29" }.to_string(),
            description: format!(
                "The first offer customers should understand from {}: clear, useful, and easy to act on.",
                brand_dna.name
            ),
            photo_prompt: format!(
                "Realistic {} hero image for {}, clean natural light, brand colors {}.",
                brand_dna.business_type.to_lowercase(),
                brand_dna.name,
                brand_dna.colors.join(", ")
            ),
            seo_title: format!("{} | {}", if is_service { "Book" } else { "Buy" }, brand_dna.name),
        },
        CatalogItem {
            name: if is_service { "Quick Consult" } else { "Starter Bundle" }.to_string(),
            price: if is_service { "$49" } else { "$59" }.to_string(),
            description: "A lower-friction entry point designed for first-time customers.".to_string(),
            photo_prompt: "Lifestyle shot showing the customer outcome, not a generic stock scene.".to_string(),
            seo_title: format!("{} starter offer", brand_dna.name),
        },
    ]
}

fn synthesize_social_calendar(brand_dna: &BrandDna) -> Vec<SocialCalendarItem> {
    let days = [
        ("Monday", "Instagram", "Meet the brand"),
        ("Tuesday", "Google Business Profile", "Customer proof"),
        ("Wednesday", "Email", "Featured offer"),
        ("Thursday", "Instagram", "Behind the scenes"),
        ("Friday", "SMS", "Weekend push"),
        ("Saturday", "Facebook", "Local reminder"),
        ("Sunday", "Instagram", "Next-week teaser"),
    ];

    days.iter()
        .map(|(day, channel, theme)| SocialCalendarItem {
            day: (*day).to_string(),
            channel: (*channel).to_string(),
            caption: format!(
                "{} from {}: a {} update with one clear next step.",
                theme, brand_dna.name, brand_dna.tone_of_voice.join(", ")
            ),
            visual_prompt: format!(
                "{} visual for {}, using {}.",
                theme,
                brand_dna.name,
                brand_dna.image_style.join("; ")
            ),
            call_to_action: "Tap to buy, book, or message us.".to_string(),
        })
        .collect()
}

fn synthesize_brand_toolbox(payload: &GenerateBrandToolboxRequest) -> BrandToolboxResponse {
    let brand_dna = synthesize_brand_dna(
        &payload.description,
        payload.website_url.as_deref(),
        payload.product_url.as_deref(),
        &payload.uploaded_asset_names,
    );
    let campaign_focus = payload
        .campaign_prompt
        .clone()
        .unwrap_or_else(|| "launch, trust building, and repeat purchases".to_string());
    let store_profile = synthesize_store_profile(&payload.description, Some(&brand_dna));

    BrandToolboxResponse {
        id: None,
        logo_concepts: synthesize_logo_concepts(&brand_dna),
        catalog: synthesize_catalog(&brand_dna),
        social_calendar: synthesize_social_calendar(&brand_dna),
        brand_book: vec![
            BrandBookSection {
                title: "Identity".to_string(),
                guidance: vec![
                    brand_dna.positioning.clone(),
                    format!("Primary audience: {}", brand_dna.audience),
                    format!("Business type: {}", brand_dna.business_type),
                ],
            },
            BrandBookSection {
                title: "Voice".to_string(),
                guidance: vec![
                    format!("Use this tone: {}.", brand_dna.tone_of_voice.join(", ")),
                    "Lead with the customer outcome, then show the product or booking action.".to_string(),
                    "Keep captions short enough for mobile scanning.".to_string(),
                ],
            },
            BrandBookSection {
                title: "Visual System".to_string(),
                guidance: vec![
                    format!("Palette: {}.", brand_dna.colors.join(", ")),
                    format!("Fonts: {}.", brand_dna.fonts.join(", ")),
                    format!("Photography: {}.", brand_dna.image_style.join("; ")),
                ],
            },
        ],
        campaign_ideas: vec![
            CampaignIdea {
                title: "First Impression Sprint".to_string(),
                goal: "Make the brand feel credible within the first visit.".to_string(),
                channels: vec!["Website".to_string(), "Instagram".to_string(), "Google Business Profile".to_string()],
                hook: format!("Meet {}: built around {}", brand_dna.name, campaign_focus),
            },
            CampaignIdea {
                title: "Product Proof".to_string(),
                goal: "Turn product or service visuals into buyer confidence.".to_string(),
                channels: vec!["Instagram".to_string(), "Facebook".to_string(), "Email".to_string()],
                hook: "Show the real product, the process, and the finished customer outcome.".to_string(),
            },
            CampaignIdea {
                title: "Weekend Conversion Push".to_string(),
                goal: "Create urgency without sounding generic.".to_string(),
                channels: vec!["SMS".to_string(), "Email".to_string(), "Link in Bio".to_string()],
                hook: "A clear limited-time offer with one tap to buy or book.".to_string(),
            },
        ],
        assets: vec![
            GeneratedBrandAsset {
                asset_type: "Social Post".to_string(),
                channel: "Instagram".to_string(),
                title: "Brand Introduction".to_string(),
                copy: format!("Meet {}. Clear, useful, and ready when you are.", brand_dna.name),
                visual_prompt: format!("A bright, realistic image for {} using brand colors {}.", brand_dna.name, brand_dna.colors.join(", ")),
                editable_fields: vec!["caption".to_string(), "image".to_string(), "call_to_action".to_string()],
            },
            GeneratedBrandAsset {
                asset_type: "Ad Creative".to_string(),
                channel: "Meta Ads".to_string(),
                title: "High-Intent Offer".to_string(),
                copy: "A simple offer, a real visual, and one clear next step.".to_string(),
                visual_prompt: "Clean product-forward ad layout with readable mobile text and strong contrast.".to_string(),
                editable_fields: vec!["headline".to_string(), "body".to_string(), "image".to_string(), "destination_url".to_string()],
            },
            GeneratedBrandAsset {
                asset_type: "Email".to_string(),
                channel: "Customer Broadcast".to_string(),
                title: "Launch Announcement".to_string(),
                copy: format!("A short branded email introducing {} and the first thing customers should try.", brand_dna.name),
                visual_prompt: "Header image with authentic product or service context, no generic stock styling.".to_string(),
                editable_fields: vec!["subject".to_string(), "preview_text".to_string(), "body".to_string(), "hero_image".to_string()],
            },
            GeneratedBrandAsset {
                asset_type: "Website Block".to_string(),
                channel: "Storefront".to_string(),
                title: "Trust Section".to_string(),
                copy: "A compact proof section with customer outcome, delivery/booking details, and a direct CTA.".to_string(),
                visual_prompt: "Mobile-first website section using the brand palette and real product/service photos.".to_string(),
                editable_fields: vec!["headline".to_string(), "proof_points".to_string(), "cta".to_string()],
            },
        ],
        photoshoot: PhotoshootPlan {
            product_source: payload
                .product_url
                .clone()
                .or_else(|| payload.uploaded_asset_names.first().cloned())
                .unwrap_or_else(|| "Upload a product or service photo".to_string()),
            templates: vec!["Studio".to_string(), "Lifestyle".to_string(), "Seasonal Campaign".to_string(), "Website Hero".to_string()],
            prompts: vec![
                format!("Create a clean studio image for {} using the palette {}.", brand_dna.name, brand_dna.colors.join(", ")),
                "Place the product in a believable lifestyle scene with natural light and no distracting props.".to_string(),
                "Generate a website hero crop with safe space for a short headline and button.".to_string(),
            ],
            shots: vec![
                GeneratedPhotoShot {
                    title: "Studio Product Shot".to_string(),
                    format: "1:1 square".to_string(),
                    prompt: format!("Studio-quality image for {} with clean background and brand palette {}.", brand_dna.name, brand_dna.colors.join(", ")),
                    usage: "Catalog, product detail, Instagram grid".to_string(),
                    mockup_svg: format!(
                        r##"<svg xmlns="http://www.w3.org/2000/svg" width="512" height="512" viewBox="0 0 512 512"><rect width="512" height="512" rx="48" fill="#F8FAFC"/><circle cx="256" cy="220" r="118" fill="{}" opacity="0.18"/><rect x="156" y="176" width="200" height="160" rx="28" fill="{}"/><text x="256" y="392" text-anchor="middle" font-family="Inter, Arial, sans-serif" font-size="24" font-weight="700" fill="{}">{}</text></svg>"##,
                        brand_dna.colors.get(1).map(String::as_str).unwrap_or("#2563EB"),
                        brand_dna.colors.get(3).map(String::as_str).unwrap_or("#14B8A6"),
                        brand_dna.colors.first().map(String::as_str).unwrap_or("#0F172A"),
                        xml_escape(&brand_dna.name),
                    ),
                },
                GeneratedPhotoShot {
                    title: "Lifestyle Hero".to_string(),
                    format: "16:9 landscape".to_string(),
                    prompt: "Believable customer context, natural light, safe space for headline and button.".to_string(),
                    usage: "Website hero, email header, ad creative".to_string(),
                    mockup_svg: format!(
                        r##"<svg xmlns="http://www.w3.org/2000/svg" width="960" height="540" viewBox="0 0 960 540"><rect width="960" height="540" fill="#F8FAFC"/><rect x="56" y="56" width="848" height="428" rx="32" fill="{}" opacity="0.14"/><circle cx="736" cy="198" r="104" fill="{}" opacity="0.55"/><rect x="112" y="308" width="384" height="44" rx="12" fill="{}"/><rect x="112" y="372" width="260" height="28" rx="10" fill="{}" opacity="0.7"/><text x="112" y="250" font-family="Outfit, Inter, Arial, sans-serif" font-size="56" font-weight="800" fill="{}">{}</text></svg>"##,
                        brand_dna.colors.get(1).map(String::as_str).unwrap_or("#2563EB"),
                        brand_dna.colors.get(3).map(String::as_str).unwrap_or("#14B8A6"),
                        brand_dna.colors.get(1).map(String::as_str).unwrap_or("#2563EB"),
                        brand_dna.colors.get(3).map(String::as_str).unwrap_or("#14B8A6"),
                        brand_dna.colors.first().map(String::as_str).unwrap_or("#0F172A"),
                        xml_escape(&brand_dna.name),
                    ),
                },
                GeneratedPhotoShot {
                    title: "Story Promo".to_string(),
                    format: "9:16 vertical".to_string(),
                    prompt: "Mobile story composition with product/service foreground and minimal text area.".to_string(),
                    usage: "Stories, reels cover, SMS landing page".to_string(),
                    mockup_svg: format!(
                        r##"<svg xmlns="http://www.w3.org/2000/svg" width="405" height="720" viewBox="0 0 405 720"><rect width="405" height="720" fill="{}"/><rect x="36" y="68" width="333" height="420" rx="36" fill="#F8FAFC" opacity="0.92"/><circle cx="204" cy="274" r="106" fill="{}" opacity="0.26"/><text x="204" y="554" text-anchor="middle" font-family="Outfit, Inter, Arial, sans-serif" font-size="36" font-weight="800" fill="#F8FAFC">{}</text><rect x="96" y="594" width="214" height="46" rx="23" fill="#F8FAFC" opacity="0.9"/></svg>"##,
                        brand_dna.colors.first().map(String::as_str).unwrap_or("#0F172A"),
                        brand_dna.colors.get(3).map(String::as_str).unwrap_or("#14B8A6"),
                        xml_escape(&brand_dna.name),
                    ),
                },
            ],
            refinement_controls: vec![
                "change background".to_string(),
                "match style reference".to_string(),
                "crop by channel".to_string(),
                "add to Brand DNA".to_string(),
            ],
        },
        store_profile,
        editable_controls: vec![
            "Edit copy".to_string(),
            "Regenerate variants".to_string(),
            "Replace image".to_string(),
            "Lock brand colors".to_string(),
            "Resize by channel".to_string(),
            "Schedule or publish".to_string(),
        ],
        export_formats: vec!["PNG".to_string(), "JPG".to_string(), "PDF brand book".to_string(), "HTML website draft".to_string(), "Social calendar JSON".to_string()],
        brand_dna,
    }
}

async fn generate_brand_toolbox(
    State(pool): State<PgPool>,
    Extension(claims): Extension<Claims>,
    Json(payload): Json<GenerateBrandToolboxRequest>,
) -> Result<Json<BrandToolboxResponse>, axum::http::StatusCode> {
    let tenant_id = Uuid::parse_str(&claims.organization_id.unwrap_or_default())
        .map_err(|_| axum::http::StatusCode::UNAUTHORIZED)?;
    let mut toolbox = synthesize_brand_toolbox(&payload);
    let toolbox_json =
        serde_json::to_value(&toolbox).map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;
    let record = db::create_brand_toolbox(
        &pool,
        tenant_id,
        toolbox.brand_dna.name.clone(),
        payload.description,
        toolbox_json,
    )
    .await
    .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;
    toolbox.id = Some(record.id);
    Ok(Json(toolbox))
}

fn brand_toolbox_from_record(
    record: db::BrandToolbox,
) -> Result<BrandToolboxResponse, axum::http::StatusCode> {
    let _ = (&record.tenant_id, &record.name, &record.source_description);
    let mut toolbox: BrandToolboxResponse = serde_json::from_value(record.toolbox)
        .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;
    toolbox.id = Some(record.id);
    Ok(toolbox)
}

async fn get_brand_toolbox(
    State(pool): State<PgPool>,
    Path(toolbox_id): Path<Uuid>,
    Extension(claims): Extension<Claims>,
) -> Result<Json<BrandToolboxResponse>, axum::http::StatusCode> {
    let tenant_id = Uuid::parse_str(&claims.organization_id.unwrap_or_default())
        .map_err(|_| axum::http::StatusCode::UNAUTHORIZED)?;
    let record = db::get_brand_toolbox(&pool, tenant_id, toolbox_id)
        .await
        .map_err(|_| axum::http::StatusCode::NOT_FOUND)?;
    Ok(Json(brand_toolbox_from_record(record)?))
}

async fn list_brand_toolboxes(
    State(pool): State<PgPool>,
    Extension(claims): Extension<Claims>,
) -> Result<Json<Vec<BrandToolboxResponse>>, axum::http::StatusCode> {
    let tenant_id = Uuid::parse_str(&claims.organization_id.unwrap_or_default())
        .map_err(|_| axum::http::StatusCode::UNAUTHORIZED)?;
    let records = db::list_brand_toolboxes(&pool, tenant_id)
        .await
        .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;
    let mut toolboxes = Vec::with_capacity(records.len());
    for record in records {
        toolboxes.push(brand_toolbox_from_record(record)?);
    }
    Ok(Json(toolboxes))
}

async fn publish_brand_toolbox_website(
    State(pool): State<PgPool>,
    Path(toolbox_id): Path<Uuid>,
    Extension(claims): Extension<Claims>,
) -> Result<Json<SiteResponse>, axum::http::StatusCode> {
    let tenant_id = Uuid::parse_str(&claims.organization_id.unwrap_or_default())
        .map_err(|_| axum::http::StatusCode::UNAUTHORIZED)?;
    let record = db::get_brand_toolbox(&pool, tenant_id, toolbox_id)
        .await
        .map_err(|_| axum::http::StatusCode::NOT_FOUND)?;
    let toolbox = brand_toolbox_from_record(record)?;
    let raw_slug = toolbox
        .brand_dna
        .name
        .to_lowercase()
        .chars()
        .map(|ch| if ch.is_ascii_alphanumeric() { ch } else { '-' })
        .collect::<String>();
    let slug = raw_slug
        .split('-')
        .filter(|part| !part.is_empty())
        .collect::<Vec<_>>()
        .join("-");
    let domain = if slug.is_empty() {
        Some("brand-toolbox.ohc.store".to_string())
    } else {
        Some(format!("{}.ohc.store", slug))
    };
    let site = publish_store_profile(&pool, tenant_id, domain, toolbox.store_profile).await?;
    Ok(Json(site))
}

async fn load_latest_brand_dna(pool: &PgPool, tenant_id: Uuid) -> Option<BrandDna> {
    let toolbox_json: serde_json::Value = sqlx::query_scalar(
        r#"
        SELECT toolbox
        FROM builder_brand_toolboxes
        WHERE tenant_id = $1
        ORDER BY updated_at DESC, created_at DESC
        LIMIT 1
        "#,
    )
    .bind(tenant_id)
    .fetch_optional(pool)
    .await
    .ok()
    .flatten()?;

    serde_json::from_value::<BrandToolboxResponse>(toolbox_json)
        .ok()
        .map(|toolbox| toolbox.brand_dna)
}








async fn generate_storefront(
    State(pool): State<PgPool>,
    Extension(claims): Extension<Claims>,
    Json(payload): Json<GenerateStorefrontRequest>,
) -> Result<Json<StoreProfile>, axum::http::StatusCode> {
    let tenant_id = Uuid::parse_str(&claims.organization_id.unwrap_or_default())
        .map_err(|_| axum::http::StatusCode::UNAUTHORIZED)?;
    let persisted_brand_dna = if payload.brand_dna.is_none() {
        load_latest_brand_dna(&pool, tenant_id).await
    } else {
        None
    };
    let active_brand_dna = payload
        .brand_dna
        .as_ref()
        .or(persisted_brand_dna.as_ref());

    let api_key = std::env::var("MINIMAX_API_KEY").unwrap_or_default();
    if api_key.trim().is_empty() {
        return Ok(Json(synthesize_store_profile(
            &payload.description,
            active_brand_dna,
        )));
    }
    let minimax = crate::minimax::MinimaxClient::new(api_key);

    // Step 1: The Advisor extracts metadata
    let advisor_prompt = format!(
        r#"You are The Advisor. Extract business metadata from the following description.
User Description: "{}"
Return a JSON object strictly matching this structure:
{{
  "name": "...",
  "business_type": "...",
  "vibe": "..."
}}
Only return the JSON. No markdown formatting, no explanations."#,
        payload.description
    );

    let mut attempts = 0;
    let mut ai_res_advisor = String::new();
    let mut ai_call_succeeded = false;
    while attempts < 3 {
        match tokio::time::timeout(std::time::Duration::from_secs(60), minimax.reason(&advisor_prompt)).await {
            Ok(Ok(res)) => {
                ai_res_advisor = res;
                ai_call_succeeded = true;
                break;
            },
            _ => {
                attempts += 1;
                tokio::time::sleep(std::time::Duration::from_secs(2u64.pow(attempts))).await;
            }
        }
    }

    let business_context: BusinessContext = if ai_call_succeeded {
        let cleaned_advisor = clean_model_json(&ai_res_advisor);
        serde_json::from_str(cleaned_advisor).unwrap_or_else(|e| {
            tracing::warn!("Failed to parse JSON from Advisor AI, using heuristic context: {}", e);
            infer_business_context(&payload.description, active_brand_dna.clone())
        })
    } else {
        tracing::warn!("Advisor AI unavailable, using heuristic context");
        infer_business_context(&payload.description, active_brand_dna.clone())
    };

    let source_context = [
        payload.website_url.as_ref().map(|url| format!("Website URL: {}", url)),
        payload.product_url.as_ref().map(|url| format!("Product URL: {}", url)),
        payload.campaign_prompt.as_ref().map(|prompt| format!("Campaign prompt: {}", prompt)),
        (!payload.uploaded_asset_names.is_empty()).then(|| format!("Uploaded assets: {}", payload.uploaded_asset_names.join(", "))),
    ]
    .into_iter()
    .flatten()
    .collect::<Vec<_>>()
    .join("\n");

    // Step 2: The Promoter generates the layout and content
    let promoter_prompt = format!(
        r#"You are The Promoter (Marketing & Advertising & SEO). Your task is to architect a mobile-first storefront that looks premium and reflects the user's business goal.
Use the following business context extracted by The Advisor:
Name: {}
Type: {}
Vibe: {}

Original User Description: "{}"
Additional brand/product grounding:
{}

First, synthesize the context to select an appropriate template, generate copywriting, and select relevant concepts.
Second, act as The Promoter (SEO) to automatically generate structured JSON-LD schemas based on the tenant\'s product catalog and chosen business type.
Then, instantly generate a structural layout draft that optimizes for the 375px viewport.
You must also act as the Operations and Finance agents to generate 3 sample products (in 'sample_products'), default shipping settings, and default tax settings.

The JSON must exactly match this structure:
{{
  "domain": null,
  "theme": "Glassmorphism",
  "sample_products": [
    {{"name": "...", "price": 10.0, "description": "..."}}
  ],
  "shipping_settings": {{
    "default_rate": 5.0,
    "free_shipping_threshold": 50.0
  }},
  "tax_settings": {{
    "default_tax_rate": 0.08
  }},
  "pages": [
    {{
      "path": "/",
      "title": "Home",
      "blocks": [
        {{
          "block_type": "HeroBlock",
          "content": {{ "headline": "...", "subtitle": "..." }},
          "sort_order": 0
        }},
        {{
          "block_type": "ProductGridBlock",
          "content": {{ "items": [{{ "name": "...", "price": "...", "description": "..." }}] }},
          "sort_order": 1
        }},
        {{
          "block_type": "ServiceBookingBlock",
          "content": {{ "title": "...", "availability": "..." }},
          "sort_order": 2
        }},
        {{
          "block_type": "TestimonialBlock",
          "content": {{ "quotes": [{{ "text": "...", "author": "..." }}] }},
          "sort_order": 3
        }}
      ],
      "seo_metadata": {{
        "@context": "https://schema.org",
        "@type": "LocalBusiness",
        "name": "...",
        "description": "..."
      }}
    }}
  ]
}}
Only return the JSON. No markdown formatting, no explanations. Make sure the blocks (HeroBlock, ProductGridBlock, ServiceBookingBlock, TestimonialBlock) and sample products perfectly reflect the extracted entities."#,
        business_context.name,
        business_context.business_type,
        business_context.vibe,
        payload.description,
        source_context
    );

    let site_draft: StoreProfile = match minimax.reason(&promoter_prompt).await {
        Ok(promoter_response) => {
            let cleaned_response = clean_model_json(&promoter_response);
            serde_json::from_str(cleaned_response).unwrap_or_else(|e| {
                tracing::warn!("Failed to parse JSON from Promoter AI, using heuristic storefront: {}", e);
                synthesize_store_profile(&payload.description, active_brand_dna)
            })
        },
        Err(e) => {
            tracing::warn!("Promoter AI unavailable, using heuristic storefront: {}", e);
            synthesize_store_profile(&payload.description, active_brand_dna)
        }
    };

    Ok(Json(site_draft))
}

async fn publish_draft(
    State(pool): State<PgPool>,
    Extension(claims): Extension<Claims>,
    Json(payload): Json<PublishDraftRequest>,
) -> Result<Json<SiteResponse>, axum::http::StatusCode> {
    let tenant_id = Uuid::parse_str(&claims.organization_id.unwrap_or_default())
        .map_err(|_| axum::http::StatusCode::UNAUTHORIZED)?;
    let site = publish_store_profile(&pool, tenant_id, payload.domain, payload.draft).await?;
    Ok(Json(site))
}

async fn publish_store_profile(
    pool: &PgPool,
    tenant_id: Uuid,
    domain: Option<String>,
    draft: StoreProfile,
) -> Result<SiteResponse, axum::http::StatusCode> {
    let mut tx = pool
        .begin()
        .await
        .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;
    sqlx::query("SELECT set_config('app.current_tenant', $1, true)")
        .bind(tenant_id.to_string())
        .execute(&mut *tx)
        .await
        .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;

    let site: SiteResponse = sqlx::query_as(
        "INSERT INTO builder_sites (tenant_id, domain) VALUES ($1, $2) RETURNING id, domain",
    )
    .bind(tenant_id)
    .bind(domain)
    .fetch_one(&mut *tx)
    .await
    .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;

    for draft_page in draft.pages {
        let page_id: Uuid = sqlx::query_scalar(
            r#"
            INSERT INTO builder_pages (tenant_id, site_id, path, title, seo_metadata)
            VALUES ($1, $2, $3, $4, $5)
            RETURNING id
            "#,
        )
        .bind(tenant_id)
        .bind(site.id)
        .bind(draft_page.path)
        .bind(draft_page.title)
        .bind(draft_page.seo_metadata)
        .fetch_one(&mut *tx)
        .await
        .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;

        for draft_block in draft_page.blocks {
            sqlx::query(
                r#"
                INSERT INTO builder_blocks (tenant_id, page_id, block_type, content, sort_order)
                VALUES ($1, $2, $3, $4, $5)
                "#,
            )
            .bind(tenant_id)
            .bind(page_id)
            .bind(draft_block.block_type)
            .bind(draft_block.content)
            .bind(draft_block.sort_order)
            .execute(&mut *tx)
            .await
            .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;
        }
    }

    tx.commit()
        .await
        .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;

    if let Err(err) = jobs::enqueue_publish_site_job(pool, tenant_id, site.id).await {
        tracing::warn!("Failed to enqueue publish job for site {}: {}", site.id, err);
    }

    Ok(site)
}
