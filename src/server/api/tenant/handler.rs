use axum::{
    extract::{State, Path},
    response::IntoResponse,
    Json,
    routing::{post, get},
    Router,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use crate::db::DB;
use crate::domain::tenant::{Tenant, TenantFlags, TenantAgentAssignment};

#[derive(Serialize, Deserialize)]
pub struct CreateTenantRequest {
    pub business_name: String,
    pub business_type: String,
}

#[derive(Serialize, Deserialize)]
pub struct CreateTenantResponse {
    pub tenant_id: String,
    pub status: String,
}

#[derive(Serialize, Deserialize)]
pub struct GetTenantResponse {
    pub tenant: Tenant,
    pub agents: Vec<TenantAgentAssignment>,
}

pub fn router<S: Clone + Send + Sync + 'static>(db: Arc<DB>) -> Router<S> {
    Router::new()
        .route("/", post(create_tenant))
        .route("/:id", get(get_tenant))
        .with_state(db)
}

async fn create_tenant(
    State(db): State<Arc<DB>>,
    Json(payload): Json<CreateTenantRequest>,
) -> impl IntoResponse {
    let tenant_id = uuid::Uuid::new_v4().to_string();

    // Assign flags based on business type
    let mut flags = TenantFlags::default();
    let lower_type = payload.business_type.to_lowercase();
    if lower_type.contains("service") || lower_type.contains("handyman") || lower_type.contains("tutor") {
        flags.enable_booking = true;
    }
    if lower_type.contains("food") || lower_type.contains("cart") || lower_type.contains("restaurant") {
        flags.enable_menu = true;
    }
    if lower_type.contains("retail") || lower_type.contains("boutique") || lower_type.contains("store") {
        flags.enable_pos = true;
        flags.enable_ecommerce = true;
    }
    if lower_type.contains("bakery") {
        flags.enable_ecommerce = true;
    }

    // Default agents
    let mut agents = vec![
        TenantAgentAssignment {
            tenant_id: tenant_id.clone(),
            agent_id: "Operations".to_string(),
            role: "Operations Manager".to_string(),
        },
        TenantAgentAssignment {
            tenant_id: tenant_id.clone(),
            agent_id: "Marketing".to_string(),
            role: "Marketing Manager".to_string(),
        },
        TenantAgentAssignment {
            tenant_id: tenant_id.clone(),
            agent_id: "Finance".to_string(),
            role: "Finance Manager".to_string(),
        },
        TenantAgentAssignment {
            tenant_id: tenant_id.clone(),
            agent_id: "CustomerSuccess".to_string(),
            role: "Customer Success Manager".to_string(),
        },
        TenantAgentAssignment {
            tenant_id: tenant_id.clone(),
            agent_id: "Legal".to_string(),
            role: "Legal Advisor".to_string(),
        },
        TenantAgentAssignment {
            tenant_id: tenant_id.clone(),
            agent_id: "Advisory".to_string(),
            role: "Business Advisor".to_string(),
        },
    ];

    if flags.enable_booking {
        agents.push(TenantAgentAssignment {
            tenant_id: tenant_id.clone(),
            agent_id: "Salesperson".to_string(),
            role: "Sales Agent".to_string(),
        });
    }

    let flags_json = serde_json::to_value(&flags).unwrap_or(serde_json::json!({}));

    let mut tx = match db.pool.begin().await {
        Ok(tx) => tx,
        Err(e) => return (axum::http::StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    };

    // Set RLS context
    use sqlx::Executor;
    if let Err(e) = tx.execute("SET LOCAL app.current_tenant = 'system'").await {
        let _ = tx.rollback().await;
        return (axum::http::StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response();
    }

    let query1 = sqlx::query("INSERT INTO tenants (id, business_name, business_type, flags) VALUES ($1, $2, $3, $4)")
        .bind(&tenant_id)
        .bind(&payload.business_name)
        .bind(&payload.business_type)
        .bind(&flags_json);

    if let Err(e) = query1.execute(&mut *tx).await {
        let _ = tx.rollback().await;
        return (axum::http::StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response();
    }

    for agent in &agents {
        let query2 = sqlx::query("INSERT INTO tenant_agents (tenant_id, agent_id, role) VALUES ($1, $2, $3)")
            .bind(&tenant_id)
            .bind(&agent.agent_id)
            .bind(&agent.role);

        if let Err(e) = query2.execute(&mut *tx).await {
            let _ = tx.rollback().await;
            return (axum::http::StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response();
        }
    }

    if let Err(e) = tx.commit().await {
        return (axum::http::StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response();
    }

    Json(CreateTenantResponse {
        tenant_id,
        status: "success".to_string(),
    }).into_response()
}

async fn get_tenant(
    State(db): State<Arc<DB>>,
    Path(id): Path<String>,
) -> impl IntoResponse {
    let mut tx = match db.pool.begin().await {
        Ok(tx) => tx,
        Err(e) => return (axum::http::StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    };

    // Set RLS context
    use sqlx::Executor;
    if let Err(e) = tx.execute("SET LOCAL app.current_tenant = 'system'").await {
        let _ = tx.rollback().await;
        return (axum::http::StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response();
    }

    let row = match sqlx::query("SELECT * FROM tenants WHERE id = $1")
        .bind(&id)
        .fetch_optional(&mut *tx)
        .await
    {
        Ok(Some(row)) => row,
        Ok(None) => {
            let _ = tx.rollback().await;
            return (axum::http::StatusCode::NOT_FOUND, "Tenant not found".to_string()).into_response();
        },
        Err(e) => {
            let _ = tx.rollback().await;
            return (axum::http::StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response();
        },
    };

    use sqlx::Row;
    let flags_value: serde_json::Value = row.get("flags");
    let flags: TenantFlags = serde_json::from_value(flags_value).unwrap_or_default();

    let tenant = Tenant {
        id: row.get("id"),
        business_name: row.get("business_name"),
        business_type: row.get("business_type"),
        flags,
        created_at: row.get("created_at"),
        updated_at: row.get("updated_at"),
    };

    let agent_rows = match sqlx::query("SELECT * FROM tenant_agents WHERE tenant_id = $1")
        .bind(&id)
        .fetch_all(&mut *tx)
        .await
    {
        Ok(rows) => rows,
        Err(e) => {
            let _ = tx.rollback().await;
            return (axum::http::StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response();
        },
    };

    let mut agents = Vec::new();
    for row in agent_rows {
        agents.push(TenantAgentAssignment {
            tenant_id: row.get("tenant_id"),
            agent_id: row.get("agent_id"),
            role: row.get("role"),
        });
    }

    let _ = tx.commit().await;

    Json(GetTenantResponse {
        tenant,
        agents,
    }).into_response()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;

    #[tokio::test]
    async fn test_create_tenant_service() {
        if std::env::var("DATABASE_URL").is_err() {
            return;
        }

        let database_url = "postgres://postgres:postgres@localhost:5432/test";
        let pool = sqlx::postgres::PgPoolOptions::new()
            .acquire_timeout(std::time::Duration::from_millis(50))
            .before_acquire(|conn, _meta| { Box::pin(async move { use sqlx::Executor; conn.execute("SET app.current_tenant = 'system'").await?; Ok(true) }) }).connect_lazy(database_url)
            .unwrap();

        let _db = Arc::new(crate::db::DB { pool: pool.clone(), store: crate::db::DbStore::Postgres });
    }
}
