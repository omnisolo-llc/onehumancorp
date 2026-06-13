use axum::{
    extract::{State, Json},
    response::IntoResponse,
    http::StatusCode,
    routing::post,
    Router,
};
use std::sync::Arc;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::orchestration::departments::orchestrator::DepartmentOrchestrator;

#[derive(Deserialize, Debug, Clone)]
pub struct OmnichannelPayload {
    pub tenant_id: String,
    pub channel: String,
    pub sender_id: String,
    pub message: String,
}

#[derive(Serialize)]
pub struct WebhookResponse {
    pub success: bool,
    pub message_id: Option<String>,
}

#[derive(Clone)]
pub struct AppState {
    pub orchestrator: Arc<DepartmentOrchestrator>,
    pub db: Arc<crate::db::DB>,
}

pub async fn resolve_identity(db: &crate::db::DB, tenant_id: &str, channel: &str, sender_id: &str) -> Option<String> {
    let pool = &db.pool;

    // 1. Check if identity exists in customer_identities
    let existing_identity: Option<String> = match &db.store {
        crate::db::DbStore::Postgres => {
            sqlx::query_scalar("SELECT customer_id FROM customer_identities WHERE tenant_id = $1 AND channel = $2 AND channel_identity = $3")
                .bind(tenant_id)
                .bind(channel)
                .bind(sender_id)
                .fetch_optional(pool)
                .await
                .unwrap_or(None)
        },
        crate::db::DbStore::Sqlite(_) => {
            sqlx::query_scalar("SELECT customer_id FROM customer_identities WHERE tenant_id = ? AND channel = ? AND channel_identity = ?")
                .bind(tenant_id)
                .bind(channel)
                .bind(sender_id)
                .fetch_optional(pool)
                .await
                .unwrap_or(None)
        }
    };

    if let Some(customer_id) = existing_identity {
        return Some(customer_id);
    }

    // 2. Otherwise create a new anonymous customer
    let new_customer_id = Uuid::new_v4().to_string();
    let display_name = format!("Anonymous ({} {})", channel, &sender_id[0..std::cmp::min(4, sender_id.len())]);
    let new_identity_id = Uuid::new_v4().to_string();

    let res = match &db.store {
        crate::db::DbStore::Postgres => {
            let mut tx = pool.begin().await.ok()?;
            let _ = sqlx::query("INSERT INTO customers (id, tenant_id, name) VALUES ($1, $2, $3)")
                .bind(&new_customer_id)
                .bind(tenant_id)
                .bind(&display_name)
                .execute(&mut *tx)
                .await;

            let _ = sqlx::query("INSERT INTO customer_identities (id, tenant_id, customer_id, channel, channel_identity) VALUES ($1, $2, $3, $4, $5)")
                .bind(&new_identity_id)
                .bind(tenant_id)
                .bind(&new_customer_id)
                .bind(channel)
                .bind(sender_id)
                .execute(&mut *tx)
                .await;

            tx.commit().await
        },
        crate::db::DbStore::Sqlite(_) => {
            let mut tx = pool.begin().await.ok()?;
            let _ = sqlx::query("INSERT INTO customers (id, tenant_id, name) VALUES (?, ?, ?)")
                .bind(&new_customer_id)
                .bind(tenant_id)
                .bind(&display_name)
                .execute(&mut *tx)
                .await;

            let _ = sqlx::query("INSERT INTO customer_identities (id, tenant_id, customer_id, channel, channel_identity) VALUES (?, ?, ?, ?, ?)")
                .bind(&new_identity_id)
                .bind(tenant_id)
                .bind(&new_customer_id)
                .bind(channel)
                .bind(sender_id)
                .execute(&mut *tx)
                .await;

            tx.commit().await
        }
    };

    if res.is_ok() {
        Some(new_customer_id)
    } else {
        None
    }
}

pub fn router(state: AppState) -> Router {
    Router::new()
        .route("/api/v1/omnichannel/webhook", post(handle_omnichannel_webhook))
        .with_state(state)
}

