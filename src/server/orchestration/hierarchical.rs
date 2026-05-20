use std::sync::Arc;
use crate::db::{DB, DbStore};
use chrono::Utc;
use uuid::Uuid;

// Simulates a K8s Operator for Hierarchical Task Delegation
pub struct K8sOperatorDelegator;

impl K8sOperatorDelegator {
    pub async fn spawn_sub_agent_pod(
        db: Arc<DB>,
        role: &str,
        instruction: &str,
        thread_id: &str,
    ) -> Result<String, String> {
        let pod_id = format!("pod-sub-agent-{}-{}", role, Uuid::new_v4());
        let payload = serde_json::json!({
            "instruction": instruction,
            "thread_id": thread_id,
            "role": role,
            "pod_id": pod_id
        });
        let payload_str = serde_json::to_string(&payload).unwrap_or_default();
        let now = Utc::now();

        // Enqueue into sub_agent_queue
        match &db.store {
            DbStore::Postgres => {
                sqlx::query(
                    r#"
                    INSERT INTO sub_agent_queue (id, tenant_id, parent_task_id, payload, status, created_at, updated_at)
                    VALUES ($1, $2, $3, $4, $5, $6, $7)
                    "#
                )
                .bind(&pod_id)
                .bind("system") // default tenant
                .bind(thread_id)
                .bind(payload_str)
                .bind("QUEUED")
                .bind(now)
                .bind(now)
                .execute(&db.pool)
                .await
                .map_err(|e| e.to_string())?;
            }
            DbStore::Sqlite(sqlite_pool) => {
                sqlx::query(
                    r#"
                    INSERT INTO sub_agent_queue (id, tenant_id, parent_task_id, payload, status, created_at, updated_at)
                    VALUES (?, ?, ?, ?, ?, ?, ?)
                    "#
                )
                .bind(&pod_id)
                .bind("system")
                .bind(thread_id)
                .bind(payload_str)
                .bind("QUEUED")
                .bind(now.to_rfc3339())
                .bind(now.to_rfc3339())
                .execute(sqlite_pool)
                .await
                .map_err(|e| e.to_string())?;
            }
        }

        Ok(format!("Sub-agent {} (ID: {}) queued for execution", role, pod_id))
    }

    pub async fn spawn_and_wait_sub_agents(
        db: Arc<DB>,
        manager_role: &str,
        sub_tasks: Vec<(&str, &str)>,
        thread_id: &str,
    ) -> Result<String, String> {
        let mut results = Vec::new();

        for (role, instruction) in sub_tasks {
            let pod_result = Self::spawn_sub_agent_pod(db.clone(), role, instruction, thread_id).await?;
            results.push(pod_result);
        }

        Ok(format!("Manager '{}' coordinated sub-agents. Results:\n{}", manager_role, results.join("\n")))
    }
}
