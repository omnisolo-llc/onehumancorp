use sqlx::{PgPool, Row};
use chrono::Utc;

pub struct SocialMediaService {
    pool: PgPool,
}

impl SocialMediaService {
    pub fn new(pool: PgPool) -> Self {
        SocialMediaService { pool }
    }

    pub async fn connect_platform(&self, org_id: &str, platform: &str) -> Result<String, String> {
        let id = format!("smp-{}", Utc::now().timestamp_nanos_opt().unwrap_or(0));
        let mut tx = self.pool.begin().await.map_err(|e| e.to_string())?;

        ::server_common::auth_utils::set_org_context(&mut *tx, org_id)
            .await
            .map_err(|e| e.to_string())?;

        sqlx::query("INSERT INTO social_media_profiles (id, organization_id, platform) VALUES ($1, $2, $3)")
            .bind(&id)
            .bind(org_id)
            .bind(platform)
            .execute(&mut *tx)
            .await
            .map_err(|e| e.to_string())?;

        tx.commit().await.map_err(|e| e.to_string())?;
        Ok(id)
    }

    pub async fn schedule_post(&self, org_id: &str, platform: &str, content: &str) -> Result<String, String> {
        let id = format!("post-{}", Utc::now().timestamp_nanos_opt().unwrap_or(0));
        let created_at = Utc::now().timestamp();
        let mut tx = self.pool.begin().await.map_err(|e| e.to_string())?;

        ::server_common::auth_utils::set_org_context(&mut *tx, org_id)
            .await
            .map_err(|e| e.to_string())?;

        sqlx::query("INSERT INTO social_media_posts (id, organization_id, platform, content, status, created_at_unix) VALUES ($1, $2, $3, $4, 'draft', $5)")
            .bind(&id)
            .bind(org_id)
            .bind(platform)
            .bind(content)
            .bind(created_at)
            .execute(&mut *tx)
            .await
            .map_err(|e| e.to_string())?;

        tx.commit().await.map_err(|e| e.to_string())?;
        Ok(id)
    }

    pub async fn edit_draft(&self, org_id: &str, post_id: &str, new_content: &str) -> Result<bool, String> {
        let mut tx = self.pool.begin().await.map_err(|e| e.to_string())?;

        ::server_common::auth_utils::set_org_context(&mut *tx, org_id)
            .await
            .map_err(|e| e.to_string())?;

        let res = sqlx::query("UPDATE social_media_posts SET content = $1 WHERE id = $2 AND organization_id = $3 AND status = 'draft'")
            .bind(new_content)
            .bind(post_id)
            .bind(org_id)
            .execute(&mut *tx)
            .await
            .map_err(|e| e.to_string())?;

        tx.commit().await.map_err(|e| e.to_string())?;
        Ok(res.rows_affected() > 0)
    }

    pub async fn approve_post(&self, org_id: &str, post_id: &str) -> Result<bool, String> {
        let mut tx = self.pool.begin().await.map_err(|e| e.to_string())?;

        ::server_common::auth_utils::set_org_context(&mut *tx, org_id)
            .await
            .map_err(|e| e.to_string())?;

        let res = sqlx::query("UPDATE social_media_posts SET status = 'approved', posted_at = CURRENT_TIMESTAMP WHERE id = $1 AND organization_id = $2")
            .bind(post_id)
            .bind(org_id)
            .execute(&mut *tx)
            .await
            .map_err(|e| e.to_string())?;

        tx.commit().await.map_err(|e| e.to_string())?;
        Ok(res.rows_affected() > 0)
    }
}
