use axum::{
    extract::{State, Json, Path, Extension},
    routing::{post, get},
    Router,
};
use std::sync::Arc;
use crate::services::catalog::invisible_agent::InvisibleCatalogAgent;

#[derive(serde::Deserialize)]
pub struct ProcessVideoRequest {
    pub video_url: String,
}

#[derive(serde::Serialize)]
pub struct ProcessVideoResponse {
    pub scan_id: String,
}

#[derive(serde::Deserialize)]
pub struct ReviewDraftRequest {
    pub approved: bool,
}

#[derive(serde::Serialize)]
pub struct ReviewDraftResponse {
    pub success: bool,
    pub product_id: Option<String>,
}

#[derive(serde::Serialize)]
pub struct ScanStatusResponse {
    pub id: String,
    pub status: String,
    pub drafts: Vec<DraftItem>,
}

#[derive(serde::Serialize)]
pub struct DraftItem {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub estimated_price_cents: Option<i64>,
    pub image_url: Option<String>,
    pub status: String,
}

pub fn router(agent: Arc<InvisibleCatalogAgent>) -> Router<Arc<dyn ohc_builtin_agent::mesh::transport::MeshTransport>> {
    let r = Router::new()
        .route("/video-scan", post(process_video_scan))
        .route("/video-scan/:scan_id", get(get_scan_status))
        .route("/drafts/:draft_id/review", post(review_draft))
        .with_state(agent);
    Router::new().merge(r)
}

async fn process_video_scan(
    State(agent): State<Arc<InvisibleCatalogAgent>>,
    Extension(claims): Extension<::server_common::Claims>,
    Json(payload): Json<ProcessVideoRequest>,
) -> Result<Json<ProcessVideoResponse>, axum::http::StatusCode> {
    let tenant_id = claims.organization_id.unwrap_or_else(|| "default_tenant".to_string());

    match agent.process_video_scan(&tenant_id, &payload.video_url).await {
        Ok(scan_id) => Ok(Json(ProcessVideoResponse { scan_id })),
        Err(e) => {
            tracing::error!("Error processing video scan: {}", e);
            Err(axum::http::StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

async fn get_scan_status(
    State(agent): State<Arc<InvisibleCatalogAgent>>,
    Path(scan_id): Path<String>,
    Extension(claims): Extension<::server_common::Claims>,
) -> Result<Json<ScanStatusResponse>, axum::http::StatusCode> {
    let tenant_id = claims.organization_id.unwrap_or_else(|| "default_tenant".to_string());

    let mut tx = agent.db.pool.begin().await.map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;
    ::server_common::auth_utils::set_org_context(&mut *tx, &tenant_id).await.map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;

    let scan_res = sqlx::query(
        "SELECT id, status FROM product_video_scans WHERE id = $1 AND tenant_id = $2"
    )
    .bind(scan_id.clone())
    .bind(&tenant_id)
    .fetch_one(&mut *tx)
    .await
    .map_err(|_| axum::http::StatusCode::NOT_FOUND)?;

    let drafts_records = sqlx::query(
        "SELECT id, name, description, estimated_price_cents, image_url, status FROM draft_catalog_items WHERE scan_id = $1 AND tenant_id = $2"
    )
    .bind(scan_id)
    .bind(&tenant_id)
    .fetch_all(&mut *tx)
    .await
    .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;

    tx.commit().await.map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;

    use sqlx::Row;
    let mut drafts = Vec::new();
    for r in drafts_records {
        drafts.push(DraftItem {
            id: r.get("id"),
            name: r.get("name"),
            description: r.get("description"),
            estimated_price_cents: r.get("estimated_price_cents"),
            image_url: r.get("image_url"),
            status: r.get("status"),
        });
    }

    Ok(Json(ScanStatusResponse {
        id: scan_res.get("id"),
        status: scan_res.get("status"),
        drafts,
    }))
}

async fn review_draft(
    State(agent): State<Arc<InvisibleCatalogAgent>>,
    Path(draft_id): Path<String>,
    Extension(claims): Extension<::server_common::Claims>,
    Json(payload): Json<ReviewDraftRequest>,
) -> Result<Json<ReviewDraftResponse>, axum::http::StatusCode> {
    let tenant_id = claims.organization_id.unwrap_or_else(|| "default_tenant".to_string());

    match agent.review_draft_item(&tenant_id, &draft_id, payload.approved).await {
        Ok(product_id) => Ok(Json(ReviewDraftResponse {
            success: true,
            product_id,
        })),
        Err(e) => {
            tracing::error!("Error reviewing draft: {}", e);
            Err(axum::http::StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}
