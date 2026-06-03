use crate::domain::repository::models::{Campaign, CampaignAsset, ChannelExecution, PromotionCode, WaitlistCampaign, PreOrderEntry};
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

    pub async fn create_waitlist_campaign(&self, campaign: &WaitlistCampaign) -> Result<(), Error> {
        sqlx::query(
            r#"
            INSERT INTO ohc_waitlist_campaigns (id, tenant_id, name, max_capacity, drops_at, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7)
            "#,
        )
        .bind(&campaign.id)
        .bind(&campaign.tenant_id)
        .bind(&campaign.name)
        .bind(&campaign.max_capacity)
        .bind(&campaign.drops_at)
        .bind(&campaign.created_at)
        .bind(&campaign.updated_at)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    pub async fn secure_pre_order(&self, entry: &PreOrderEntry) -> Result<(), Error> {
        let mut tx = self.pool.begin().await?;

        let current_count: i64 = sqlx::query_scalar::<_, i64>(
            r#"
            SELECT COUNT(*) FROM ohc_pre_order_entries
            WHERE waitlist_campaign_id = $1 AND status != 'CANCELLED'
            "#,
        )
        .bind(&entry.waitlist_campaign_id)
        .fetch_one(&mut *tx)
        .await?;

        let campaign: WaitlistCampaign = sqlx::query_as(
            r#"
            SELECT * FROM ohc_waitlist_campaigns
            WHERE id = $1 AND tenant_id = $2
            "#,
        )
        .bind(&entry.waitlist_campaign_id)
        .bind(&entry.tenant_id)
        .fetch_one(&mut *tx)
        .await?;

        if current_count >= campaign.max_capacity as i64 {
            tx.rollback().await?;
            return Err(sqlx::Error::RowNotFound); // Simulated capacity full error
        }

        sqlx::query(
            r#"
            INSERT INTO ohc_pre_order_entries (id, tenant_id, waitlist_campaign_id, customer_id, status, deposit_amount_cents, source, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
            "#,
        )
        .bind(&entry.id)
        .bind(&entry.tenant_id)
        .bind(&entry.waitlist_campaign_id)
        .bind(&entry.customer_id)
        .bind(&entry.status)
        .bind(&entry.deposit_amount_cents)
        .bind(&entry.source)
        .bind(&entry.created_at)
        .bind(&entry.updated_at)
        .execute(&mut *tx)
        .await?;

        tx.commit().await?;

        Ok(())
    }
}
