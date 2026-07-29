use sqlx::{PgPool, FromRow};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use tracing::info;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct UnifiedThread {
    pub id: String,
    pub tenant_id: String,
    pub customer_id: Option<String>,
    pub channel: String,
    pub status: String,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct UnifiedMessage {
    pub id: String,
    pub tenant_id: String,
    pub thread_id: String,
    pub sender_type: String,
    pub content: String,
    pub created_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct UnifiedTriageAction {
    pub id: String,
    pub tenant_id: String,
    pub thread_id: String,
    pub action_type: String,
    pub action_payload: Option<String>,
    pub status: String,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

pub struct InboxService {
    pool: PgPool,
}

impl InboxService {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn ingest_message(
        &self,
        tenant_id: &str,
        customer_id: Option<&str>,
        channel: &str,
        sender_type: &str,
        content: &str,
    ) -> Result<String, sqlx::Error> {
        let mut tx = self.pool.begin().await?;

        sqlx::query("SELECT set_config('app.current_tenant', $1, true)")
            .bind(tenant_id)
            .execute(&mut *tx)
            .await?;

        let mut thread_id = None;
        if let Some(cid) = customer_id {
            let row = sqlx::query_as::<_, (String,)>(
                r#"SELECT id FROM unified_threads WHERE tenant_id = $1 AND customer_id = $2 AND channel = $3 AND status = 'open' LIMIT 1"#
            )
            .bind(tenant_id)
            .bind(cid)
            .bind(channel)
            .fetch_optional(&mut *tx)
            .await?;
            if let Some((id,)) = row {
                thread_id = Some(id);
            }
        }

        let tid = match thread_id {
            Some(id) => id,
            None => {
                let new_tid = Uuid::new_v4().to_string();
                sqlx::query(
                    r#"INSERT INTO unified_threads (id, tenant_id, customer_id, channel, status) VALUES ($1, $2, $3, $4, 'open')"#
                )
                .bind(&new_tid)
                .bind(tenant_id)
                .bind(customer_id)
                .bind(channel)
                .execute(&mut *tx)
                .await?;
                new_tid
            }
        };

        let msg_id = Uuid::new_v4().to_string();
        sqlx::query(
            r#"INSERT INTO unified_messages (id, tenant_id, thread_id, sender_type, content) VALUES ($1, $2, $3, $4, $5)"#
        )
        .bind(&msg_id)
        .bind(tenant_id)
        .bind(&tid)
        .bind(sender_type)
        .bind(content)
        .execute(&mut *tx)
        .await?;

        if sender_type == "customer" {
            self.trigger_ai_triage(&mut tx, tenant_id, &tid, content).await?;
        }

        tx.commit().await?;

        info!("Ingested message {} into thread {}", msg_id, tid);
        Ok(msg_id)
    }

    async fn trigger_ai_triage(
        &self,
        tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
        tenant_id: &str,
        thread_id: &str,
        message_content: &str,
    ) -> Result<(), sqlx::Error> {
        let job_id = Uuid::new_v4().to_string();
        let payload = serde_json::json!({
            "message_id": thread_id, // Map message_id to thread_id for unified queue consumption
            "source": "unified_inbox",
            "content": message_content,
            "sender_id": "customer"
        });

        sqlx::query("INSERT INTO ohc_job_queue (id, tenant_id, job_type, payload, status) VALUES ($1, $2, 'message_triage', $3, 'PENDING')")
            .bind(&job_id)
            .bind(tenant_id)
            .bind(payload.to_string())
            .execute(&mut **tx)
            .await?;

        info!("Triggered AI triage for thread {}", thread_id);
        Ok(())
    }

    pub async fn get_pending_actions(&self, tenant_id: &str) -> Result<Vec<UnifiedTriageAction>, sqlx::Error> {
        let mut tx = self.pool.begin().await?;
        sqlx::query("SELECT set_config('app.current_tenant', $1, true)")
            .bind(tenant_id)
            .execute(&mut *tx)
            .await?;

        let actions = sqlx::query_as::<_, UnifiedTriageAction>(
            r#"SELECT id, tenant_id, thread_id, action_type, action_payload, status, created_at, updated_at FROM unified_triage_actions WHERE tenant_id = $1 AND status = 'pending' ORDER BY created_at ASC"#
        )
        .bind(tenant_id)
        .fetch_all(&mut *tx)
        .await?;

        tx.commit().await?;
        Ok(actions)
    }

    pub async fn resolve_action(&self, tenant_id: &str, action_id: &str, resolution: &str) -> Result<(), sqlx::Error> {
        let mut tx = self.pool.begin().await?;
        sqlx::query("SELECT set_config('app.current_tenant', $1, true)")
            .bind(tenant_id)
            .execute(&mut *tx)
            .await?;

        let row = sqlx::query_as::<_, (String, Option<String>)>(
            r#"UPDATE unified_triage_actions SET status = $1, updated_at = NOW() WHERE id = $2 AND tenant_id = $3 RETURNING thread_id, action_payload"#
        )
        .bind(resolution)
        .bind(action_id)
        .bind(tenant_id)
        .fetch_optional(&mut *tx)
        .await?;

        if let Some((thread_id, action_payload)) = row {
             if resolution == "approved" {
                 if let Some(payload) = action_payload {
                     if let Ok(parsed) = serde_json::from_str::<serde_json::Value>(&payload) {
                         if let Some(reply) = parsed.get("draft_reply").and_then(|v| v.as_str()) {
                             let msg_id = Uuid::new_v4().to_string();
                             sqlx::query(
                                r#"INSERT INTO unified_messages (id, tenant_id, thread_id, sender_type, content) VALUES ($1, $2, $3, 'agent', $4)"#
                             )
                             .bind(msg_id)
                             .bind(tenant_id)
                             .bind(&thread_id)
                             .bind(reply)
                             .execute(&mut *tx)
                             .await?;
                             info!("Executed approved DraftReply action {} for thread {}", action_id, thread_id);
                         }
                     }
                 }
             }
        }

        tx.commit().await?;
        Ok(())
    }
}
