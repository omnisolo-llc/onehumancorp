use sqlx::PgPool;
use chrono::Utc;
use uuid::Uuid;
use tonic::Status;
use ::server_ohc::orchestration::{EmailCampaign, CreateEmailCampaignRequest};

pub async fn get_campaigns(pool: &PgPool, org_id: &str) -> Result<Vec<EmailCampaign>, Status> {
    let rows = sqlx::query("SELECT id, title, subject, content_html, status, total_recipients, open_count, click_count, scheduled_at, sent_at FROM email_campaigns WHERE tenant_id = $1")
        .bind(org_id)
        .fetch_all(pool)
        .await
        .map_err(|e| Status::internal(e.to_string()))?;

    let campaigns = rows.into_iter().map(|row| {
        use sqlx::Row;
        EmailCampaign {
            id: row.get("id"),
            title: row.get("title"),
            subject: row.get("subject"),
            content_html: row.get("content_html"),
            status: row.get("status"),
            total_recipients: row.get("total_recipients"),
            open_count: row.get("open_count"),
            click_count: row.get("click_count"),
            scheduled_at_unix: row.get::<Option<chrono::DateTime<Utc>>, _>("scheduled_at").map(|t| t.timestamp()).unwrap_or(0),
            sent_at_unix: row.get::<Option<chrono::DateTime<Utc>>, _>("sent_at").map(|t| t.timestamp()).unwrap_or(0),
        }
    }).collect();

    Ok(campaigns)
}

pub async fn create_campaign(pool: &PgPool, org_id: &str, req: CreateEmailCampaignRequest) -> Result<EmailCampaign, Status> {
    let id = format!("camp-{}", Uuid::new_v4());
    let scheduled_at = if req.scheduled_at_unix > 0 {
        Some(chrono::DateTime::from_timestamp(req.scheduled_at_unix, 0).unwrap_or_else(|| Utc::now()))
    } else {
        None
    };

    sqlx::query("INSERT INTO email_campaigns (id, tenant_id, title, subject, content_html, status, scheduled_at) VALUES ($1, $2, $3, $4, $5, 'DRAFT', $6)")
        .bind(&id)
        .bind(org_id)
        .bind(&req.title)
        .bind(&req.subject)
        .bind(&req.content_html)
        .bind(scheduled_at)
        .execute(pool)
        .await
        .map_err(|e| Status::internal(e.to_string()))?;

    Ok(EmailCampaign {
        id,
        title: req.title,
        subject: req.subject,
        content_html: req.content_html,
        status: "DRAFT".to_string(),
        total_recipients: 0,
        open_count: 0,
        click_count: 0,
        scheduled_at_unix: req.scheduled_at_unix,
        sent_at_unix: 0,
    })
}
