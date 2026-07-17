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
    pub context_payload: Option<sqlx::types::Json<serde_json::Value>>,
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
                d.id as draft_id,
                w.id as work_item_id,
                w.tenant_id,
                p.id as customer_id,
                p.name as customer_name,
                w.source,
                d.response,
                d.status,
                d.created_at,
                w.payload as context_payload
            FROM agent_draft d
            JOIN work_item w ON d.work_item_id = w.id
            JOIN customer_profile p ON w.customer_id = p.id AND p.tenant_id = w.tenant_id
            WHERE w.tenant_id = $1 AND d.status = 'DRAFT'
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

        // Fetch the work item payload to execute specific actions if needed
        let draft: Option<(Uuid, Option<sqlx::types::Json<serde_json::Value>>)> = sqlx::query_as(
            r#"
            SELECT w.id, w.payload
            FROM agent_draft d
            JOIN work_item w ON d.work_item_id = w.id
            WHERE d.id = $1 AND w.tenant_id = $2
            "#
        )
        .bind(draft_id)
        .bind(tenant_id)
        .fetch_optional(&mut *tx)
        .await?;

        let result = sqlx::query(
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
        .execute(&mut *tx)
        .await?;

        if result.rows_affected() == 1 {
            // Process specific draft actions
            if let Some((_work_item_id, Some(payload))) = draft {
                let feature_type = payload.get("feature_type").and_then(|v| v.as_str()).unwrap_or("");

                if feature_type == "draft_purchase_order" {
                    if let Some(po_id) = payload.get("po_id").and_then(|v| v.as_str()) {
                        let _ = sqlx::query(
                            r#"
                            UPDATE purchase_orders
                            SET status = 'SENT', updated_at = NOW()
                            WHERE id = $1 AND tenant_id = $2
                            "#
                        )
                        .bind(po_id)
                        .bind(tenant_id.to_string())
                        .execute(&mut *tx)
                        .await;
                    }
                }
            }
        }

        tx.commit().await?;
        Ok(result.rows_affected() == 1)
    }

    pub async fn update_draft_response(&self, draft_id: Uuid, tenant_id: Uuid, new_response: &str) -> Result<bool, sqlx::Error> {
        // Multi-tenant isolation: we must ensure this draft belongs to a work_item that belongs to this tenant
        let mut tx = self.db.pool.begin().await?;
        ::server_common::auth_utils::set_org_context(&mut *tx, &tenant_id.to_string()).await?;
        let result = sqlx::query(
            r#"
            UPDATE agent_draft
            SET response = $1, updated_at = NOW()
            WHERE id = $2 AND work_item_id IN (
                SELECT id FROM work_item WHERE tenant_id = $3
            )
            "#
        )
        .bind(new_response)
        .bind(draft_id)
        .bind(tenant_id)
        .execute(&mut *tx)
        .await?;
        tx.commit().await?;
        Ok(result.rows_affected() == 1)
    }
}
