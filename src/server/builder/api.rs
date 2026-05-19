use crate::minimax::MinimaxClient;
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
        .with_state(pool)
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
    if payload.block_type != "HeroBlock" && payload.block_type != "ProductGridBlock" && payload.block_type != "ContactFormBlock" && payload.block_type != "BookingCalendarBlock" && payload.block_type != "ServiceBookingBlock" && payload.block_type != "TestimonialBlock" { return Err(axum::http::StatusCode::BAD_REQUEST); }
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
    let client = MinimaxClient::new(api_key);

    let system_prompt = r#"
You are an expert web designer AI. Your task is to generate a JSON structure for a website based on a description.
The JSON must perfectly match this schema and be returned inside a ```json``` code block:
{
  "domain": null,
  "pages": [
    {
      "path": "/",
      "title": "Home",
      "blocks": [
        {
          "block_type": "HeroBlock",
          "content": {"headline": "...", "subtitle": "..."},
          "sort_order": 0
        },
        {
          "block_type": "ProductGridBlock",
          "content": {"items": ["...", "..."]},
          "sort_order": 1
        }
      ],
      "seo_metadata": {
        "@context": "https://schema.org",
        "@type": "...",
        "name": "..."
      }
    }
  ]
}
Allowed block types: "HeroBlock", "ProductGridBlock", "ContactFormBlock", "BookingCalendarBlock", "ServiceBookingBlock", "TestimonialBlock".
Make the design specific to the user description.
"#;
    let prompt = format!("{}
User Description: {}", system_prompt, payload.description);

    let ai_res = client.reason(&prompt).await.map_err(|e| {
        tracing::error!("LLM Error: {}", e);
        axum::http::StatusCode::INTERNAL_SERVER_ERROR
    })?;

    // Extract JSON block
    let json_str = if let Some(start) = ai_res.find("```json") {
        let after_start = &ai_res[start + 7..];
        if let Some(end) = after_start.find("```") {
            &after_start[..end]
        } else {
            after_start
        }
    } else {
        &ai_res
    };

    let draft: SiteDraft = serde_json::from_str(json_str.trim()).map_err(|e| {
        tracing::error!("Failed to parse LLM JSON: {}. LLM Response: {}", e, ai_res);
        axum::http::StatusCode::INTERNAL_SERVER_ERROR
    })?;

    Ok(Json(draft))
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
