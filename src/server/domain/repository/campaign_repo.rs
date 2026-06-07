use crate::domain::repository::models::{Campaign, CampaignAsset, ChannelExecution, PromotionCode, LeadGenCampaign};
use sqlx::{PgPool, Error};

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

    pub async fn create_lead_gen_campaign(&self, campaign: &LeadGenCampaign) -> Result<(), Error> {
        sqlx::query(
            r#"
            INSERT INTO lead_gen_campaigns (id, tenant_id, budget, radius_miles, zip_code, status, created_at, updated_at)
            VALUES ($1::uuid, $2::uuid, $3, $4, $5, $6, $7, $8)
            "#,
        )
        .bind(uuid::Uuid::parse_str(&campaign.id).unwrap_or_default())
        .bind(uuid::Uuid::parse_str(&campaign.tenant_id).unwrap_or_default())
        .bind(&campaign.budget)
        .bind(campaign.radius_miles)
        .bind(&campaign.zip_code)
        .bind(&campaign.status)
        .bind(&campaign.created_at)
        .bind(&campaign.updated_at)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    pub async fn get_lead_gen_campaign(&self, tenant_id: &str, id: &str) -> Result<LeadGenCampaign, Error> {
        sqlx::query_as::<_, LeadGenCampaign>(
            r#"
            SELECT * FROM lead_gen_campaigns WHERE tenant_id = $1::uuid AND id = $2::uuid
            "#,
        )
        .bind(uuid::Uuid::parse_str(tenant_id).unwrap_or_default())
        .bind(uuid::Uuid::parse_str(id).unwrap_or_default())
        .fetch_one(&self.pool)
        .await
    }
}
