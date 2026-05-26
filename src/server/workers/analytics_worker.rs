use std::sync::Arc;
use crate::db::DB;
use chrono::Utc;

pub struct AnalyticsWorker {
    db: Arc<DB>,
}

impl AnalyticsWorker {
    pub fn new(db: Arc<DB>) -> Self {
        Self { db }
    }

    pub fn start(&self) {
        let db = self.db.clone();
        tokio::spawn(async move {
            loop {
                if let Err(e) = Self::run_once(&db).await {
                    tracing::error!("Analytics worker error: {}", e);
                }
                tokio::time::sleep(tokio::time::Duration::from_secs(3600)).await;
            }
        });
    }

    async fn run_once(db: &Arc<DB>) -> Result<(), Box<dyn std::error::Error>> {
        let yesterday = Utc::now().naive_utc().date() - chrono::Duration::days(1);

        let tenants = sqlx::query!("SELECT DISTINCT tenant_id FROM business_events WHERE DATE(occurred_at) = $1", yesterday)
            .fetch_all(&db.pool)
            .await?;

        for tenant in tenants {
            let events = sqlx::query!("SELECT event_type, payload FROM business_events WHERE tenant_id = $1 AND DATE(occurred_at) = $2", tenant.tenant_id, yesterday)
                .fetch_all(&db.pool)
                .await?;

            let mut summary = String::new();
            if events.is_empty() {
                summary = "No significant activity was recorded yesterday.".to_string();
            } else {
                let mut checkouts = 0;
                let mut page_views = 0;
                for event in events {
                    if event.event_type == "checkout_completed" {
                        checkouts += 1;
                    } else if event.event_type == "page_view" {
                        page_views += 1;
                    }
                }

                // Integrate with LLM service instead of hardcoded rules
                let prompt = format!("You are a friendly business advisor. I am a small business owner. Yesterday, my business had {} page views and {} checkouts. Write a very brief (1-2 sentences) plain-language summary or suggestion for me. Do not use complex jargon. Be encouraging.", page_views, checkouts);

                let system_prompt = "You are a helpful business assistant for small business owners.".to_string();

                // Fire and wait for LLM
                match crate::services::autodream_pipeline::execute_llm_prompt("gpt-4o-mini", &system_prompt, &prompt).await {
                    Ok(llm_resp) => {
                        summary = llm_resp;
                    },
                    Err(e) => {
                        tracing::warn!("LLM generation failed for daily briefing: {}, falling back to basic string", e);
                        if checkouts > 0 {
                            summary = format!("Great news! You had {} checkouts yesterday from {} page views. Keep up the good work!", checkouts, page_views);
                        } else if page_views > 0 {
                            summary = format!("Your store had {} page views yesterday, but no checkouts. Want to offer a 10% discount to those who looked?", page_views);
                        } else {
                            summary = "Your store had some background activity yesterday.".to_string();
                        }
                    }
                }
            }

            let id = uuid::Uuid::new_v4().to_string();
            let _ = sqlx::query!("INSERT INTO daily_briefings (id, tenant_id, plain_language_summary, briefing_date) VALUES ($1, $2, $3, $4)",
                id, tenant.tenant_id, summary, yesterday)
                .execute(&db.pool)
                .await;
        }

        Ok(())
    }
}
