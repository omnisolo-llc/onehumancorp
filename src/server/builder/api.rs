use axum::{
    extract::{Path, State, Extension},
    routing::{get, post, put},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx::PgPool;
use uuid::Uuid;

use super::db;
use super::jobs;

pub fn router<S: Clone + Send + Sync + 'static>(pool: PgPool) -> axum::Router<S> {
    Router::new()
        .route("/sites", get(list_sites).post(create_site))
        .route("/sites/:site_id/pages", get(list_pages).post(create_page))
        .route(
            "/pages/:page_id/blocks",
            get(list_blocks).post(create_block),
        )
        .route("/blocks/:block_id", put(update_block))
        .route("/pages/:page_id/blocks/reorder", post(reorder_blocks))
        .route("/sites/:site_id/publish", post(publish_site))
        .with_state(pool)
}

#[derive(Serialize)]
#[derive(Debug)]
pub struct SiteResponse {
    pub id: Uuid,
    pub domain: Option<String>,
}

#[derive(Deserialize)]
pub struct CreateSiteRequest {
    pub domain: Option<String>,
}

async fn list_sites(
    axum::extract::Extension(claims): axum::extract::Extension<crate::auth::Claims>,
    State(pool): State<PgPool>,
) -> Result<Json<Vec<SiteResponse>>, axum::http::StatusCode> {
    let tenant_id_str = claims.organization_id.unwrap_or_default();
    let tenant_id = Uuid::parse_str(&tenant_id_str).map_err(|_| axum::http::StatusCode::UNAUTHORIZED)?;
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
    axum::extract::Extension(claims): axum::extract::Extension<crate::auth::Claims>,
    State(pool): State<PgPool>,
    Json(payload): Json<CreateSiteRequest>,
) -> Result<Json<SiteResponse>, axum::http::StatusCode> {
    let tenant_id_str = claims.organization_id.unwrap_or_default();
    let tenant_id = Uuid::parse_str(&tenant_id_str).map_err(|_| axum::http::StatusCode::UNAUTHORIZED)?;
    let site = db::create_site(&pool, tenant_id, payload.domain)
        .await
        .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(Json(SiteResponse {
        id: site.id,
        domain: site.domain,
    }))
}

