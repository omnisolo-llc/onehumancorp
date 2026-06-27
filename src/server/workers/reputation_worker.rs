use tracing::{info, error};
use tokio::time::{sleep, Duration};
use sqlx::PgPool;
use sqlx::Row;
use uuid::Uuid;

pub struct ReputationWorker {
    db: PgPool,
}

impl ReputationWorker {
    pub fn new(db: PgPool) -> Self {
        Self { db }
    }

    pub async fn run(&self) {
        info!("ReputationWorker started.");
        loop {
            if let Err(e) = self.process_pending_reviews().await {
                error!("Error processing reputation reviews: {}", e);
            }
            if let Err(e) = self.poll_completed_bookings().await {
                error!("Error polling completed bookings: {}", e);
            }
            sleep(Duration::from_secs(60)).await;
        }
    }

    async fn poll_completed_bookings(&self) -> Result<(), Box<dyn std::error::Error>> {
        let pool = &self.db;
        // Find bookings completed in the last hour that aren't already tracked
        let completed_bookings = sqlx::query(
            r#"
            SELECT b.id, b.tenant_id, b.customer_id
            FROM bookings b
            LEFT JOIN reputation_reviews r ON b.id = r.booking_id
            WHERE b.status = 'completed'
            AND r.id IS NULL
            AND b.updated_at >= NOW() - INTERVAL '1 hour'
            "#
        )
        .fetch_all(pool)
        .await?;

        for row in completed_bookings {
            let booking_id: String = row.get("id");
            let tenant_id: String = row.get("tenant_id");
            // Customer might be null for walk-ins
            let customer_id_opt: Option<uuid::Uuid> = row.get("customer_id");

            if let Some(customer_id) = customer_id_opt {
                let id = format!("rep_{}", Uuid::new_v4());
                sqlx::query(
                    r#"
                    INSERT INTO reputation_reviews (id, tenant_id, customer_id, booking_id, status)
                    VALUES ($1, $2, $3, $4, 'pending')
                    ON CONFLICT DO NOTHING
                    "#
                )
                .bind(&id)
                .bind(&tenant_id)
                .bind(&customer_id.to_string())
                .bind(&booking_id)
                .execute(pool)
                .await?;
                info!("Queued review pulse check for booking {}", booking_id);
            }
        }
        Ok(())
    }

    async fn process_pending_reviews(&self) -> Result<(), Box<dyn std::error::Error>> {
        let pool = &self.db;

        // Find reviews in 'pending' status where time since order complete > delay
        // We'll approximate delay by checking if the booking was completed > delay_hours ago
        let pending_reviews = sqlx::query(
            r#"
            SELECT r.id, r.tenant_id, r.customer_id, s.auto_request_enabled, s.delay_hours
            FROM reputation_reviews r
            JOIN reputation_settings s ON r.tenant_id = s.tenant_id
            LEFT JOIN bookings b ON r.booking_id = b.id
            WHERE r.status = 'pending'
            AND s.auto_request_enabled = true
            AND (b.id IS NULL OR b.updated_at <= NOW() - (s.delay_hours || ' hours')::interval)
            "#
        )
        .fetch_all(pool)
        .await?;

        for row in pending_reviews {
            let id: String = row.get("id");
            let tenant_id: String = row.get("tenant_id");
            let customer_id: String = row.get("customer_id");

            // Mark as pulse_sent
            sqlx::query(
                "UPDATE reputation_reviews SET status = 'pulse_sent', pulse_sent_at = NOW() WHERE id = $1"
            )
            .bind(&id)
            .execute(pool)
            .await?;

            info!("Sent review pulse check for customer {} in tenant {}", customer_id, tenant_id);
            // Simulate sending SMS here
        }

        Ok(())
    }

    // Function to handle incoming customer response (simulated SMS reply)
    pub async fn handle_customer_reply(&self, tenant_id: &str, customer_id: &str, reply_text: &str) -> Result<(), Box<dyn std::error::Error>> {
        let pool = &self.db;

        // Find active pulse check
        let review: Option<sqlx::postgres::PgRow> = sqlx::query(
            "SELECT id FROM reputation_reviews WHERE tenant_id = $1 AND customer_id = $2 AND status = 'pulse_sent'"
        )
        .bind(tenant_id)
        .bind(customer_id)
        .fetch_optional(pool)
        .await?;

        if let Some(row) = review {
            let id: String = row.get("id");

            // Normally would use LLM here (Gemini integration stub)
            let rating = self.analyze_sentiment_with_llm(reply_text).await.unwrap_or(0);

            let sentiment = if rating >= 4 {
                "positive"
            } else if rating > 0 {
                "negative"
            } else {
                "neutral"
            };

            let mut status = "replied";
            if sentiment == "positive" {
                status = "public_prompted";
                info!("Customer {} rated {}. Prompting for public review.", customer_id, rating);
            } else if sentiment == "negative" {
                status = "owner_escalated";
                info!("Customer {} rated {}. Escalating to owner.", customer_id, rating);

                let task_id = format!("task_{}", Uuid::new_v4().to_string());
                sqlx::query(
                    r#"
                    INSERT INTO swarm_tasks (id, tenant_id, title, description, priority, status)
                    VALUES ($1, $2, $3, $4, $5, $6)
                    "#
                )
                .bind(&task_id)
                .bind(tenant_id)
                .bind(format!("Follow up on negative review ({}/5 stars)", rating))
                .bind("Action needed: Customer left a poor review. Draft Reply: 'I'll be out tomorrow to fix this'.")
                .bind("P1")
                .bind("pending")
                .execute(pool)
                .await?;

                sqlx::query(
                    "UPDATE reputation_reviews SET escalated_task_id = $1 WHERE id = $2"
                )
                .bind(&task_id)
                .bind(&id)
                .execute(pool)
                .await?;
            }

            sqlx::query(
                "UPDATE reputation_reviews SET status = $1, rating = $2, sentiment = $3, feedback_text = $4 WHERE id = $5"
            )
            .bind(status)
            .bind(if rating > 0 { Some(rating) } else { None })
            .bind(sentiment)
            .bind(reply_text)
            .bind(&id)
            .execute(pool)
            .await?;
        }

        Ok(())
    }

    async fn analyze_sentiment_with_llm(&self, text: &str) -> Result<i32, Box<dyn std::error::Error>> {
        // Stub for Gemini Pro LLM sentiment analysis
        // In reality, this would call out to Gemini via HTTP or a library
        // and ask "Score the following text as a 1-5 rating: <text>"
        let text_lower = text.to_lowercase();
        if text_lower.contains("great") || text_lower.contains("excellent") || text_lower.contains("good") {
            Ok(5)
        } else if text_lower.contains("bad") || text_lower.contains("terrible") || text_lower.contains("poor") {
            Ok(1)
        } else {
            Ok(text.trim().parse::<i32>().unwrap_or(0))
        }
    }
}
