use sqlx::{PgPool, Row};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CustomerProfileSummary {
    pub total_interactions: i64,
    pub last_interaction: Option<DateTime<Utc>>,
    pub segments: Vec<String>,
    pub preferences: Vec<String>,
    pub summary: String,
}

pub struct CustomerMemoryGraphService {
    pool: PgPool,
}

impl CustomerMemoryGraphService {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn ingest_interaction(&self, tenant_id: &str, customer_id: &str, channel: &str, raw_content: &str) -> Result<Uuid, sqlx::Error> {
        let event_id = Uuid::new_v4();

        let mut tx = self.pool.begin().await?;
        ::server_common::auth_utils::set_org_context(&mut *tx, tenant_id).await?;

        if sqlx::query("SELECT 1 FROM customers WHERE id = $1 AND tenant_id = $2 FOR SHARE")
            .bind(customer_id)
            .bind(tenant_id)
            .fetch_optional(&mut *tx)
            .await?
            .is_none()
        {
            return Err(sqlx::Error::RowNotFound);
        }

        sqlx::query(
            "INSERT INTO interaction_events (id, tenant_id, customer_id, channel, raw_content) VALUES ($1, $2, $3, $4, $5)"
        )
        .bind(event_id)
        .bind(tenant_id)
        .bind(customer_id)
        .bind(channel)
        .bind(raw_content)
        .execute(&mut *tx)
        .await?;

        // Enqueue background job for AI processing
        let job_id = Uuid::new_v4();
        sqlx::query(
            "INSERT INTO interaction_event_jobs (job_id, tenant_id, interaction_event_id, status) VALUES ($1, $2, $3, 'pending')"
        )
        .bind(job_id)
        .bind(tenant_id)
        .bind(event_id)
        .execute(&mut *tx)
        .await?;

        tx.commit().await?;
        Ok(event_id)
    }

    pub async fn get_profile_summary(&self, tenant_id: &str, customer_id: &str) -> Result<CustomerProfileSummary, sqlx::Error> {
        let mut tx = self.pool.begin().await?;
        ::server_common::auth_utils::set_org_context(&mut *tx, tenant_id).await?;

        let record = sqlx::query(
            "SELECT profile_summary FROM customers WHERE id = $1 AND tenant_id = $2"
        )
        .bind(customer_id)
        .bind(tenant_id)
        .fetch_optional(&mut *tx)
        .await?;

        let summary = if let Some(row) = record {
            if let Ok(val) = row.try_get::<sqlx::types::Json<CustomerProfileSummary>, _>("profile_summary") {
                val.0
            } else {
                CustomerProfileSummary {
                    total_interactions: 0,
                    last_interaction: None,
                    segments: vec![],
                    preferences: vec![],
                    summary: "No summary available.".to_string(),
                }
            }
        } else {
            CustomerProfileSummary {
                total_interactions: 0,
                last_interaction: None,
                segments: vec![],
                preferences: vec![],
                summary: "Customer not found.".to_string(),
            }
        };

        tx.commit().await?;
        Ok(summary)
    }

}
#[cfg(test)]
mod tests {
    use super::*;
    use sqlx::postgres::PgPoolOptions;

    async fn isolated_postgres_pool() -> Option<(PgPool, PgPool, String, String)> {
        let database_url = std::env::var("OHC_TEST_POSTGRES_URL")
            .ok()
            .or_else(|| std::env::var("OHC_DATABASE_URL").ok())?;
        if !database_url.starts_with("postgres") {
            return None;
        }
        let admin = PgPoolOptions::new()
            .max_connections(1)
            .connect(&database_url)
            .await
            .ok()?;
        let suffix = Uuid::new_v4().simple().to_string();
        let schema = format!("memory_test_{suffix}");
        let role = format!("memory_role_{suffix}");
        let password = format!("memory_password_{suffix}");
        sqlx::query(&format!("CREATE ROLE {role} LOGIN PASSWORD '{password}'"))
            .execute(&admin)
            .await
            .ok()?;
        sqlx::query(&format!("CREATE SCHEMA {schema} AUTHORIZATION {role}"))
            .execute(&admin)
            .await
            .ok()?;
        let search_path = format!("SET search_path TO {schema}, public");
        let connection_options = database_url
            .parse::<sqlx::postgres::PgConnectOptions>()
            .ok()?
            .username(&role)
            .password(&password);
        let pool = PgPoolOptions::new()
            .max_connections(3)
            .after_connect(move |connection, _| {
                let search_path = search_path.clone();
                Box::pin(async move {
                    sqlx::query(&search_path).execute(connection).await?;
                    Ok(())
                })
            })
            .connect_with(connection_options)
            .await
            .ok()?;
        Some((admin, pool, schema, role))
    }

