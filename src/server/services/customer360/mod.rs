use std::sync::Arc;
use axum::{
    extract::{Path, State, Query},
    routing::get,
    Json, Router,
};
use serde::{Deserialize, Serialize};
use crate::db::DatabaseConnection;
use crate::api::auth::TenantAuth;

#[derive(Clone)]
pub struct Customer360Service {
    db: Arc<dyn DatabaseConnection>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct InteractionEvent {
    pub id: String,
    pub customer_id: String,
    pub event_type: String,
    pub payload: serde_json::Value,
    pub occurred_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Deserialize)]
pub struct PaginationQuery {
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}

impl Customer360Service {
    pub fn new(db: Arc<dyn DatabaseConnection>) -> Self {
        Self { db }
    }

    pub fn router(&self) -> Router {
        Router::new()
            .route(
                "/:customer_id/interactions",
                get(Self::get_customer_interactions),
            )
            .with_state(self.clone())
    }

    async fn get_customer_interactions(
        State(state): State<Customer360Service>,
        auth: TenantAuth,
        Path(customer_id): Path<String>,
        Query(query): Query<PaginationQuery>,
    ) -> Result<Json<Vec<InteractionEvent>>, axum::http::StatusCode> {
        let tenant_id = auth.tenant_id;

        let pool = state.db.pool();

        let limit = query.limit.unwrap_or(20);
        let offset = query.offset.unwrap_or(0);

        let rows = sqlx::query!(
            r#"
            SELECT id, customer_id, event_type, payload, occurred_at
            FROM interaction_events
            WHERE tenant_id = $1 AND customer_id = $2
            ORDER BY occurred_at DESC
            LIMIT $3 OFFSET $4
            "#,
            tenant_id,
            customer_id,
            limit,
            offset
        )
        .fetch_all(pool)
        .await
        .map_err(|e| {
            tracing::error!("Failed to fetch interaction events: {}", e);
            axum::http::StatusCode::INTERNAL_SERVER_ERROR
        })?;

        let events: Vec<InteractionEvent> = rows
            .into_iter()
            .map(|row| InteractionEvent {
                id: row.id,
                customer_id: row.customer_id,
                event_type: row.event_type,
                payload: row.payload,
                occurred_at: row.occurred_at.unwrap_or_else(chrono::Utc::now),
            })
            .collect();

        Ok(Json(events))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::db_pool::{create_pool, run_migrations};
    use crate::db::DbPool;
    use serde_json::json;
    use std::env;

    async fn setup_db() -> Arc<dyn DatabaseConnection> {
        let db_url = env::var("DATABASE_URL").unwrap_or_else(|_| "postgres://ohc:ohc@localhost:5432/ohc".to_string());
        let pool = create_pool(&db_url).await.unwrap();
        run_migrations(&pool).await.unwrap();

        // Clean up tables
        sqlx::query!("TRUNCATE TABLE interaction_events CASCADE").execute(&pool).await.unwrap();

        Arc::new(DbPool { pool })
    }

    #[tokio::test]
    async fn test_tenant_isolation_and_get_interactions() {
        let db = setup_db().await;
        let pool = db.pool();

        let tenant1 = "t1";
        let tenant2 = "t2";
        let cust1 = "c1";

        // Insert customer
        sqlx::query!(
            "INSERT INTO customers (id, tenant_id, name) VALUES ($1, $2, $3)",
            cust1, tenant1, "Test Customer"
        )
        .execute(pool)
        .await
        .unwrap();

        // Create events for tenant 1
        sqlx::query!(
            "INSERT INTO interaction_events (id, tenant_id, customer_id, event_type, payload) VALUES ($1, $2, $3, $4, $5)",
            "e1", tenant1, cust1, "MESSAGE", json!({"text": "hello t1"})
        )
        .execute(pool)
        .await
        .unwrap();

        // Create events for tenant 2, but same customer_id (should not happen in reality, but tests isolation)
        sqlx::query!(
            "INSERT INTO interaction_events (id, tenant_id, customer_id, event_type, payload) VALUES ($1, $2, $3, $4, $5)",
            "e2", tenant2, cust1, "MESSAGE", json!({"text": "hello t2"})
        )
        .execute(pool)
        .await
        .unwrap();

        let svc = Customer360Service::new(db.clone());

        // Test fetching for tenant 1
        let req = axum::extract::Request::builder()
            .uri(format!("/{}/interactions", cust1))
            .body(axum::body::Body::empty())
            .unwrap();

        let auth = TenantAuth {
            tenant_id: tenant1.to_string(),
            user_id: "u1".to_string(),
            org_id: tenant1.to_string(),
        };

        let res = Customer360Service::get_customer_interactions(
            State(svc.clone()),
            auth.clone(),
            Path(cust1.to_string()),
            Query(PaginationQuery { limit: None, offset: None }),
        ).await.unwrap();

        assert_eq!(res.0.len(), 1);
        assert_eq!(res.0[0].id, "e1");

        // Test fetching for tenant 2
        let auth2 = TenantAuth {
            tenant_id: tenant2.to_string(),
            user_id: "u1".to_string(),
            org_id: tenant2.to_string(),
        };

        let res2 = Customer360Service::get_customer_interactions(
            State(svc.clone()),
            auth2,
            Path(cust1.to_string()),
            Query(PaginationQuery { limit: None, offset: None }),
        ).await.unwrap();

        assert_eq!(res2.0.len(), 1);
        assert_eq!(res2.0[0].id, "e2");
    }
}
