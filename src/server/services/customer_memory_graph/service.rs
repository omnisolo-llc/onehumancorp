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
    #[serde(default)]
    pub customer_name: String,
    #[serde(default)]
    pub interactions: Vec<serde_json::Value>,
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
        sqlx::query("SELECT set_config('app.current_tenant', $1, true)")
            .bind(tenant_id)
            .execute(&mut *tx)
            .await?;

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
        sqlx::query("SELECT set_config('app.current_tenant', $1, true)")
            .bind(tenant_id)
            .execute(&mut *tx)
            .await?;

        let record = sqlx::query(
            "SELECT profile_summary, name FROM customers WHERE id = $1"
        )
        .bind(customer_id)
        .fetch_optional(&mut *tx)
        .await?;

        let mut summary = if let Some(row) = record {
            let mut s = if let Ok(val) = row.try_get::<sqlx::types::Json<CustomerProfileSummary>, _>("profile_summary") {
                val.0
            } else {
                CustomerProfileSummary {
                    total_interactions: 0,
                    last_interaction: None,
                    segments: vec![],
                    preferences: vec![],
                    summary: "No summary available.".to_string(),
                    customer_name: "Unknown Customer".to_string(),
                    interactions: vec![],
                }
            };
            if let Ok(name) = row.try_get::<String, _>("name") {
                s.customer_name = name;
            }
            s
        } else {
            CustomerProfileSummary {
                total_interactions: 0,
                last_interaction: None,
                segments: vec![],
                preferences: vec![],
                summary: "Customer not found.".to_string(),
                customer_name: "Unknown Customer".to_string(),
                interactions: vec![],
            }
        };

        let interaction_records = sqlx::query(
            "SELECT channel, raw_content, created_at FROM interaction_events WHERE customer_id = $1 ORDER BY created_at DESC"
        )
        .bind(customer_id)
        .fetch_all(&mut *tx)
        .await?;

        for record in interaction_records {
            let channel: String = record.try_get("channel").unwrap_or_default();
            let raw_content: String = record.try_get("raw_content").unwrap_or_default();
            let created_at: Option<DateTime<Utc>> = record.try_get("created_at").ok();

            summary.interactions.push(serde_json::json!({
                "channel": channel,
                "description": raw_content,
                "created_at": created_at,
            }));
        }

        summary.total_interactions = summary.interactions.len() as i64;

        if summary.last_interaction.is_none() && !summary.interactions.is_empty() {
            summary.last_interaction = summary.interactions[0].get("created_at").and_then(|v| serde_json::from_value(v.clone()).ok());
        }

        tx.commit().await?;
        Ok(summary)
    }

    // Process jobs using SKIP LOCKED
    pub async fn process_pending_jobs(&self) -> Result<(), sqlx::Error> {
        // Fetch a batch of jobs across any tenant (admin operation)
        // Usually workers would have an admin context or set tenant per job
        let mut tx = self.pool.begin().await?;

        // Temporarily bypass RLS to find jobs (or we process per tenant).
        // For simplicity, we bypass it for the queue worker.
        ::server_common::auth_utils::set_org_context(&mut *tx, "").await?;

        let jobs = sqlx::query(
            "SELECT job_id, tenant_id, interaction_event_id FROM interaction_event_jobs
             WHERE status = 'pending'
             FOR UPDATE SKIP LOCKED LIMIT 10"
        )
        .fetch_all(&mut *tx)
        .await?;

        for job in jobs {
            let job_id: Uuid = job.get("job_id");
            let tenant_id: String = job.get("tenant_id");
            let event_id: Uuid = job.get("interaction_event_id");

            sqlx::query("SELECT set_config('app.current_tenant', $1, true)")
                .bind(&tenant_id)
                .execute(&mut *tx)
                .await?;

            let event = sqlx::query(
                "SELECT customer_id, channel, raw_content FROM interaction_events WHERE id = $1"
            )
            .bind(event_id)
            .fetch_one(&mut *tx)
            .await?;

            let mut customer_id: String = event.get("customer_id");
            let channel: String = event.get("channel");
            let content: String = event.get("raw_content");

            // Omnichannel Identity Graph: Merge logic
            // 1. Try to extract identifiers from content (e.g. email or phone)
            // In a real agent we would ask LLM, here we do deterministic regex matching for emails or assume the channel itself provides identity.
            // If we have an existing customer with this channel/content as identity, we merge.
            // For now, if the content contains an email, we match the email.
            let mut extracted_email = None;
            if content.contains('@') && content.contains('.') {
                let parts: Vec<&str> = content.split_whitespace().collect();
                for p in parts {
                    if p.contains('@') && p.contains('.') {
                        extracted_email = Some(p.to_string());
                        break;
                    }
                }
            }

            if let Some(email) = extracted_email {
                let existing_id: Option<String> = sqlx::query_scalar(
                    "SELECT id FROM customers WHERE email = $1 AND id != $2"
                )
                .bind(&email)
                .bind(&customer_id)
                .fetch_optional(&mut *tx)
                .await?;

                if let Some(target_id) = existing_id {
                    // Merge! Update interactions
                    let _ = sqlx::query("UPDATE interaction_events SET customer_id = $1 WHERE customer_id = $2")
                        .bind(&target_id)
                        .bind(&customer_id)
                        .execute(&mut *tx)
                        .await;

                    // Update identities
                    let _ = sqlx::query("UPDATE customer_identities SET customer_id = $1 WHERE customer_id = $2")
                        .bind(&target_id)
                        .bind(&customer_id)
                        .execute(&mut *tx)
                        .await;

                    let _ = sqlx::query("UPDATE context_snippets SET customer_id = $1 WHERE customer_id = $2")
                        .bind(&target_id)
                        .bind(&customer_id)
                        .execute(&mut *tx)
                        .await;

                    let _ = sqlx::query("UPDATE work_item SET customer_id = $1 WHERE customer_id = $2")
                        .bind(&target_id)
                        .bind(&customer_id)
                        .execute(&mut *tx)
                        .await;

                    // Update customer_timeline
                    let _ = sqlx::query("UPDATE customer_timeline SET customer_id = $1 WHERE customer_id = $2")
                        .bind(&target_id)
                        .bind(&customer_id)
                        .execute(&mut *tx)
                        .await;

                    let _ = sqlx::query("UPDATE subscriptions SET customer_id = $1 WHERE customer_id = $2")
                        .bind(&target_id)
                        .bind(&customer_id)
                        .execute(&mut *tx)
                        .await;

                    let _ = sqlx::query("UPDATE projects SET customer_id = $1 WHERE customer_id = $2")
                        .bind(&target_id)
                        .bind(&customer_id)
                        .execute(&mut *tx)
                        .await;

                    let _ = sqlx::query("UPDATE bookings SET customer_id = $1 WHERE customer_id = $2")
                        .bind(&target_id)
                        .bind(&customer_id)
                        .execute(&mut *tx)
                        .await;

                    let _ = sqlx::query("UPDATE orders SET customer_id = $1 WHERE customer_id = $2")
                        .bind(&target_id)
                        .bind(&customer_id)
                        .execute(&mut *tx)
                        .await;

                    let _ = sqlx::query("UPDATE ai_memories SET customer_id = $1 WHERE customer_id = $2")
                        .bind(&target_id)
                        .bind(&customer_id)
                        .execute(&mut *tx)
                        .await;

                    let _ = sqlx::query("UPDATE appointments SET customer_id = $1 WHERE customer_id = $2")
                        .bind(&target_id)
                        .bind(&customer_id)
                        .execute(&mut *tx)
                        .await;

                    let _ = sqlx::query("UPDATE interactions SET customer_id = $1 WHERE customer_id = $2")
                        .bind(&target_id)
                        .bind(&customer_id)
                        .execute(&mut *tx)
                        .await;

                    let _ = sqlx::query("UPDATE customer_timeline SET customer_id = $1 WHERE customer_id = $2")
                        .bind(&target_id)
                        .bind(&customer_id)
                        .execute(&mut *tx)
                        .await;

                    // Remove old
                    let _ = sqlx::query("DELETE FROM customers WHERE id = $1")
                        .bind(&customer_id)
                        .execute(&mut *tx)
                        .await;

                    customer_id = target_id;
                } else {
                    // Update email if not present
                    sqlx::query("UPDATE customers SET email = $1 WHERE id = $2 AND email IS NULL")
                        .bind(&email)
                        .bind(&customer_id)
                        .execute(&mut *tx)
                        .await?;
                }
            }

            // Add a mock snippet based on content
            let snippet_id = Uuid::new_v4();
            let category = if content.to_lowercase().contains("vegan") {
                "Dietary Preference"
            } else {
                "General Inquiry"
            };

            let extracted_value = if content.to_lowercase().contains("vegan") {
                "Vegan"
            } else {
                "Interested"
            };

            sqlx::query(
                "INSERT INTO context_snippets (id, tenant_id, customer_id, category, extracted_value)
                 VALUES ($1, $2, $3, $4, $5)"
            )
            .bind(snippet_id)
            .bind(&tenant_id)
            .bind(&customer_id)
            .bind(category)
            .bind(extracted_value)
            .execute(&mut *tx)
            .await?;

            // Update customer profile summary
            let summary = CustomerProfileSummary {
                total_interactions: 1, // simplified
                last_interaction: Some(Utc::now()),
                segments: vec!["Returning".to_string()],
                preferences: vec![extracted_value.to_string()],
                summary: format!("Customer recently asked about: {}", category),
            };

            sqlx::query(
                "UPDATE customers SET profile_summary = $1 WHERE id = $2"
            )
            .bind(sqlx::types::Json(summary))
            .bind(&customer_id)
            .execute(&mut *tx)
            .await?;

            sqlx::query(
                "UPDATE interaction_event_jobs SET status = 'completed', updated_at = CURRENT_TIMESTAMP WHERE job_id = $1"
            )
            .bind(job_id)
            .execute(&mut *tx)
            .await?;
        }

        tx.commit().await?;
        Ok(())
    }
}
