use std::sync::Arc;
use crate::db::DB;
use std::time::Duration;
use sqlx::Row;

pub struct EmailCampaignWorker {
    pub db: Arc<DB>,
    pub poll_interval: Duration,
}

impl EmailCampaignWorker {
    pub fn new(db: Arc<DB>) -> Self {
        Self {
            db,
            poll_interval: Duration::from_secs(10),
        }
    }

    pub fn start(&self) {
        let db = self.db.clone();
        let interval_duration = self.poll_interval;
        tokio::spawn(async move {
            let mut interval = tokio::time::interval(interval_duration);
            loop {
                interval.tick().await;
                let _ = Self::poll(&db).await;
            }
        });
    }

    pub async fn poll(db: &Arc<DB>) -> Result<(), String> {
        // Find campaigns scheduled to be sent
        match &db.store {
            crate::db::DbStore::Postgres => {
                let campaigns = sqlx::query("SELECT id, tenant_id, title, subject, content_html FROM email_campaigns WHERE status = 'DRAFT' AND (scheduled_at IS NULL OR scheduled_at <= CURRENT_TIMESTAMP) LIMIT 5")
                    .fetch_all(&db.pool)
                    .await
                    .map_err(|e| e.to_string())?;

                for row in campaigns {
                    let id: String = row.get("id");
                    let tenant_id: String = row.get("tenant_id");
                    let title: String = row.get("title");
                    let subject: String = row.get("subject");
                    let content: String = row.get("content_html");

                    tracing::info!("Refining email campaign with AI: {} for tenant: {}", title, tenant_id);

                    let prompt = format!("Refine this email campaign for a small business. Title: {}. Subject: {}. Content: {}. Make it professional, high-conversion, and friendly. Output refined HTML only.", title, subject, content);

                    let mut refined_content = content;
                    if let Ok(mut client) = ::server_ohc::orchestration::hub_service_client::HubServiceClient::connect(std::env::var("OHC_HUB_URL").unwrap_or_else(|_| "http://127.0.0.1:8081".to_string())).await {
                        let reason_req = ::server_ohc::orchestration::ReasonRequest {
                            prompt,
                            from_agent_id: "Ambassador".into(),
                        };
                        if let Ok(res) = client.reason(tonic::Request::new(reason_req)).await {
                            refined_content = res.into_inner().content;
                        }
                    }

                    sqlx::query("UPDATE email_campaigns SET status = 'SENT', content_html = $1, sent_at = CURRENT_TIMESTAMP WHERE id = $2")
                        .bind(&refined_content)
                        .bind(&id)
                        .execute(&db.pool)
                        .await
                        .map_err(|e| e.to_string())?;
                }
            },
            crate::db::DbStore::Sqlite(pool) => {
                let campaigns = sqlx::query("SELECT id, tenant_id, title, subject, content_html FROM email_campaigns WHERE status = 'DRAFT' AND (scheduled_at IS NULL OR scheduled_at <= CURRENT_TIMESTAMP) LIMIT 5")
                    .fetch_all(pool)
                    .await
                    .map_err(|e| e.to_string())?;

                for row in campaigns {
                    let id: String = row.get("id");
                    let tenant_id: String = row.get("tenant_id");
                    let title: String = row.get("title");
                    let subject: String = row.get("subject");
                    let content: String = row.get("content_html");

                    tracing::info!("Refining email campaign with AI: {} for tenant: {}", title, tenant_id);

                    let prompt = format!("Refine this email campaign for a small business. Title: {}. Subject: {}. Content: {}. Make it professional, high-conversion, and friendly. Output refined HTML only.", title, subject, content);

                    let mut refined_content = content;
                    if let Ok(mut client) = ::server_ohc::orchestration::hub_service_client::HubServiceClient::connect(std::env::var("OHC_HUB_URL").unwrap_or_else(|_| "http://127.0.0.1:8081".to_string())).await {
                        let reason_req = ::server_ohc::orchestration::ReasonRequest {
                            prompt,
                            from_agent_id: "Ambassador".into(),
                        };
                        if let Ok(res) = client.reason(tonic::Request::new(reason_req)).await {
                            refined_content = res.into_inner().content;
                        }
                    }

                    sqlx::query("UPDATE email_campaigns SET status = 'SENT', content_html = ?, sent_at = CURRENT_TIMESTAMP WHERE id = ?")
                        .bind(&refined_content)
                        .bind(&id)
                        .execute(pool)
                        .await
                        .map_err(|e| e.to_string())?;
                }
            }
        };

        Ok(())
    }
}
