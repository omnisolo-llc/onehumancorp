use sqlx::FromRow;
use uuid::Uuid;
use chrono::{DateTime, Utc};
use std::sync::Arc;
use crate::db::DB;

#[derive(Clone, Debug, FromRow, serde::Serialize)]
pub struct ActionRequiredDraft {
    pub draft_id: Uuid,
    pub work_item_id: Uuid,
    pub tenant_id: Uuid,
    pub customer_id: Uuid,
    pub customer_name: Option<String>,
    pub source: String,
    pub response: String,
    pub status: String,
    pub created_at: Option<DateTime<Utc>>,
}

pub struct ActionRequiredQueueRepo {
    db: Arc<DB>,
}

impl ActionRequiredQueueRepo {
    pub fn new(db: Arc<DB>) -> Self {
        Self { db }
    }

    pub async fn get_pending_drafts(&self, tenant_id: Uuid) -> Result<Vec<ActionRequiredDraft>, sqlx::Error> {
        let records = sqlx::query_as::<_, ActionRequiredDraft>(
            r#"
            SELECT
                d.id as draft_id,
                w.id as work_item_id,
                w.tenant_id,
                p.id as customer_id,
                p.name as customer_name,
                w.source,
                d.response,
                d.status,
                d.created_at
            FROM agent_draft d
            JOIN work_item w ON d.work_item_id = w.id
            JOIN customer_profile p ON w.customer_id = p.id
            WHERE w.tenant_id = $1 AND d.status = 'DRAFT'
            ORDER BY d.created_at ASC
            "#
        )
        .bind(tenant_id)
        .fetch_all(&self.db.pool)
        .await?;

        Ok(records)
    }

    pub async fn approve_draft(&self, draft_id: Uuid, tenant_id: Uuid) -> Result<(), sqlx::Error> {
        // We verify the tenant_id through the join to ensure isolation
        sqlx::query(
            r#"
            UPDATE agent_draft
            SET status = 'APPROVED', updated_at = NOW()
            WHERE id = $1 AND work_item_id IN (
                SELECT id FROM work_item WHERE tenant_id = $2
            )
            "#
        )
        .bind(draft_id)
        .bind(tenant_id)
        .execute(&self.db.pool)
        .await?;

        Ok(())
    }
}
