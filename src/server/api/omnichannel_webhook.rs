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
    #[serde(alias = "source")]
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

pub async fn resolve_identity(db: &crate::db::DB, tenant_id: &str, channel: &str, sender_id: &str) -> String {
    let pool = &db.pool;

    // 1. Check if identity exists in customer_identities
    let existing_identity: Option<String> = match &db.store {
        crate::db::DbStore::Postgres => {
            sqlx::query_scalar("SELECT customer_id FROM customer_identities WHERE tenant_id = $1 AND channel = $2 AND channel_identity = $3")
                .bind(tenant_id)
                .bind(channel)
                .bind(sender_id)
                .fetch_optional(pool)
                .await.ok().flatten()
        },
        crate::db::DbStore::Sqlite(sqlite_pool) => {
            sqlx::query_scalar("SELECT customer_id FROM customer_identities WHERE tenant_id = ? AND channel = ? AND channel_identity = ?")
                .bind(tenant_id)
                .bind(channel)
                .bind(sender_id)
                .fetch_optional(sqlite_pool)
                .await.ok().flatten()
        }
    };

    if let Some(id) = existing_identity {
        return id;
    }

    // 2. If not found, try to resolve by phone or email in customers table (basic resolution)
    // Assume sender_id might be a phone number or email depending on channel
    let potential_customer_id: Option<String> = match &db.store {
        crate::db::DbStore::Postgres => {
             sqlx::query_scalar("SELECT id FROM customers WHERE tenant_id = $1 AND (phone = $2 OR email = $2) LIMIT 1")
                .bind(tenant_id)
                .bind(sender_id)
                .fetch_optional(pool)
                .await.ok().flatten()
        },
        crate::db::DbStore::Sqlite(sqlite_pool) => {
             sqlx::query_scalar("SELECT id FROM customers WHERE tenant_id = ? AND (phone = ? OR email = ?) LIMIT 1")
                .bind(tenant_id)
                .bind(sender_id)
                .bind(sender_id)
                .fetch_optional(sqlite_pool)
                .await.ok().flatten()
        }
    };

    let id = if let Some(found_id) = potential_customer_id {
        found_id
    } else {
        let new_id = Uuid::new_v4().to_string();
        let email = if sender_id.contains('@') { sender_id } else { "" };
        let phone = if !sender_id.contains('@') && sender_id.chars().any(|c| c.is_digit(10)) { sender_id } else { "" };
        let name = "Unknown Customer";

        match &db.store {
            crate::db::DbStore::Postgres => {
                let _ = sqlx::query("INSERT INTO customers (id, tenant_id, name, email, phone) VALUES ($1, $2, $3, $4, $5)")
                    .bind(&new_id)
                    .bind(tenant_id)
                    .bind(name)
                    .bind(if email.is_empty() { None } else { Some(email) })
                    .bind(if phone.is_empty() { None } else { Some(phone) })
                    .execute(pool)
                    .await;
            },
            crate::db::DbStore::Sqlite(sqlite_pool) => {
                let _ = sqlx::query("INSERT INTO customers (id, tenant_id, name, email, phone) VALUES (?, ?, ?, ?, ?)")
                    .bind(&new_id)
                    .bind(tenant_id)
                    .bind(name)
                    .bind(if email.is_empty() { None } else { Some(email) })
                    .bind(if phone.is_empty() { None } else { Some(phone) })
                    .execute(sqlite_pool)
                    .await;
            }
        };
        new_id
    };

    // Cache this new identity link
    let identity_id = Uuid::new_v4().to_string();
    match &db.store {
        crate::db::DbStore::Postgres => {
            let _ = sqlx::query("INSERT INTO customer_identities (id, tenant_id, customer_id, channel, channel_identity) VALUES ($1, $2, $3, $4, $5) ON CONFLICT DO NOTHING")
                .bind(&identity_id)
                .bind(tenant_id)
                .bind(&id)
                .bind(channel)
                .bind(sender_id)
                .execute(pool)
                .await;
        },
        crate::db::DbStore::Sqlite(sqlite_pool) => {
            let _ = sqlx::query("INSERT OR IGNORE INTO customer_identities (id, tenant_id, customer_id, channel, channel_identity) VALUES (?, ?, ?, ?, ?)")
                .bind(&identity_id)
                .bind(tenant_id)
                .bind(&id)
                .bind(channel)
                .bind(sender_id)
                .execute(sqlite_pool)
                .await;
        }
    };

    id
}

pub fn router(state: AppState) -> Router {
    Router::new()
        .route("/", post(handle_omnichannel_webhook))
        .with_state(state)
}

