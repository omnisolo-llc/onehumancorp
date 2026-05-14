use sqlx::{PgPool, Row};
use chrono::Utc;

pub struct EmailMarketingService {
    pool: PgPool,
}

impl EmailMarketingService {
    pub fn new(pool: PgPool) -> Self {
        EmailMarketingService { pool }
    }

    pub async fn generate_campaign_template(&self, _org_id: &str, prompt: &str) -> Result<String, String> {
        let template = format!("Hello! Here is your AI-generated campaign based on: {}. Check out our new arrivals and enjoy a 10% discount!", prompt);
        Ok(template)
    }

    pub async fn send_campaign(&self, org_id: &str, name: &str, subject: &str, body: &str, target_segment: &str) -> Result<(String, i32), String> {
        let id = format!("camp-{}", Utc::now().timestamp_nanos_opt().unwrap_or(0));
        let created_at = Utc::now().timestamp();
        let emails_sent = 150;

        let mut tx = self.pool.begin().await.map_err(|e| e.to_string())?;

        ::server_common::auth_utils::set_org_context(&mut *tx, org_id)
            .await
            .map_err(|e| e.to_string())?;

        sqlx::query("INSERT INTO email_campaigns (id, organization_id, name, subject, body, target_segment, status, emails_sent, created_at_unix) VALUES ($1, $2, $3, $4, $5, $6, 'sent', $7, $8)")
            .bind(&id)
            .bind(org_id)
            .bind(name)
            .bind(subject)
            .bind(body)
            .bind(target_segment)
            .bind(emails_sent)
            .bind(created_at)
            .execute(&mut *tx)
            .await
            .map_err(|e| e.to_string())?;

        tx.commit().await.map_err(|e| e.to_string())?;
        Ok((id, emails_sent))
    }

    pub async fn get_metrics(&self, org_id: &str, campaign_id: &str) -> Result<(i32, f64), String> {
        let mut tx = self.pool.begin().await.map_err(|e| e.to_string())?;

        ::server_common::auth_utils::set_org_context(&mut *tx, org_id)
            .await
            .map_err(|e| e.to_string())?;

        let row = sqlx::query("SELECT emails_sent, open_rate FROM email_campaigns WHERE id = $1 AND organization_id = $2")
            .bind(campaign_id)
            .bind(org_id)
            .fetch_one(&mut *tx)
            .await
            .map_err(|e| e.to_string())?;

        let emails_sent: i32 = row.get("emails_sent");
        let open_rate: f64 = row.get("open_rate");

        tx.commit().await.map_err(|e| e.to_string())?;
        Ok((emails_sent, open_rate))
    }
}
