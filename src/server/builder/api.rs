use axum::{
    extract::Extension,
    extract::{Path, State},
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
pub struct SiteDraft {
    pub domain: Option<String>,
    pub pages: Vec<DraftPage>,
}

#[derive(Deserialize)]
pub struct GenerateStorefrontRequest {
    pub description: String,
}

#[derive(Deserialize)]
pub struct PublishDraftRequest {
    pub domain: Option<String>,
    pub draft: SiteDraft,
}


use super::db;
use super::jobs;

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
    Router::new()
        .route("/sites", get(list_sites).post(create_site))
        .route("/sites/{site_id}/pages", get(list_pages).post(create_page))
        .route(
            "/pages/{page_id}/blocks",
            get(list_blocks).post(create_block),
        )
        .route("/blocks/{block_id}", put(update_block))
        .route("/pages/{page_id}/blocks/reorder", post(reorder_blocks))
        .route("/sites/{site_id}/publish", post(publish_site))
        .route("/generate", post(generate_storefront))
        .route("/publish_draft", post(publish_draft))
        .route("/geo_score", post(geo_score))
        .route("/auto_seo", post(auto_seo))
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

#[derive(Serialize)]
#[derive(serde::Deserialize)]
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
    Ok(Json(PageResponse {
        id: page.id,
        path: page.path,
        title: page.title,
        seo_metadata: page.seo_metadata,
    }))
}

#[derive(Serialize)]
#[derive(serde::Deserialize)]
pub struct BlockResponse {
    pub id: Uuid,
    pub block_type: String,
    pub content: Value,
    pub sort_order: i32,
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










async fn generate_storefront(
    Extension(claims): Extension<Claims>,
    Json(payload): Json<GenerateStorefrontRequest>,
) -> Result<Json<SiteDraft>, axum::http::StatusCode> {
    let _tenant_id = Uuid::parse_str(&claims.organization_id.unwrap_or_default()).map_err(|_| axum::http::StatusCode::UNAUTHORIZED)?;

    let api_key = std::env::var("MINIMAX_API_KEY").unwrap_or_default();
    let minimax = crate::minimax::MinimaxClient::new(api_key);

    let prompt = format!(
        r#"You are The Promoter (Marketing & Advertising & SEO). Your task is to architect a mobile-first storefront that looks premium and reflects the user's business goal.
First, synthesize the user's business description to select an appropriate template, generate copywriting, and select relevant concepts.
Second, act as The Promoter (SEO) to automatically generate meta tags, descriptions, and sitemaps based on the chosen business type and generated content.
Then, instantly generate a structural layout draft that optimizes for the 375px viewport.

User Description: "{}"

The JSON must exactly match this structure:
{{
  "domain": null,
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
Only return the JSON. No markdown formatting, no explanations. Make sure the blocks (HeroBlock, ProductGridBlock, ServiceBookingBlock, TestimonialBlock) perfectly reflect the extracted entities."#,
        payload.description
    );

    let response = minimax.reason(&prompt).await.map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;

    // Clean up response if it contains markdown formatting
    let cleaned_response = response.trim().trim_start_matches("```json").trim_start_matches("```").trim_end_matches("```").trim();

    let site_draft: SiteDraft = serde_json::from_str(cleaned_response).map_err(|e| {
        tracing::error!("Failed to parse JSON from AI: {}", e);
        axum::http::StatusCode::INTERNAL_SERVER_ERROR
    })?;

    Ok(Json(site_draft))
}

async fn publish_draft(
    State(pool): State<PgPool>,
    Extension(claims): Extension<Claims>,
    Json(payload): Json<PublishDraftRequest>,
) -> Result<Json<SiteResponse>, axum::http::StatusCode> {
    let tenant_id = Uuid::parse_str(&claims.organization_id.unwrap_or_default()).map_err(|_| axum::http::StatusCode::UNAUTHORIZED)?;

    // 1. Create Site
    let site = db::create_site(&pool, tenant_id, payload.domain)
        .await
        .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;

    // 2. Iterate pages and blocks
    for draft_page in payload.draft.pages {
        let page = db::create_page(&pool, tenant_id, site.id, draft_page.path, draft_page.title)
            .await
            .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;

        // Update SEO metadata
        sqlx::query("UPDATE builder_pages SET seo_metadata = $1 WHERE id = $2")
            .bind(&draft_page.seo_metadata)
            .bind(page.id)
            .execute(&pool)
            .await
            .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;

        for draft_block in draft_page.blocks {
            db::create_block(&pool, tenant_id, page.id, draft_block.block_type, draft_block.content, draft_block.sort_order)
                .await
                .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;
        }
    }

    // 3. Enqueue Job
    jobs::enqueue_publish_site_job(&pool, tenant_id, site.id)
        .await
        .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(SiteResponse {
        id: site.id,
        domain: site.domain,
    }))
}
