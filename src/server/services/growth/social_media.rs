use sqlx::PgPool;
use chrono::Utc;
use uuid::Uuid;
use tonic::Status;
use ::server_ohc::orchestration::{SocialPost, CreateSocialPostRequest};

pub async fn get_posts(pool: &PgPool, org_id: &str) -> Result<Vec<SocialPost>, Status> {
    let rows = sqlx::query("SELECT id, platform, content, media_urls, status, scheduled_at, published_at FROM social_posts WHERE tenant_id = $1")
        .bind(org_id)
        .fetch_all(pool)
        .await
        .map_err(|e| Status::internal(e.to_string()))?;

    let posts = rows.into_iter().map(|row| {
        use sqlx::Row;
        SocialPost {
            id: row.get("id"),
            platform: row.get("platform"),
            content: row.get("content"),
            media_urls: row.get("media_urls"),
            status: row.get("status"),
            scheduled_at_unix: row.get::<Option<chrono::DateTime<Utc>>, _>("scheduled_at").map(|t| t.timestamp()).unwrap_or(0),
            published_at_unix: row.get::<Option<chrono::DateTime<Utc>>, _>("published_at").map(|t| t.timestamp()).unwrap_or(0),
        }
    }).collect();

    Ok(posts)
}

pub async fn create_post(pool: &PgPool, org_id: &str, req: CreateSocialPostRequest) -> Result<SocialPost, Status> {
    let id = format!("post-{}", Uuid::new_v4());
    let scheduled_at = if req.scheduled_at_unix > 0 {
        Some(chrono::DateTime::from_timestamp(req.scheduled_at_unix, 0).unwrap_or_else(|| Utc::now()))
    } else {
        None
    };

    sqlx::query("INSERT INTO social_posts (id, tenant_id, platform, content, media_urls, status, scheduled_at) VALUES ($1, $2, $3, $4, $5, 'DRAFT', $6)")
        .bind(&id)
        .bind(org_id)
        .bind(&req.platform)
        .bind(&req.content)
        .bind(&req.media_urls)
        .bind(scheduled_at)
        .execute(pool)
        .await
        .map_err(|e| Status::internal(e.to_string()))?;

    Ok(SocialPost {
        id,
        platform: req.platform,
        content: req.content,
        media_urls: req.media_urls,
        status: "DRAFT".to_string(),
        scheduled_at_unix: req.scheduled_at_unix,
        published_at_unix: 0,
    })
}
