use sqlx::PgPool;
use serde_json::Value;
use uuid::Uuid;
use chrono::Utc;
use crate::domain::repository::agent_feed_repo::{AgentFeedRepository, AgentFeedItem};

pub struct WorkTriageService {
    pool: PgPool,
}

impl WorkTriageService {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn process_incoming_alert(
        &self,
        tenant_id: &str,
        event_source: String,
        context_payload: Option<Value>,
        proposed_action: Option<Value>,
        provided_correlation_id: Option<String>,
        provided_priority_score: Option<i32>,
    ) -> Result<AgentFeedItem, sqlx::Error> {
        let repo = AgentFeedRepository::new(self.pool.clone());

        let mut priority_score = provided_priority_score.unwrap_or(0);

        if provided_priority_score.is_none() {
            if event_source.to_lowercase().contains("failure") || event_source.to_lowercase().contains("failed") || event_source.to_lowercase().contains("incident") || event_source.to_lowercase().contains("dispute") {
                priority_score = 100;
            } else if event_source.to_lowercase().contains("message") || event_source.to_lowercase().contains("inquiry") || event_source.to_lowercase().contains("dm") {
                priority_score = 50;
            } else if event_source.to_lowercase().contains("order") || event_source.to_lowercase().contains("booking") {
                priority_score = 75;
            } else {
                priority_score = 10;
            }
        }

        let correlation_id = provided_correlation_id.unwrap_or_else(|| event_source.clone());

        let existing_items = sqlx::query_as::<_, AgentFeedItem>(
            r#"
            SELECT
                id,
                tenant_id,
                event_source,
                context_payload,
                proposed_action,
                lifecycle_state,
                priority_score,
                correlation_id,
                created_at,
                updated_at
            FROM agent_feed_items
            WHERE tenant_id = $1 AND correlation_id = $2 AND lifecycle_state = 'PENDING_APPROVAL'
            ORDER BY created_at DESC
            LIMIT 1
            "#
        )
        .bind(tenant_id)
        .bind(&correlation_id)
        .fetch_optional(&self.pool)
        .await?;

        if let Some(mut existing) = existing_items {
            let mut current_context = existing.context_payload.clone().map(|j| j.0).unwrap_or(serde_json::json!({}));

            let current_count = current_context.get("grouped_count").and_then(|v| v.as_i64()).unwrap_or(1);
            let new_count = current_count + 1;

            if let Some(obj) = current_context.as_object_mut() {
                obj.insert("grouped_count".to_string(), serde_json::json!(new_count));
                let desc = obj.get("description").and_then(|v| v.as_str()).unwrap_or("");
                if !desc.contains("grouped items") {
                    obj.insert("description".to_string(), serde_json::json!(format!("{} ({} grouped items)", desc, new_count)));
                } else {
                    let parts: Vec<&str> = desc.split(" (").collect();
                    if parts.len() > 0 {
                        obj.insert("description".to_string(), serde_json::json!(format!("{} ({} grouped items)", parts[0], new_count)));
                    }
                }
            }

            let updated_context = Some(sqlx::types::Json(current_context.clone()));

            sqlx::query(
                r#"
                UPDATE agent_feed_items
                SET context_payload = $1, updated_at = NOW(), priority_score = GREATEST(priority_score, $2)
                WHERE tenant_id = $3 AND id = $4
                "#
            )
            .bind(&updated_context)
            .bind(priority_score)
            .bind(tenant_id)
            .bind(&existing.id)
            .execute(&self.pool)
            .await?;

            existing.context_payload = updated_context;
            if let Some(existing_score) = existing.priority_score {
                if priority_score > existing_score {
                    existing.priority_score = Some(priority_score);
                }
            }

            return Ok(existing);
        }

        let item = AgentFeedItem {
            id: Uuid::new_v4().to_string(),
            tenant_id: tenant_id.to_string(),
            event_source,
            context_payload: context_payload.map(sqlx::types::Json),
            proposed_action: proposed_action.map(sqlx::types::Json),
            lifecycle_state: "PENDING_APPROVAL".to_string(),
            priority_score: Some(priority_score),
            correlation_id: Some(correlation_id),
            created_at: Some(Utc::now()),
            updated_at: Some(Utc::now()),
        };

        repo.create(item).await
    }
}
