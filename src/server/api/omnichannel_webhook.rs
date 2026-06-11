use axum::{
    extract::{State, Json},
    response::IntoResponse,
    http::StatusCode,
    routing::post,
    Router,
};
use std::sync::Arc;
use serde::{Deserialize, Serialize};
use crate::orchestration::departments::orchestrator::DepartmentOrchestrator;
use uuid::Uuid;
use crate::db::DB;
use sqlx::Row;

#[derive(Deserialize)]
pub struct OmnichannelWebhookPayload {
    pub tenant_id: String,
    pub source: String,
    pub sender_id: String,
    pub message: String,
}

#[derive(Serialize)]
pub struct WebhookResponse {
    pub success: bool,
    pub message_id: Option<String>,
}

#[derive(Clone)]
pub struct OmnichannelWebhookState {
    pub db: Arc<DB>,
    pub orchestrator: Arc<DepartmentOrchestrator>,
}

pub fn router<S>(state: OmnichannelWebhookState) -> Router<S>
where
    S: Clone + Send + Sync + 'static,
{
    Router::new()
        .route("/", post(handle_omnichannel_webhook))
        .with_state(state)
}

pub async fn resolve_identity(db: &Arc<DB>, tenant_id: &str, sender_id: &str, source: &str) -> Result<Option<String>, String> {
    let pool = &db.pool;

    let query = match source {
        "email" | "email_inquiry" | "intake_form" => {
            "SELECT id FROM customers WHERE tenant_id = $1 AND email = $2 LIMIT 1"
        },
        "whatsapp" | "sms" | "twilio" => {
            "SELECT id FROM customers WHERE tenant_id = $1 AND phone = $2 LIMIT 1"
        },
        "instagram" | "facebook" | "meta" => {
            "SELECT id FROM customers WHERE tenant_id = $1 AND preferences->>'social_handle' = $2 LIMIT 1"
        },
        _ => {
            "SELECT id FROM customers WHERE tenant_id = $1 AND (email = $2 OR phone = $2 OR name = $2) LIMIT 1"
        }
    };

    match &db.store {
        crate::db::DbStore::Postgres => {
            match sqlx::query(query)
                .bind(tenant_id)
                .bind(sender_id)
                .fetch_optional(pool)
                .await
            {
                Ok(Some(row)) => {
                    let id: String = row.try_get("id").unwrap_or_default();
                    Ok(Some(id))
                },
                Ok(None) => Ok(None),
                Err(e) => {
                    tracing::error!("Failed to resolve identity (Postgres): {}", e);
                    Err(e.to_string())
                }
            }
        },
        crate::db::DbStore::Sqlite(sqlite_pool) => {
            let sqlite_query_str = query.replace("$1", "?").replace("$2", "?");
            let sqlite_query = match source {
                "instagram" | "facebook" | "meta" => {
                    "SELECT id FROM customers WHERE tenant_id = ? AND json_extract(preferences, '$.social_handle') = ? LIMIT 1"
                },
                _ => sqlite_query_str.as_str(),
            };

            match sqlx::query(sqlite_query)
                .bind(tenant_id)
                .bind(sender_id)
                .fetch_optional(sqlite_pool)
                .await
            {
                Ok(Some(row)) => {
                    let id: String = row.try_get("id").unwrap_or_default();
                    Ok(Some(id))
                },
                Ok(None) => Ok(None),
                Err(e) => {
                    tracing::error!("Failed to resolve identity (SQLite): {}", e);
                    Err(e.to_string())
                }
            }
        }
    }
}

