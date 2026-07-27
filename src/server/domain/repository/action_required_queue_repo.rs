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
        let mut tx = self.db.pool.begin().await?;
        ::server_common::auth_utils::set_org_context(&mut *tx, &tenant_id.to_string()).await?;
        let records = sqlx::query_as::<_, ActionRequiredDraft>(
            r#"
            SELECT
                d.id::uuid as draft_id,
                w.id::uuid as work_item_id,
                w.tenant_id::uuid,
                p.id::uuid as customer_id,
                p.name as customer_name,
                w.channel as source,
                d.action_payload as response,
                d.status,
                d.created_at
            FROM unified_triage_actions d
            JOIN unified_threads w ON d.thread_id = w.id
            LEFT JOIN chat_contacts p ON w.customer_id = p.id AND p.tenant_id = w.tenant_id
            WHERE w.tenant_id = $1::text AND d.status = 'pending' AND d.action_type = 'DraftReply'
            ORDER BY d.created_at ASC
            "#
        )
        .bind(tenant_id)
        .fetch_all(&mut *tx)
        .await?;

        tx.commit().await?;

        Ok(records)
    }

    pub async fn approve_draft(&self, draft_id: Uuid, tenant_id: Uuid) -> Result<bool, sqlx::Error> {
        // We verify the tenant_id through the join to ensure isolation
        let mut tx = self.db.pool.begin().await?;
        ::server_common::auth_utils::set_org_context(&mut *tx, &tenant_id.to_string()).await?;
        let result = sqlx::query(
            r#"
            UPDATE unified_triage_actions
            SET status = 'approved', updated_at = NOW()
            WHERE id = $1::text AND thread_id IN (
                SELECT id FROM unified_threads WHERE tenant_id = $2::text
            )
            "#
        )
        .bind(draft_id.to_string())
        .bind(tenant_id.to_string())
        .execute(&mut *tx)
        .await?;
        tx.commit().await?;
        Ok(result.rows_affected() == 1)
    }

    pub async fn update_draft_response(&self, draft_id: Uuid, tenant_id: Uuid, new_response: &str) -> Result<bool, sqlx::Error> {
        // Multi-tenant isolation: we must ensure this draft belongs to a work_item that belongs to this tenant
        let mut tx = self.db.pool.begin().await?;
        ::server_common::auth_utils::set_org_context(&mut *tx, &tenant_id.to_string()).await?;
        let result = sqlx::query(
            r#"
            UPDATE unified_triage_actions
            SET action_payload = $1, updated_at = NOW()
            WHERE id = $2::text AND thread_id IN (
                SELECT id FROM unified_threads WHERE tenant_id = $3::text
            )
            "#
        )
        .bind(new_response)
        .bind(draft_id.to_string())
        .bind(tenant_id.to_string())
        .execute(&mut *tx)
        .await?;
        tx.commit().await?;
        Ok(result.rows_affected() == 1)
    }
}
