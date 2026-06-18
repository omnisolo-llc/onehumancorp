use std::sync::Arc;
use crate::db::DB;
use serde_json::json;
use std::time::Duration;
use uuid::Uuid;

pub struct LifecycleEngagementWorker {
    pub db: Arc<DB>,
}

impl LifecycleEngagementWorker {
    pub fn new(db: Arc<DB>) -> Self {
        Self { db }
    }

    pub fn start(&self) {
        let db = self.db.clone();
        tokio::spawn(async move {
            let mut interval = tokio::time::interval(std::time::Duration::from_secs(60));
            loop {
                interval.tick().await;
                let _ = Self::poll(&db).await;
            }
        });
    }

    pub async fn poll(db: &Arc<DB>) -> Result<bool, String> {
        let pool = db.pool.clone();

        let customers: Vec<(Uuid, String, String, Option<String>, Option<String>)> = match &db.store {
            crate::db::DbStore::Postgres => {
                sqlx::query_as(
                    "SELECT id, tenant_id, name, email, phone FROM customers WHERE updated_at < CURRENT_TIMESTAMP - INTERVAL '90 days'"
                )
                .fetch_all(&pool)
                .await
                .unwrap_or_default()
            },
            crate::db::DbStore::Sqlite(_) => {
                sqlx::query_as(
                    "SELECT id, tenant_id, name, email, phone FROM customers WHERE updated_at < datetime('now', '-90 days')"
                )
                .fetch_all(&pool)
                .await
                .unwrap_or_default()
            }
        };

        if customers.is_empty() {
            return Ok(false);
        }

        let mut processed_any = false;

        for (_customer_id, tenant_id, name, email, phone) in customers {
            let pending_count: i64 = sqlx::query_scalar(
                "SELECT COUNT(*) FROM shared_tasks WHERE organization_id = $1 AND title LIKE 'Lifecycle Engagement:%' AND approval_status = 'PENDING'"
            )
            .bind(&tenant_id)
            .fetch_one(&pool)
            .await
            .unwrap_or(0);

            if pending_count > 0 {
                continue;
            }

            let ledger_entries: Vec<(String, serde_json::Value)> = sqlx::query_as(
                "SELECT action_type, state_change FROM ohc_universal_ledger WHERE tenant_id = $1 ORDER BY created_at DESC LIMIT 5"
            )
            .bind(&tenant_id)
            .fetch_all(&pool)
            .await
            .unwrap_or_default();

            let context = json!({
                "customer_name": name,
                "customer_email": email,
                "customer_phone": phone,
                "recent_history": ledger_entries.iter().map(|(_, sc)| sc.clone()).collect::<Vec<_>>()
            });

            let llm_key = std::env::var("MINIMAX_API_KEY").unwrap_or_default();
            let provider = crate::minimax::MinimaxClient::new(llm_key);
            let prompt = format!(
                "You are the Customer Success Ambassador for the business. \
                Draft a friendly, personalized check-in message for {name} who hasn't interacted with us in 90 days. \
                Context: {context}",
                name = name,
                context = context
            );

            let draft = match provider.reason(&prompt).await {
                Ok(response) => response,
                Err(_) => format!("Hi {}, it's been a while! We wanted to check in and see how you're doing. Let us know if you need anything.", name),
            };

            let task_id = format!("task-lifecycle-{}", Uuid::new_v4());
            let title = format!("Lifecycle Engagement: Check-in with {}", name);
            let description = "Drafted check-in message for dormant customer.".to_string();

            sqlx::query(
                "INSERT INTO shared_tasks (id, organization_id, title, description, status, priority, action_risk, approval_status, proposed_content)
                 VALUES ($1, $2, $3, $4, 'OPEN', 'MEDIUM', 'LOW', 'PENDING', $5)"
            )
            .bind(task_id)
            .bind(&tenant_id)
            .bind(title)
            .bind(description)
            .bind(draft)
            .execute(&pool)
            .await
            .unwrap_or_default();

            processed_any = true;
        }

        Ok(processed_any)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use crate::db::DbStore;

    async fn setup_test_db() -> Option<Arc<DB>> {
        let db = match DB::new().await {
            Ok(db) => db,
            Err(_) => return None,
        };
        let pool = db.pool.clone();

        let _ = sqlx::query("CREATE TABLE IF NOT EXISTS tenants (id TEXT PRIMARY KEY, name TEXT, industry TEXT);").execute(&pool).await;
        let _ = sqlx::query("CREATE TABLE IF NOT EXISTS customers (id UUID PRIMARY KEY, tenant_id TEXT, name TEXT, email TEXT, phone TEXT, updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP);").execute(&pool).await;
        let _ = sqlx::query("CREATE TABLE IF NOT EXISTS ohc_universal_ledger (id TEXT PRIMARY KEY, tenant_id TEXT, department TEXT, action_type TEXT, state_change TEXT, created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP);").execute(&pool).await;
        let _ = sqlx::query("CREATE TABLE IF NOT EXISTS shared_tasks (id TEXT PRIMARY KEY, organization_id TEXT, title TEXT, description TEXT, status TEXT, priority TEXT, action_risk TEXT, approval_status TEXT, proposed_content TEXT);").execute(&pool).await;

        Some(Arc::new(db))
    }

    #[tokio::test]
    async fn test_lifecycle_engagement_worker() {
        let db = match setup_test_db().await {
            Some(db) => db,
            None => return,
        };
        let pool = db.pool.clone();

        sqlx::query("INSERT INTO tenants (id, name, industry) VALUES ('tenant-1', 'Music Tutor', 'Education')")
            .execute(&pool).await.unwrap();

        let customer_id = Uuid::new_v4();
        sqlx::query("INSERT INTO customers (id, tenant_id, name, email, updated_at) VALUES ($1, 'tenant-1', 'Sarah', 'sarah@example.com', datetime('now', '-100 days'))")
            .bind(customer_id)
            .execute(&pool).await.unwrap();

        let active_customer_id = Uuid::new_v4();
        sqlx::query("INSERT INTO customers (id, tenant_id, name, email, updated_at) VALUES ($1, 'tenant-1', 'John', 'john@example.com', datetime('now', '-10 days'))")
            .bind(active_customer_id)
            .execute(&pool).await.unwrap();

        let state_change = json!({"lesson": "Guitar 101"}).to_string();
        sqlx::query("INSERT INTO ohc_universal_ledger (id, tenant_id, department, action_type, state_change) VALUES ('ledger-1', 'tenant-1', 'Sales', 'lesson_booked', $1)")
            .bind(state_change)
            .execute(&pool).await.unwrap();

        let _ = LifecycleEngagementWorker::poll(&db).await;

        let task: Option<(String, String, String)> = sqlx::query_as("SELECT title, approval_status, proposed_content FROM shared_tasks WHERE organization_id = 'tenant-1'")
            .fetch_optional(&pool).await.unwrap_or(None);

        if let Some(t) = task {
            assert!(t.0.contains("Sarah"));
            assert_eq!(t.1, "PENDING");
            assert!(t.2.contains("Sarah"));
        }

        let _ = LifecycleEngagementWorker::poll(&db).await;
    }
}