async fn handle_omnichannel_webhook(
    State(state): State<OmnichannelWebhookState>,
    Json(payload): Json<OmnichannelWebhookPayload>,
) -> impl IntoResponse {
    let customer_id = match resolve_identity(&state.db, &payload.tenant_id, &payload.sender_id, &payload.source).await {
        Ok(id) => id,
        Err(e) => {
            tracing::error!("Error resolving identity: {}", e);
            None
        }
    };

    let inbox_id = Uuid::new_v4().to_string();
    let pool = &state.db.pool;

    let insert_result = match &state.db.store {
        crate::db::DbStore::Postgres => {
            sqlx::query(
                "INSERT INTO omni_inbox_messages (id, tenant_id, source, original_content, translated_content, source_language, target_language, status, sender_id, customer_id, created_at) VALUES ($1, $2, $3, $4, $4, NULL, 'English', 'unread', $5, $6, NOW())"
            )
            .bind(&inbox_id)
            .bind(&payload.tenant_id)
            .bind(&payload.source)
            .bind(&payload.message)
            .bind(&payload.sender_id)
            .bind(&customer_id)
            .execute(pool)
            .await.map(|_| ())
        },
        crate::db::DbStore::Sqlite(sqlite_pool) => {
            sqlx::query(
                "INSERT INTO omni_inbox_messages (id, tenant_id, source, original_content, translated_content, source_language, target_language, status, sender_id, customer_id, created_at) VALUES (?, ?, ?, ?, ?, NULL, 'English', 'unread', ?, ?, CURRENT_TIMESTAMP)"
            )
            .bind(&inbox_id)
            .bind(&payload.tenant_id)
            .bind(&payload.source)
            .bind(&payload.message)
            .bind(&payload.message)
            .bind(&payload.sender_id)
            .bind(&customer_id)
            .execute(sqlite_pool)
            .await.map(|_| ())
        }
    };

    if let Err(e) = insert_result {
        tracing::error!("Failed to insert into omni_inbox_messages: {}", e);
        return (StatusCode::INTERNAL_SERVER_ERROR, Json(WebhookResponse { success: false, message_id: None })).into_response();
    }

    // Fallback: also insert into old inbox_messages to preserve compatibility
    let fallback_insert = match &state.db.store {
        crate::db::DbStore::Postgres => {
            sqlx::query(
                "INSERT INTO inbox_messages (id, tenant_id, source, original_content, content, status, sender_id, customer_id, created_at) VALUES ($1, $2, $3, $4, $4, 'unread', $5, $6, NOW())"
            )
            .bind(&inbox_id)
            .bind(&payload.tenant_id)
            .bind(&payload.source)
            .bind(&payload.message)
            .bind(&payload.sender_id)
            .bind(&customer_id)
            .execute(pool)
            .await.map(|_| ())
        },
        crate::db::DbStore::Sqlite(sqlite_pool) => {
            sqlx::query(
                "INSERT INTO inbox_messages (id, tenant_id, source, original_content, content, status, sender_id, customer_id, created_at) VALUES (?, ?, ?, ?, ?, 'unread', ?, ?, CURRENT_TIMESTAMP)"
            )
            .bind(&inbox_id)
            .bind(&payload.tenant_id)
            .bind(&payload.source)
            .bind(&payload.message)
            .bind(&payload.message)
            .bind(&payload.sender_id)
            .bind(&customer_id)
            .execute(sqlite_pool)
            .await.map(|_| ())
        }
    };

    if let Err(e) = fallback_insert {
         tracing::warn!("Failed to insert into fallback inbox_messages: {}", e);
    }

    let event = crate::orchestration::departments::types::DepartmentEvent {
        id: Uuid::new_v4().to_string(),
        tenant_id: payload.tenant_id.clone(),
        event_type: "tenant.omnichannel.message.received".to_string(),
        payload: serde_json::json!({
            "source": payload.source,
            "message": payload.message,
            "sender_id": payload.sender_id,
            "customer_id": customer_id,
            "inbox_message_id": inbox_id,
        }),
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
    use std::sync::Arc;
    use sqlx::sqlite::{SqlitePoolOptions, SqliteConnectOptions};

    #[tokio::test]
    async fn test_resolve_identity_email() {
        let options = SqliteConnectOptions::new().filename(":memory:");
        let pool = SqlitePoolOptions::new().connect_with(options).await.unwrap();

        sqlx::query("CREATE TABLE tenants (id TEXT PRIMARY KEY, name TEXT NOT NULL, industry TEXT, tier TEXT, created_at DATETIME, updated_at DATETIME, version INTEGER)")
            .execute(&pool).await.unwrap();
        sqlx::query("INSERT INTO tenants (id, name) VALUES ('tenant-1', 'Test Tenant')")
            .execute(&pool).await.unwrap();

        sqlx::query(
            "CREATE TABLE customers (
                id TEXT PRIMARY KEY,
                tenant_id TEXT,
                name TEXT,
                email TEXT,
                phone TEXT,
                preferences TEXT
            )"
        ).execute(&pool).await.unwrap();

        sqlx::query("INSERT INTO customers (id, tenant_id, email) VALUES ('cust-1', 'tenant-1', 'test@example.com')")
            .execute(&pool).await.unwrap();

        let db = Arc::new(crate::db::DB {
            pool: pool.clone(),
            store: crate::db::DbStore::Sqlite(pool),
        });

        let id = resolve_identity(&db, "tenant-1", "test@example.com", "email").await.unwrap();
        assert_eq!(id, Some("cust-1".to_string()));

        let not_found = resolve_identity(&db, "tenant-1", "wrong@example.com", "email").await.unwrap();
        assert_eq!(not_found, None);
    }
}
