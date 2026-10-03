use ::server_common::Claims;
use axum::{
    Json, Router,
    extract::Extension,
    extract::{Path, State},
    middleware::{self, Next},
    response::Response,
    routing::{get, post, put},
};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx::PgPool;
use uuid::Uuid;

pub use super::generation::{
    BrandDna, BrandToolboxResponse, DraftBlock, DraftPage, PublishDraftRequest, StoreProfile,
};

use super::db;
use super::jobs;

fn default_builder_tenant_id() -> Uuid {
    let raw =
        std::env::var("OMNISOLO_DEFAULT_TENANT_ID").unwrap_or_else(|_| "e2e-tenant".to_string());
    Uuid::parse_str(&raw).unwrap_or_else(|_| {
        // Keep string tenant defaults stable across requests so generate/publish
        // flows can read the same records under Bazel and local dev.
        Uuid::parse_str("00000000-0000-0000-0000-000000000001").expect("static UUID")
    })
}

async fn ensure_builder_claims(mut req: axum::extract::Request, next: Next) -> Response {
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
        "HeroBlock" => content.get("headline").is_some() && content.get("subtitle").is_some(),
        "ProductGridBlock" => content.get("items").and_then(|v| v.as_array()).is_some(),
        "ServiceBookingBlock" => {
            content.get("title").is_some() && content.get("availability").is_some()
        }
        "TestimonialBlock" => content.get("quotes").and_then(|v| v.as_array()).is_some(),
        "ContactFormBlock" | "BookingCalendarBlock" => content.is_object(),
        _ => false,
    }
}

pub fn router<S: Clone + Send + Sync + 'static>(pool: PgPool) -> axum::Router<S> {
    let edge_state = std::sync::Arc::new(super::edge::EdgeWorkerState { pool: pool.clone() });

    Router::new()
        .route(
            "/edge/{tenant_id}/{site_id}",
            get(super::edge::StorefrontRouter::handle_edge_request).layer(
                axum::middleware::from_fn(
                    crate::utils::edge_caching_middleware::edge_caching_middleware,
                ),
            ),
        )
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
        .merge(super::generation::router(pool.clone()))
        .route("/brand_toolbox", get(list_brand_toolboxes))
        .route("/brand_toolbox/{toolbox_id}", get(get_brand_toolbox))
        .route(
            "/brand_toolbox/{toolbox_id}/publish_website",
            post(publish_brand_toolbox_website),
        )
        .route("/publish_draft", post(publish_draft))
        .route("/auto_seo", post(auto_seo))
        .route_layer(middleware::from_fn(ensure_builder_claims))
        .layer(axum::Extension(edge_state))
        .with_state(pool)
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

fn parse_tenant_id(organization_id: Option<&str>) -> Result<Uuid, axum::http::StatusCode> {
    let org = organization_id.unwrap_or_default().trim();
    if org.is_empty() {
        return Err(axum::http::StatusCode::UNAUTHORIZED);
    }
    match Uuid::parse_str(org) {
        Ok(u) => Ok(u),
        Err(_) => Ok(Uuid::new_v5(&Uuid::NAMESPACE_DNS, org.as_bytes())),
    }
}

#[derive(Deserialize)]
pub struct CreateSiteRequest {
    pub domain: Option<String>,
}

async fn list_sites(
    State(pool): State<PgPool>,
    Extension(claims): Extension<Claims>,
) -> Result<Json<Vec<SiteResponse>>, axum::http::StatusCode> {
    let tenant_id = parse_tenant_id(claims.organization_id.as_deref())?;
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
    let tenant_id = parse_tenant_id(claims.organization_id.as_deref())?;
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

    let tenant_id = parse_tenant_id(claims.organization_id.as_deref())?;

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
            seo_metadata: row
                .page_seo_metadata
                .clone()
                .unwrap_or_else(|| serde_json::json!({})),
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
        "description": "Generated by OmniSolo Auto SEO"
    });

    Ok(Json(schema_json))
}

