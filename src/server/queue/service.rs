pub struct TaskQueueService {
    pool: sqlx::PgPool,
}

impl TaskQueueService {
    pub fn new(pool: sqlx::PgPool) -> Self {
        TaskQueueService { pool }
    }

    pub async fn push_task(&self, task: SharedTaskModel) -> Result<(), sqlx::Error> {
        let payload_str = serde_json::to_string(&task.payload).unwrap_or_default();
        let deps_str = serde_json::to_string(&task.dependencies).unwrap_or_else(|_| "[]".to_string());

        let mut tx = self.pool.begin().await?;
        set_org_context(&mut *tx, &task.tenant_id).await?;
        sqlx::query("INSERT INTO shared_tasks (id, parent_id, epic_id, title, status, assigned_agent, payload, tenant_id, dependencies) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9::jsonb)")
            .bind(task.id)
            .bind(task.parent_id)
            .bind(task.epic_id)
            .bind(task.title)
            .bind("PENDING")
            .bind(task.assigned_agent)
            .bind(payload_str)
            .bind(task.tenant_id)
            .bind(deps_str)
            .execute(&mut *tx)
            .await?;
        tx.commit().await?;
        Ok(())
    }

    pub async fn claim_task(&self, agent_id: &str) -> Result<Option<SharedTaskModel>, sqlx::Error> {
        let mut tx = self.pool.begin().await?;
        sqlx::query("SET LOCAL ROLE ohc_bypassrls").execute(&mut *tx).await?;
        let row = sqlx::query("UPDATE shared_tasks SET status = 'IN_PROGRESS', assigned_agent = $1 WHERE id = (SELECT st.id FROM shared_tasks st WHERE st.status = 'PENDING' AND (st.assigned_agent IS NULL OR st.assigned_agent = $1) AND NOT EXISTS (SELECT 1 FROM jsonb_array_elements_text(st.dependencies) AS dep_id JOIN shared_tasks parent ON parent.id::text = dep_id WHERE parent.status != 'COMPLETED') ORDER BY st.created_at ASC LIMIT 1 FOR UPDATE SKIP LOCKED) RETURNING id, tenant_id, parent_id, epic_id, title, status, assigned_agent, payload, dependencies::text AS dependencies, created_at, updated_at")
            .bind(agent_id)
            .fetch_optional(&mut *tx)
            .await?;
        tx.commit().await?;

        if let Some(row) = row {
            let payload_str: String = row.get("payload");
            let payload: serde_json::Value = serde_json::from_str(&payload_str).unwrap_or_else(|_| serde_json::json!({}));
            let deps_str: String = row.get("dependencies");
            let dependencies: serde_json::Value = serde_json::from_str(&deps_str).unwrap_or_else(|_| serde_json::json!([]));

            Ok(Some(SharedTaskModel {
                id: row.get("id"),
                tenant_id: row.get("tenant_id"),
                parent_id: row.get("parent_id"),
                epic_id: row.get("epic_id"),
                title: row.get("title"),
                status: row.get("status"),
                assigned_agent: row.get("assigned_agent"),
                payload,
                dependencies,
                created_at: row.get("created_at"),
                updated_at: row.get("updated_at"),
            }))
        } else {
            Ok(None)
        }
    }

    pub async fn complete_task(&self, task_id: &str) -> Result<(), sqlx::Error> {
        let mut tx = self.pool.begin().await?;
        set_org_context(&mut *tx, "system").await?;
        sqlx::query("UPDATE shared_tasks SET status = 'COMPLETED', updated_at = CURRENT_TIMESTAMP WHERE id = $1")
            .bind(task_id)
            .execute(&mut *tx)
            .await?;
        tx.commit().await?;
        Ok(())
    }



    pub async fn fail_task(&self, task_id: &str, reason: &str) -> Result<(), sqlx::Error> {
        let payload_update = serde_json::to_string(&serde_json::json!({"error": reason})).unwrap_or_else(|_| "{}".to_string());
        // We could merge this better using jsonb operators or just save status
        let mut tx = self.pool.begin().await?;
        set_org_context(&mut *tx, "system").await?;
        sqlx::query("UPDATE shared_tasks SET status = 'FAILED', payload = COALESCE(payload, '{}'::jsonb) || $2::jsonb, updated_at = CURRENT_TIMESTAMP WHERE id = $1")
            .bind(task_id)
            .bind(payload_update)
            .execute(&mut *tx)
            .await?;
        tx.commit().await?;
        Ok(())
    }


    pub async fn get_completed_tasks(&self, limit: i64) -> Result<Vec<SharedTaskModel>, sqlx::Error> {
        let rows = sqlx::query("SELECT id, tenant_id, parent_id, epic_id, title, status, assigned_agent, payload, dependencies::text AS dependencies, created_at, updated_at FROM shared_tasks WHERE status = 'COMPLETED' LIMIT $1")
            .bind(limit)
            .fetch_all(&self.pool)
            .await?;

        let mut tasks = Vec::new();
        for row in rows {
            let payload_str: String = row.get("payload");
            let payload: serde_json::Value = serde_json::from_str(&payload_str).unwrap_or_else(|_| serde_json::json!({}));
            let deps_str: String = row.get("dependencies");
            let dependencies: serde_json::Value = serde_json::from_str(&deps_str).unwrap_or_else(|_| serde_json::json!([]));

            tasks.push(SharedTaskModel {
                id: row.get("id"),
                tenant_id: row.get("tenant_id"),
                parent_id: row.get("parent_id"),
                epic_id: row.get("epic_id"),
                title: row.get("title"),
                status: row.get("status"),
                assigned_agent: row.get("assigned_agent"),
                payload,
                dependencies,
                created_at: row.get("created_at"),
                updated_at: row.get("updated_at"),
            });
        }

        Ok(tasks)
    }
}
