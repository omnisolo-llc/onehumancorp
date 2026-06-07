use crate::domain::repository::models::{Campaign, CampaignAsset};
use sqlx::{PgPool, Error};
use uuid::Uuid;

pub struct CampaignRepository {
    pool: PgPool,
}

impl CampaignRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn create_campaign(&self, campaign: &Campaign) -> Result<(), Error> {
        sqlx::query(
            r#"
            INSERT INTO campaigns (id, tenant_id, goal, status, start_time, end_time, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
            "#,
        )
        .bind(&campaign.id)
        .bind(&campaign.tenant_id)
        .bind(&campaign.goal)
        .bind(&campaign.status)
        .bind(&campaign.start_time)
        .bind(&campaign.end_time)
        .bind(&campaign.created_at)
        .bind(&campaign.updated_at)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    pub async fn get_campaign(&self, tenant_id: &str, id: &str) -> Result<Campaign, Error> {
        sqlx::query_as::<_, Campaign>(
            r#"
            SELECT * FROM campaigns WHERE tenant_id = $1 AND id = $2
            "#,
        )
        .bind(tenant_id)
        .bind(id)
        .fetch_one(&self.pool)
        .await
    }

    pub async fn update_campaign_status(&self, tenant_id: &str, id: &str, status: &str) -> Result<(), Error> {
        sqlx::query(
            r#"
            UPDATE campaigns SET status = $1, updated_at = CURRENT_TIMESTAMP WHERE tenant_id = $2 AND id = $3
            "#,
        )
        .bind(status)
        .bind(tenant_id)
        .bind(id)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    pub async fn add_asset(&self, asset: &CampaignAsset) -> Result<(), Error> {
        sqlx::query(
            r#"
            INSERT INTO campaign_assets (id, tenant_id, campaign_id, type, content_url, created_at)
            VALUES ($1, $2, $3, $4, $5, $6)
            "#,
        )
        .bind(&asset.id)
        .bind(&asset.tenant_id)
        .bind(&asset.campaign_id)
        .bind(&asset.r#type)
        .bind(&asset.content_url)
        .bind(&asset.created_at)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    pub async fn get_assets(&self, tenant_id: &str, campaign_id: &str) -> Result<Vec<CampaignAsset>, Error> {
        sqlx::query_as::<_, CampaignAsset>(
            r#"
            SELECT * FROM campaign_assets WHERE tenant_id = $1 AND campaign_id = $2
            "#,
        )
        .bind(tenant_id)
        .bind(campaign_id)
        .fetch_all(&self.pool)
        .await
    }

    pub async fn record_channel_execution(
        &self,
        tenant_id: &str,
        campaign_id: &str,
        channel: &str,
        metrics_sent: i32,
    ) -> Result<(), Error> {
        sqlx::query(
            r#"
            INSERT INTO channel_executions (id, tenant_id, campaign_id, channel, metrics_sent)
            VALUES ($1, $2, $3, $4, $5)
            "#,
        )
        .bind(Uuid::new_v4().to_string())
        .bind(tenant_id)
        .bind(campaign_id)
        .bind(channel)
        .bind(metrics_sent)
        .execute(&self.pool)
        .await?;

        Ok(())
    }
}