#[derive(Serialize, serde::Deserialize)]
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
    let tenant_id = parse_tenant_id(claims.organization_id.as_deref())?;
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
    let tenant_id = parse_tenant_id(claims.organization_id.as_deref())?;
    let page = db::create_page(&pool, tenant_id, site_id, payload.path, payload.title)
        .await
        .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;

    if let Err(err) = jobs::enqueue_publish_site_job(&pool, tenant_id, site_id).await {
        tracing::error!(
            "Failed to enqueue publish job for site {}: {}",
            site_id,
            err
        );
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
    let tenant_id = parse_tenant_id(claims.organization_id.as_deref())?;
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
    let tenant_id = parse_tenant_id(claims.organization_id.as_deref())?;
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
    cache
        .invalidate_by_tag(&format!("tenant-id:{}", tenant_id))
        .await;

    let pool_clone = pool.clone();
    tokio::spawn(async move {
        let site_id_query =
            sqlx::query_scalar::<_, Uuid>("SELECT site_id FROM builder_pages WHERE id = $1")
                .bind(page_id)
                .fetch_optional(&pool_clone)
                .await;

        if let Ok(Some(site_id)) = site_id_query {
            let cache_key = format!("edge_site_{}_{}_en-US", tenant_id, site_id);
            let _ = crate::builder::edge::regenerate_cache(
                pool_clone.clone(),
                tenant_id,
                site_id,
                cache_key,
                cache.clone(),
            )
            .await;

            if let Err(err) =
                crate::builder::jobs::enqueue_publish_site_job(&pool_clone, tenant_id, site_id)
                    .await
            {
                tracing::error!(
                    "Failed to enqueue publish job for site {}: {}",
                    site_id,
                    err
                );
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
    let tenant_id = parse_tenant_id(claims.organization_id.as_deref())?;

    // Fetch block to check its type for validation
    let existing_block = db::get_block(&pool, tenant_id, block_id)
        .await
        .map_err(|_| axum::http::StatusCode::NOT_FOUND)?;
    if !validate_block(&existing_block.block_type, &payload.content) {
        return Err(axum::http::StatusCode::BAD_REQUEST);
    }

    let block = db::update_block(&pool, tenant_id, block_id, payload.content)
        .await
        .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;

    let cache = crate::builder::edge::get_edge_cache();
    cache
        .invalidate_by_tag(&format!("tenant-id:{}", tenant_id))
        .await;

    let pool_clone = pool.clone();
    let page_id = existing_block.page_id;
    tokio::spawn(async move {
        let site_id_query =
            sqlx::query_scalar::<_, Uuid>("SELECT site_id FROM builder_pages WHERE id = $1")
                .bind(page_id)
                .fetch_optional(&pool_clone)
                .await;

        if let Ok(Some(site_id)) = site_id_query {
            let cache_key = format!("edge_site_{}_{}_en-US", tenant_id, site_id);
            let _ = crate::builder::edge::regenerate_cache(
                pool_clone.clone(),
                tenant_id,
                site_id,
                cache_key,
                cache.clone(),
            )
            .await;

            if let Err(err) =
                crate::builder::jobs::enqueue_publish_site_job(&pool_clone, tenant_id, site_id)
                    .await
            {
                tracing::error!(
                    "Failed to enqueue publish job for site {}: {}",
                    site_id,
                    err
                );
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
    let tenant_id = parse_tenant_id(claims.organization_id.as_deref())?;
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
    let tenant_id = parse_tenant_id(claims.organization_id.as_deref())?;
    jobs::enqueue_publish_site_job(&pool, tenant_id, site_id)
        .await
        .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(axum::http::StatusCode::ACCEPTED)
}

fn brand_toolbox_from_record(
    record: db::BrandToolbox,
    raw_tenant: &str,
) -> Result<BrandToolboxResponse, axum::http::StatusCode> {
    let _ = (&record.tenant_id, &record.name, &record.source_description);
    let mut toolbox: BrandToolboxResponse = serde_json::from_value(record.toolbox)
        .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;
    if !super::generation::has_generation_provenance(&toolbox, raw_tenant) {
        return Err(axum::http::StatusCode::CONFLICT);
    }
    toolbox.id = Some(record.id);
    Ok(toolbox)
}

async fn get_brand_toolbox(
    State(pool): State<PgPool>,
    Path(toolbox_id): Path<Uuid>,
    Extension(claims): Extension<Claims>,
) -> Result<Json<BrandToolboxResponse>, axum::http::StatusCode> {
    let tenant_id = parse_tenant_id(claims.organization_id.as_deref())?;
    let record = db::get_brand_toolbox(
        &pool,
        tenant_id,
        claims.organization_id.as_deref().unwrap_or_default(),
        toolbox_id,
    )
    .await
    .map_err(|_| axum::http::StatusCode::NOT_FOUND)?;
    Ok(Json(brand_toolbox_from_record(
        record,
        claims.organization_id.as_deref().unwrap_or_default(),
    )?))
}

async fn list_brand_toolboxes(
    State(pool): State<PgPool>,
    Extension(claims): Extension<Claims>,
) -> Result<Json<Vec<BrandToolboxResponse>>, axum::http::StatusCode> {
    let tenant_id = parse_tenant_id(claims.organization_id.as_deref())?;
    let records = db::list_brand_toolboxes(
        &pool,
        tenant_id,
        claims.organization_id.as_deref().unwrap_or_default(),
    )
    .await
    .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;
    let mut toolboxes = Vec::with_capacity(records.len());
    for record in records {
        toolboxes.push(brand_toolbox_from_record(
            record,
            claims.organization_id.as_deref().unwrap_or_default(),
        )?);
    }
    Ok(Json(toolboxes))
}

async fn publish_brand_toolbox_website(
    State(pool): State<PgPool>,
    Path(toolbox_id): Path<Uuid>,
    Extension(claims): Extension<Claims>,
) -> Result<Json<SiteResponse>, axum::http::StatusCode> {
    let tenant_id = parse_tenant_id(claims.organization_id.as_deref())?;
    let record = db::get_brand_toolbox(
        &pool,
        tenant_id,
        claims.organization_id.as_deref().unwrap_or_default(),
        toolbox_id,
    )
    .await
    .map_err(|_| axum::http::StatusCode::NOT_FOUND)?;
    let toolbox = brand_toolbox_from_record(
        record,
        claims.organization_id.as_deref().unwrap_or_default(),
    )?;
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
        Some("brand-toolbox.cloud.omnisolo.co".to_string())
    } else {
        Some(format!("{}.cloud.omnisolo.co", slug))
    };
    let site = publish_store_profile(&pool, tenant_id, domain, toolbox.store_profile).await?;
    Ok(Json(site))
}

async fn publish_draft(
    State(pool): State<PgPool>,
    Extension(claims): Extension<Claims>,
    Json(payload): Json<PublishDraftRequest>,
) -> Result<Json<SiteResponse>, axum::http::StatusCode> {
    let tenant_id = parse_tenant_id(claims.organization_id.as_deref())?;
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
        tracing::error!(
            "Failed to enqueue publish job for site {}: {}",
            site.id,
            err
        );
    }

    Ok(site)
}