    #[tokio::test]
    async fn postgres_ingest_and_summary_are_tenant_scoped() {
        let Some((admin, pool, schema, role)) = isolated_postgres_pool().await else {
            return;
        };
        sqlx::query(
            "CREATE TABLE customers (
                id TEXT NOT NULL,
                tenant_id TEXT NOT NULL,
                profile_summary JSONB,
                PRIMARY KEY (id, tenant_id)
             )",
        )
        .execute(&pool)
        .await
        .unwrap();
        sqlx::query(
            "CREATE TABLE interaction_events (
                id UUID PRIMARY KEY,
                tenant_id TEXT NOT NULL,
                customer_id TEXT NOT NULL,
                channel TEXT NOT NULL,
                raw_content TEXT NOT NULL
             )",
        )
        .execute(&pool)
        .await
        .unwrap();
        sqlx::query(
            "CREATE TABLE interaction_event_jobs (
                job_id UUID PRIMARY KEY,
                tenant_id TEXT NOT NULL,
                interaction_event_id UUID NOT NULL,
                status TEXT NOT NULL
             )",
        )
        .execute(&pool)
        .await
        .unwrap();
        for table in ["customers", "interaction_events", "interaction_event_jobs"] {
            for statement in [
                format!("ALTER TABLE {table} ENABLE ROW LEVEL SECURITY"),
                format!("ALTER TABLE {table} FORCE ROW LEVEL SECURITY"),
                format!(
                    "CREATE POLICY tenant_isolation ON {table}
                     USING (tenant_id = current_setting('app.current_tenant', true))
                     WITH CHECK (tenant_id = current_setting('app.current_tenant', true))"
                ),
            ] {
                sqlx::query(&statement).execute(&pool).await.unwrap();
            }
        }

        let summary_a = CustomerProfileSummary {
            total_interactions: 2,
            last_interaction: None,
            segments: vec!["tenant-a".to_string()],
            preferences: vec![],
            summary: "summary-a".to_string(),
        };
        let summary_b = CustomerProfileSummary {
            total_interactions: 7,
            last_interaction: None,
            segments: vec!["tenant-b".to_string()],
            preferences: vec![],
            summary: "summary-b".to_string(),
        };
        for (tenant_id, customer_id, summary) in [
            ("tenant-a", "shared-customer", &summary_a),
            ("tenant-b", "shared-customer", &summary_b),
            ("tenant-b", "tenant-b-only", &summary_b),
        ] {
            let mut tx = pool.begin().await.unwrap();
            ::server_common::auth_utils::set_org_context(&mut *tx, tenant_id)
                .await
                .unwrap();
            sqlx::query(
                "INSERT INTO customers (id, tenant_id, profile_summary) VALUES ($1, $2, $3)",
            )
            .bind(customer_id)
            .bind(tenant_id)
            .bind(sqlx::types::Json(summary))
            .execute(&mut *tx)
            .await
            .unwrap();
            tx.commit().await.unwrap();
        }

        let service = CustomerMemoryGraphService::new(pool.clone());
        assert!(matches!(
            service
                .ingest_interaction("tenant-a", "tenant-b-only", "email", "private")
                .await,
            Err(sqlx::Error::RowNotFound)
        ));
        service
            .ingest_interaction("tenant-a", "shared-customer", "email", "allowed")
            .await
            .unwrap();
        let mut tenant_a_tx = pool.begin().await.unwrap();
        ::server_common::auth_utils::set_org_context(&mut *tenant_a_tx, "tenant-a")
            .await
            .unwrap();
        let stored_tenants: Vec<String> = sqlx::query_scalar(
            "SELECT tenant_id FROM interaction_events ORDER BY tenant_id",
        )
        .fetch_all(&mut *tenant_a_tx)
        .await
        .unwrap();
        tenant_a_tx.commit().await.unwrap();
        assert_eq!(stored_tenants, vec!["tenant-a"]);

        let loaded = service
            .get_profile_summary("tenant-a", "shared-customer")
            .await
            .unwrap();
        assert_eq!(loaded.summary, "summary-a");
        assert_eq!(loaded.total_interactions, 2);

        pool.close().await;
        sqlx::query(&format!("DROP SCHEMA {schema} CASCADE"))
            .execute(&admin)
            .await
            .unwrap();
        sqlx::query(&format!("DROP ROLE {role}"))
            .execute(&admin)
            .await
            .unwrap();
        admin.close().await;
    }
}
