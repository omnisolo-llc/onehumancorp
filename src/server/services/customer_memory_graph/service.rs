use sqlx::{PgPool, Row};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct InteractionEvent {
    pub id: Uuid,
    pub tenant_id: String,
    pub customer_id: String,
    pub channel: String,
    pub raw_content: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CustomerProfileSummary {
    pub total_interactions: i64,
    pub last_interaction: Option<DateTime<Utc>>,
    pub segments: Vec<String>,
    pub preferences: Vec<String>,
    pub summary: String,
    #[serde(default)]
    pub events: Vec<InteractionEvent>,
}

pub struct CustomerMemoryGraphService {
    pool: PgPool,
}

impl CustomerMemoryGraphService {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn resolve_customer(&self, tenant_id: &str, channel: &str, identifier: &str) -> Result<String, sqlx::Error> {
        let mut tx = self.pool.begin().await?;
        sqlx::query("SELECT set_config('app.current_tenant', $1, true)")
            .bind(tenant_id)
            .execute(&mut *tx)
            .await?;

        // Check if alias exists
        let record = sqlx::query(
            "SELECT customer_id FROM customer_aliases WHERE tenant_id = $1 AND channel_type = $2 AND identifier = $3"
        )
        .bind(tenant_id)
        .bind(channel)
        .bind(identifier)
        .fetch_optional(&mut *tx)
        .await?;

        if let Some(row) = record {
            let customer_id: String = row.get("customer_id");
            tx.commit().await?;
            return Ok(customer_id);
        }

        // Try probabilistic matching: match by email or phone (if identifier looks like it)
        let customer_record = if identifier.contains("@") {
            sqlx::query("SELECT id FROM customers WHERE tenant_id = $1 AND email = $2")
                .bind(tenant_id)
                .bind(identifier)
                .fetch_optional(&mut *tx)
                .await?
        } else {
            sqlx::query("SELECT id FROM customers WHERE tenant_id = $1 AND phone = $2")
                .bind(tenant_id)
                .bind(identifier)
                .fetch_optional(&mut *tx)
                .await?
        };

        let customer_id = if let Some(row) = customer_record {
            row.get::<String, _>("id")
        } else {
            // Create new customer
            let new_customer_id = format!("c_{}", Uuid::new_v4().simple());
            sqlx::query(
                "INSERT INTO customers (id, tenant_id, name, email, phone) VALUES ($1, $2, $3, $4, $5)"
            )
            .bind(&new_customer_id)
            .bind(tenant_id)
            .bind(format!("Unknown {}", identifier))
            .bind(if identifier.contains("@") { Some(identifier) } else { None })
            .bind(if !identifier.contains("@") { Some(identifier) } else { None })
            .execute(&mut *tx)
            .await?;
            new_customer_id
        };

        // Create alias
        sqlx::query(
            "INSERT INTO customer_aliases (id, tenant_id, customer_id, channel_type, identifier) VALUES ($1, $2, $3, $4, $5)"
        )
        .bind(Uuid::new_v4())
        .bind(tenant_id)
        .bind(&customer_id)
        .bind(channel)
        .bind(identifier)
        .execute(&mut *tx)
        .await?;

        tx.commit().await?;
        Ok(customer_id)
    }

    pub async fn ingest_interaction(&self, tenant_id: &str, customer_id_or_identifier: &str, channel: &str, raw_content: &str) -> Result<Uuid, sqlx::Error> {
        let customer_id = self.resolve_customer(tenant_id, channel, customer_id_or_identifier).await?;
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
        .bind(&customer_id)
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
            "SELECT profile_summary FROM customers WHERE id = $1"
        )
        .bind(customer_id)
        .fetch_optional(&mut *tx)
        .await?;

        let events_records = sqlx::query(
            "SELECT id, tenant_id, customer_id, channel, raw_content, created_at FROM interaction_events WHERE tenant_id = $1 AND customer_id = $2 ORDER BY created_at DESC"
        )
        .bind(tenant_id)
        .bind(customer_id)
        .fetch_all(&mut *tx)
        .await?;

        let events: Vec<InteractionEvent> = events_records.into_iter().map(|row| InteractionEvent {
            id: row.get("id"),
            tenant_id: row.get("tenant_id"),
            customer_id: row.get("customer_id"),
            channel: row.get("channel"),
            raw_content: row.get("raw_content"),
            created_at: row.get("created_at"),
        }).collect();

        let summary = if let Some(row) = record {
            if let Ok(val) = row.try_get::<sqlx::types::Json<CustomerProfileSummary>, _>("profile_summary") {
                let mut s = val.0;
                s.events = events.clone();
                s
            } else {
                CustomerProfileSummary {
                    total_interactions: 0,
                    last_interaction: None,
                    segments: vec![],
                    preferences: vec![],
                    summary: "No summary available.".to_string(),
                    events: events.clone(),
                }
            }
        } else {
            CustomerProfileSummary {
                total_interactions: 0,
                last_interaction: None,
                segments: vec![],
                preferences: vec![],
                summary: "Customer not found.".to_string(),
                events: events.clone(),
            }
        };

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

            // Mocking the AI extraction part:
            // 1. Fetch event content
            // 2. Call LLM (mocked here)
            // 3. Extract context snippet
            // 4. Update customer profile summary

            // Set tenant for the rest of processing
            sqlx::query("SELECT set_config('app.current_tenant', $1, true)")
                .bind(&tenant_id)
                .execute(&mut *tx)
                .await?;

            let event = sqlx::query(
                "SELECT customer_id, raw_content FROM interaction_events WHERE id = $1"
            )
            .bind(event_id)
            .fetch_one(&mut *tx)
            .await?;

            let customer_id: String = event.get("customer_id");
            let content: String = event.get("raw_content");

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

            // Mark job as completed
            // Restore no-tenant bypass for job updates if needed, or leave tenant bound
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