pub async fn handle_omnichannel_webhook(
    State(state): State<AppState>,
    Json(payload): Json<OmnichannelPayload>,
) -> impl IntoResponse {

    // Resolve identity
    let customer_id_opt = resolve_identity(&state.db, &payload.tenant_id, &payload.channel, &payload.sender_id).await;

    let customer_id = customer_id_opt.unwrap_or_else(|| "unknown".to_string());

    let inbox_id = Uuid::new_v4().to_string();
    let tenant_id = payload.tenant_id.clone();

    // Save to inbox_messages
    let mut payload_json = serde_json::to_value(&payload).unwrap_or(serde_json::json!({}));
    if let Some(obj) = payload_json.as_object_mut() {
        obj.insert("customer_id".to_string(), serde_json::Value::String(customer_id.clone()));
    }

    let insert_res = match &state.db.store {
        crate::db::DbStore::Postgres => {
            sqlx::query("INSERT INTO inbox_messages (id, tenant_id, customer_id, channel, message, payload, status) VALUES ($1, $2, $3, $4, $5, $6, 'NEW')")
                .bind(&inbox_id)
                .bind(&tenant_id)
                .bind(&customer_id)
                .bind(&payload.channel)
                .bind(&payload.message)
                .bind(&payload_json)
                .execute(&state.db.pool)
                .await
        },
        crate::db::DbStore::Sqlite(_) => {
            sqlx::query("INSERT INTO inbox_messages (id, tenant_id, customer_id, channel, message, payload, status) VALUES (?, ?, ?, ?, ?, ?, 'NEW')")
                .bind(&inbox_id)
                .bind(&tenant_id)
                .bind(&customer_id)
                .bind(&payload.channel)
                .bind(&payload.message)
                .bind(&payload_json)
                .execute(&state.db.pool)
                .await
        }
    };

    if let Err(e) = insert_res {
        tracing::error!("Failed to persist omnichannel message: {}", e);
        return (StatusCode::INTERNAL_SERVER_ERROR, Json(WebhookResponse { success: false, message_id: None })).into_response();
    }

    // Trigger triage evaluation via queue to offload heavy sync
    let triage_job_id = Uuid::new_v4().to_string();
    let enqueue_res = match &state.db.store {
        crate::db::DbStore::Postgres => {
            sqlx::query("INSERT INTO ohc_job_queue (id, tenant_id, job_type, payload, status) VALUES ($1, $2, 'message_triage', $3, 'PENDING')")
                .bind(&triage_job_id)
                .bind(&tenant_id)
                .bind(serde_json::json!({ "message_id": inbox_id }))
                .execute(&state.db.pool)
                .await
        },
        crate::db::DbStore::Sqlite(_) => {
            sqlx::query("INSERT INTO ohc_job_queue (id, tenant_id, job_type, payload, status) VALUES (?, ?, 'message_triage', ?, 'PENDING')")
                .bind(&triage_job_id)
                .bind(&tenant_id)
                .bind(serde_json::json!({ "message_id": inbox_id }))
                .execute(&state.db.pool)
                .await
        }
    };

    if let Err(e) = enqueue_res {
        tracing::error!("Failed to enqueue message_triage job: {}", e);
    }

    let event = crate::orchestration::departments::types::DepartmentEvent {
        id: Uuid::new_v4().to_string(),
        tenant_id: tenant_id.clone(),
        event_type: "tenant.omnichannel.message.received".to_string(),
        payload: payload_json,
    };

    let orchestrator_clone = state.orchestrator.clone();
    tokio::spawn(async move {
        let _ = orchestrator_clone.dispatch_event(event).await;
    });

    (StatusCode::OK, Json(WebhookResponse { success: true, message_id: Some(inbox_id) })).into_response()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::DB;
    use sqlx::SqlitePool;

    #[tokio::test]
    async fn test_resolve_identity() {
        // We use sqlite in memory for tests
        let pool = SqlitePool::connect("sqlite::memory:").await.unwrap();

        let schema = r#"
            CREATE TABLE customers (
                id TEXT PRIMARY KEY,
                tenant_id TEXT NOT NULL,
                name TEXT NOT NULL,
                email TEXT,
                phone TEXT
            );
            CREATE TABLE customer_identities (
                id TEXT PRIMARY KEY,
                tenant_id TEXT NOT NULL,
                customer_id TEXT NOT NULL,
                channel TEXT NOT NULL,
                channel_identity TEXT NOT NULL
            );
        "#;

        sqlx::query(schema).execute(&pool).await.unwrap();

        let db = DB {
            pool: pool.clone(),
            store: crate::db::DbStore::Sqlite(pool.clone()),
        };

        // 1. Resolve unknown identity (should create new)
        let cust1 = resolve_identity(&db, "t1", "ig_dm", "user123").await;
        assert!(cust1.is_some());
        let c1_id = cust1.unwrap();

        // Verify inserted
        let count: i32 = sqlx::query_scalar("SELECT count(*) FROM customers").fetch_one(&pool).await.unwrap();
        assert_eq!(count, 1);

        // 2. Resolve same identity (should return existing)
        let cust2 = resolve_identity(&db, "t1", "ig_dm", "user123").await;
        assert_eq!(cust2.unwrap(), c1_id);

        let count2: i32 = sqlx::query_scalar("SELECT count(*) FROM customers").fetch_one(&pool).await.unwrap();
        assert_eq!(count2, 1);
    }
}
