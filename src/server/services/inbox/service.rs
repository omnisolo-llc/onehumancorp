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
        _message_content: &str,
    ) -> Result<(), sqlx::Error> {
        // Queue the inbound message for the Autonomous Negotiator.
        // The orchestration layer will pick this up to generate a quote.
        let action_id = Uuid::new_v4().to_string();
        let payload = serde_json::json!({
            "inbound_message": _message_content,
            "thread_id": thread_id
        });
        let payload_str = serde_json::to_string(&payload).unwrap_or_default();

        sqlx::query(
            r#"INSERT INTO unified_triage_actions (id, tenant_id, thread_id, action_type, action_payload, status) VALUES ($1, $2, $3, 'AgenticNegotiationQueue', $4, 'queued_for_agent')"#
        )
        .bind(&action_id)
        .bind(tenant_id)
        .bind(thread_id)
        .bind(&payload_str)
        .execute(&mut **tx)
        .await?;

        info!("Queued AI Agentic Negotiation for thread {} (action {})", thread_id, action_id);
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
                         let mut reply_opt = parsed.get("draft_reply").and_then(|v| v.as_str()).map(|s| s.to_string());
                         if reply_opt.is_none() && parsed.get("feature_type").and_then(|v| v.as_str()) == Some("quote_draft") {
                             let price = parsed.get("suggested_price").and_then(|v| v.as_f64()).unwrap_or(0.0);
                             reply_opt = Some(format!("I have prepared a quote for your request. The suggested price is ${:.2}. Let me know if you would like to proceed.", price));
                         }
                         if let Some(reply) = reply_opt {
                             let msg_id = Uuid::new_v4().to_string();
                             sqlx::query(
                                r#"INSERT INTO unified_messages (id, tenant_id, thread_id, sender_type, content) VALUES ($1, $2, $3, 'agent', $4)"#
                             )
                             .bind(msg_id)
                             .bind(tenant_id)
                             .bind(&thread_id)
                             .bind(&reply)
                             .execute(&mut *tx)
                             .await?;
                             info!("Executed approved action {} for thread {}", action_id, thread_id);
                         }
                     }
                 }
             }
        }

        tx.commit().await?;
        Ok(())
    }
}
