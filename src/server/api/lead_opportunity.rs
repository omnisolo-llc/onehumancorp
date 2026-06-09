use axum::{extract::{State, Query}, Json, routing::{get, post, put, delete}, Router};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::RwLock;

#[derive(Serialize, Deserialize, Debug)]
pub struct Lead {
    pub id: String,
    pub tenant_id: String,
    pub source: String,
    pub contact_info: Option<String>,
    pub context: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Opportunity {
    pub id: String,
    pub tenant_id: String,
    pub lead_id: Option<String>,
    pub title: String,
    pub stage: String,
    pub estimated_value_cents: i64,
    pub priority: String,
}

#[derive(Deserialize)]
pub struct TenantQuery {
    pub tenant_id: String,
}

pub async fn list_leads(
    State(db): State<Arc<sqlx::PgPool>>,
    Query(query): Query<TenantQuery>,
) -> Json<Vec<Lead>> {
    let mut tx = db.begin().await.unwrap();
    sqlx::query("SET LOCAL app.current_tenant = $1")
        .bind(&query.tenant_id)
        .execute(&mut *tx)
        .await
        .unwrap();

    let leads = sqlx::query_as!(
        Lead,
        "SELECT id, tenant_id, source, contact_info, context FROM leads WHERE tenant_id = $1 ORDER BY created_at DESC",
        query.tenant_id
    )
    .fetch_all(&mut *tx)
    .await
    .unwrap_or_default();

    tx.commit().await.unwrap();
    Json(leads)
}

pub async fn list_opportunities(
    State(db): State<Arc<sqlx::PgPool>>,
    Query(query): Query<TenantQuery>,
) -> Json<Vec<Opportunity>> {
    let mut tx = db.begin().await.unwrap();
    sqlx::query("SET LOCAL app.current_tenant = $1")
        .bind(&query.tenant_id)
        .execute(&mut *tx)
        .await
        .unwrap();

    let opportunities = sqlx::query_as!(
        Opportunity,
        "SELECT id, tenant_id, lead_id, title, stage, estimated_value_cents, priority FROM opportunities WHERE tenant_id = $1 ORDER BY created_at DESC",
        query.tenant_id
    )
    .fetch_all(&mut *tx)
    .await
    .unwrap_or_default();

    tx.commit().await.unwrap();
    Json(opportunities)
}

pub async fn update_opportunity_stage(
    State(db): State<Arc<sqlx::PgPool>>,
    Query(query): Query<TenantQuery>,
    Json(payload): Json<serde_json::Value>,
) -> Json<serde_json::Value> {
    let id = payload.get("id").unwrap().as_str().unwrap();
    let stage = payload.get("stage").unwrap().as_str().unwrap();

    let mut tx = db.begin().await.unwrap();
    sqlx::query("SET LOCAL app.current_tenant = $1")
        .bind(&query.tenant_id)
        .execute(&mut *tx)
        .await
        .unwrap();

    let _ = sqlx::query("UPDATE opportunities SET stage = $1, updated_at = NOW() WHERE id = $2 AND tenant_id = $3")
        .bind(stage)
        .bind(id)
        .bind(&query.tenant_id)
        .execute(&mut *tx)
        .await
        .unwrap();

    tx.commit().await.unwrap();
    Json(serde_json::json!({"success": true}))
}

pub fn router(db: Arc<sqlx::PgPool>) -> Router {
    Router::new()
        .route("/api/leads", get(list_leads).with_state(db.clone()))
        .route("/api/opportunities", get(list_opportunities).with_state(db.clone()))
        .route("/api/opportunities/stage", put(update_opportunity_stage).with_state(db.clone()))
}