pub async fn handle_omnichannel_webhook(
    State(state): State<AppState>,
    Json(payload): Json<OmnichannelPayload>,
) -> impl IntoResponse {
    let tenant_id = &payload.tenant_id;
    let channel = &payload.channel;
    let sender_id = &payload.sender_id;
    let message = &payload.message;

    // 1. Resolve Identity
    let customer_id = resolve_identity(&state.db, tenant_id, channel, sender_id).await;

    // Create ServiceLead if applicable
    let service_lead_id = uuid::Uuid::new_v4().to_string();
    if channel == "intake_form" || channel == "email_inquiry" || channel == "work_intake" || channel == "instagram_dm" || channel == "sms" || channel == "booking_form" {
        let _ = match &state.db.store {
            crate::db::DbStore::Postgres => {
                let _ = sqlx::query("INSERT INTO service_leads (id, tenant_id, customer_id, description, source, status, created_at, updated_at) VALUES ($1, $2, $3, $4, $5, 'new', NOW(), NOW())")
                    .bind(&service_lead_id)
                    .bind(tenant_id)
                    .bind(uuid::Uuid::parse_str(&customer_id).ok())
                    .bind(message)
                    .bind(channel)
                    .execute(&state.db.pool)
                    .await;
            },
            crate::db::DbStore::Sqlite(sqlite_pool) => {
                let _ = sqlx::query("INSERT INTO service_leads (id, tenant_id, customer_id, description, source, status, created_at, updated_at) VALUES (?, ?, ?, ?, ?, 'new', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP)")
                    .bind(&service_lead_id)
                    .bind(tenant_id)
                    .bind(uuid::Uuid::parse_str(&customer_id).ok().map(|u| u.to_string()))
                    .bind(message)
                    .bind(channel)
                    .execute(sqlite_pool)
                    .await;
            }
        };
    }

    // 2. Persist Message into inbox_messages
    let inbox_id = Uuid::new_v4().to_string();
    let intent_id = Uuid::new_v4().to_string();
    let _ = match &state.db.store {
        crate::db::DbStore::Postgres => sqlx::query("INSERT INTO work_intents (id, tenant_id, source, intent_type, payload, status, created_at, updated_at) VALUES ($1, $2, $3, $4, $5, $6, NOW(), NOW())")
            .bind(&intent_id)
            .bind(tenant_id)
            .bind(channel)
            .bind("customer_inquiry")
            .bind(serde_json::json!({"message": message, "sender_id": sender_id, "customer_id": customer_id}))
            .bind("PENDING")
            .execute(&state.db.pool).await.map(|_| ()).map_err(|e| e),
        crate::db::DbStore::Sqlite(sqlite_pool) => sqlx::query("INSERT INTO work_intents (id, tenant_id, source, intent_type, payload, status, created_at, updated_at) VALUES (?, ?, ?, ?, ?, ?, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP)")
            .bind(&intent_id)
            .bind(tenant_id)
            .bind(channel)
            .bind("customer_inquiry")
            .bind(serde_json::json!({"message": message, "sender_id": sender_id, "customer_id": customer_id}).to_string())
            .bind("PENDING")
            .execute(sqlite_pool).await.map(|_| ()).map_err(|e| e),
    };

    let insert_result = match &state.db.store {
        crate::db::DbStore::Postgres => {
            let res = sqlx::query(
                "INSERT INTO inbox_messages (id, tenant_id, source, original_content, content, draft_reply, status, sender_id, created_at) VALUES ($1, $2, $3, $4, $5, '', 'unread', $6, NOW())"
            )
            .bind(&inbox_id)
            .bind(tenant_id)
            .bind(channel)
            .bind(message)
            .bind(message)
            .bind(sender_id)
            .execute(&state.db.pool)
            .await;

            if res.is_ok() {
                if let Err(e) = sqlx::query(
                    "INSERT INTO omni_inbox_messages (id, tenant_id, source, original_content, translated_content, target_language, draft_reply, status, sender_id, customer_id, created_at) VALUES ($1, $2, $3, $4, $5, 'English', '', 'unread', $6, $7, NOW())"
                )
                .bind(&inbox_id)
                .bind(tenant_id)
                .bind(channel)
                .bind(message)
                .bind(message)
                .bind(sender_id)
                .bind(&customer_id)
                .execute(&state.db.pool)
                .await {
                    tracing::error!("Failed to insert omni_inbox_messages: {}", e);
                }
            }
            res.map(|_| ())
        },
        crate::db::DbStore::Sqlite(sqlite_pool) => {
            let res = sqlx::query(
                "INSERT INTO inbox_messages (id, tenant_id, source, original_content, content, draft_reply, status, sender_id, created_at) VALUES (?, ?, ?, ?, ?, '', 'unread', ?, CURRENT_TIMESTAMP)"
            )
            .bind(&inbox_id)
            .bind(tenant_id)
            .bind(channel)
            .bind(message)
            .bind(message)
            .bind(sender_id)
            .execute(sqlite_pool)
            .await;

            if res.is_ok() {
                if let Err(e) = sqlx::query(
                    "INSERT INTO omni_inbox_messages (id, tenant_id, source, original_content, translated_content, target_language, draft_reply, status, sender_id, customer_id, created_at) VALUES (?, ?, ?, ?, ?, 'English', '', 'unread', ?, ?, CURRENT_TIMESTAMP)"
                )
                .bind(&inbox_id)
                .bind(tenant_id)
                .bind(channel)
                .bind(message)
                .bind(message)
                .bind(sender_id)
                .bind(&customer_id)
                .execute(sqlite_pool)
                .await {
                    tracing::error!("Failed to insert omni_inbox_messages (SQLite): {}", e);
                }
            }
            res.map(|_| ())
        }
    };

    if let Err(e) = insert_result {
        tracing::error!("Failed to insert omnichannel inbox message: {}", e);
        return (StatusCode::INTERNAL_SERVER_ERROR, Json(WebhookResponse { success: false, message_id: None })).into_response();
    }

    // 3. Enqueue message_triage job
    let job_id = Uuid::new_v4().to_string();
    let payload_json = serde_json::json!({
        "message_id": inbox_id,
        "source": channel,
        "content": message,
        "sender_id": sender_id,
        "customer_id": customer_id,
        "service_lead_id": service_lead_id,
        "message": message
    });

    let enqueue_result = match &state.db.store {
        crate::db::DbStore::Postgres => {
            sqlx::query("INSERT INTO ohc_job_queue (id, tenant_id, job_type, payload, status) VALUES ($1, $2, 'message_triage', $3, 'PENDING')")
                .bind(&job_id)
                .bind(tenant_id)
                .bind(payload_json.to_string())
                .execute(&state.db.pool)
                .await
                .map(|_| ())
        },
        crate::db::DbStore::Sqlite(sqlite_pool) => {
            sqlx::query("INSERT INTO ohc_job_queue (id, tenant_id, job_type, payload, status) VALUES (?, ?, 'message_triage', ?, 'PENDING')")
                .bind(&job_id)
                .bind(tenant_id)
                .bind(payload_json.to_string())
                .execute(sqlite_pool)
                .await
                .map(|_| ())
        }
    };

    if let Err(e) = enqueue_result {
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
                channel_identity TEXT NOT NULL,
                UNIQUE(tenant_id, channel, channel_identity)
            );
        "#;
        sqlx::query(schema).execute(&pool).await.unwrap();

        let db = DB {
            pool: sqlx::PgPool::connect_lazy("postgres://dummy").unwrap(), // Dummy pg pool, won't be used since store is Sqlite
            store: crate::db::DbStore::Sqlite(pool.clone()),
        };

        // 1. Insert a test customer
        sqlx::query("INSERT INTO customers (id, tenant_id, name, email, phone) VALUES ('cust-1', 'tenant-1', 'Test User', 'test@example.com', '+1234567890')")
            .execute(&pool)
            .await
            .unwrap();

        // 2. Test resolution by email
        let resolved_id = resolve_identity(&db, "tenant-1", "email", "test@example.com").await;
        assert_eq!(resolved_id, "cust-1".to_string());

        // 3. Test that it was cached in customer_identities
        let cached_id: String = sqlx::query_scalar("SELECT customer_id FROM customer_identities WHERE tenant_id = 'tenant-1' AND channel = 'email' AND channel_identity = 'test@example.com'")
            .fetch_one(&pool)
            .await
            .unwrap();
        assert_eq!(cached_id, "cust-1");

        // 4. Test resolution by phone
        let resolved_id2 = resolve_identity(&db, "tenant-1", "whatsapp", "+1234567890").await;
        assert_eq!(resolved_id2, "cust-1".to_string());

        // 5. Test resolution from cache directly (another call to same email)
        let resolved_id3 = resolve_identity(&db, "tenant-1", "email", "test@example.com").await;
        assert_eq!(resolved_id3, "cust-1".to_string());

        // 6. Test unknown identity creates a new customer and returns id
        let resolved_id4 = resolve_identity(&db, "tenant-1", "instagram", "unknown_handle").await;
        assert!(!resolved_id4.is_empty());
        assert_ne!(resolved_id4, "cust-1");
    }
}