#[derive(Serialize)]
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
    axum::extract::Extension(claims): axum::extract::Extension<crate::auth::Claims>,
    State(pool): State<PgPool>,
    Path(site_id): Path<Uuid>,
) -> Result<Json<Vec<PageResponse>>, axum::http::StatusCode> {
    let tenant_id_str = claims.organization_id.unwrap_or_default();
    let tenant_id = Uuid::parse_str(&tenant_id_str).map_err(|_| axum::http::StatusCode::UNAUTHORIZED)?;
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
    axum::extract::Extension(claims): axum::extract::Extension<crate::auth::Claims>,
    State(pool): State<PgPool>,
    Path(site_id): Path<Uuid>,
    Json(payload): Json<CreatePageRequest>,
) -> Result<Json<PageResponse>, axum::http::StatusCode> {
    let tenant_id_str = claims.organization_id.unwrap_or_default();
    let tenant_id = Uuid::parse_str(&tenant_id_str).map_err(|_| axum::http::StatusCode::UNAUTHORIZED)?;
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
    axum::extract::Extension(claims): axum::extract::Extension<crate::auth::Claims>,
    State(pool): State<PgPool>,
    Path(page_id): Path<Uuid>,
) -> Result<Json<Vec<BlockResponse>>, axum::http::StatusCode> {
    let tenant_id_str = claims.organization_id.unwrap_or_default();
    let tenant_id = Uuid::parse_str(&tenant_id_str).map_err(|_| axum::http::StatusCode::UNAUTHORIZED)?;
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
    axum::extract::Extension(claims): axum::extract::Extension<crate::auth::Claims>,
    State(pool): State<PgPool>,
    Path(page_id): Path<Uuid>,
    Json(payload): Json<CreateBlockRequest>,
) -> Result<Json<BlockResponse>, axum::http::StatusCode> {
    if payload.block_type != "HeroBlock" && payload.block_type != "ProductGridBlock" && payload.block_type != "ContactFormBlock" && payload.block_type != "BookingCalendarBlock" { return Err(axum::http::StatusCode::BAD_REQUEST); }
    let tenant_id_str = claims.organization_id.unwrap_or_default();
    let tenant_id = Uuid::parse_str(&tenant_id_str).map_err(|_| axum::http::StatusCode::UNAUTHORIZED)?;
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
    axum::extract::Extension(claims): axum::extract::Extension<crate::auth::Claims>,
    State(pool): State<PgPool>,
    Path(block_id): Path<Uuid>,
    Json(payload): Json<UpdateBlockRequest>,
) -> Result<Json<BlockResponse>, axum::http::StatusCode> {
    let tenant_id_str = claims.organization_id.unwrap_or_default();
    let tenant_id = Uuid::parse_str(&tenant_id_str).map_err(|_| axum::http::StatusCode::UNAUTHORIZED)?;
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
    axum::extract::Extension(claims): axum::extract::Extension<crate::auth::Claims>,
    State(pool): State<PgPool>,
    Path(page_id): Path<Uuid>,
    Json(payload): Json<ReorderBlocksRequest>,
) -> Result<axum::http::StatusCode, axum::http::StatusCode> {
    let tenant_id_str = claims.organization_id.unwrap_or_default();
    let tenant_id = Uuid::parse_str(&tenant_id_str).map_err(|_| axum::http::StatusCode::UNAUTHORIZED)?;
    db::reorder_blocks(&pool, tenant_id, page_id, payload.block_ids)
        .await
        .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(axum::http::StatusCode::OK)
}

async fn publish_site(
    axum::extract::Extension(claims): axum::extract::Extension<crate::auth::Claims>,
    State(pool): State<PgPool>,
    Path(site_id): Path<Uuid>,
) -> Result<axum::http::StatusCode, axum::http::StatusCode> {
    let tenant_id_str = claims.organization_id.unwrap_or_default();
    let tenant_id = Uuid::parse_str(&tenant_id_str).map_err(|_| axum::http::StatusCode::UNAUTHORIZED)?;
    jobs::enqueue_publish_site_job(&pool, tenant_id, site_id)
        .await
        .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(axum::http::StatusCode::ACCEPTED)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_tenant_isolation_builder_api() {
        if std::env::var("DATABASE_URL").is_err() && std::env::var("OHC_DATABASE_URL").is_err() {
            return; // skip if db is unavailable
        }

        let database_url = "postgres://postgres:postgres@localhost:5432/test";
        let pool = sqlx::postgres::PgPoolOptions::new()
            .connect_lazy(database_url)
            .unwrap();

        let claims_invalid = crate::auth::Claims {
            sub: "test".to_string(),
            username: "test".to_string(),
            email: "test@test.com".to_string(),
            roles: vec![],
            organization_id: Some("invalid-uuid".to_string()),
            session_id: None,
            iat: 0,
            exp: 0,
            jti: "".to_string(),
        };

        let res = list_sites(axum::extract::Extension(claims_invalid), State(pool.clone())).await;
        assert!(res.is_err());
        assert_eq!(res.unwrap_err(), axum::http::StatusCode::UNAUTHORIZED);

        let claims_no_org = crate::auth::Claims {
            sub: "test".to_string(),
            username: "test".to_string(),
            email: "test@test.com".to_string(),
            roles: vec![],
            organization_id: None,
            session_id: None,
            iat: 0,
            exp: 0,
            jti: "".to_string(),
        };
        let res_no_org = list_sites(axum::extract::Extension(claims_no_org), State(pool.clone())).await;
        assert!(res_no_org.is_err());
        assert_eq!(res_no_org.unwrap_err(), axum::http::StatusCode::UNAUTHORIZED);
    }
}
