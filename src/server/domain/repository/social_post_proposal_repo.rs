use sqlx::{PgPool, Error};
use super::models::SocialPostProposal;

pub struct SocialPostProposalRepository {
    pool: PgPool,
}

impl SocialPostProposalRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn create_proposal(&self, proposal: SocialPostProposal) -> Result<(), Error> {
        sqlx::query(
            "INSERT INTO social_post_proposals (id, tenant_id, product_id, content, image_url, seo_alt_text, seo_meta_description, status, created_at_unix, updated_at_unix)
             VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)"
        )
        .bind(&proposal.id)
        .bind(&proposal.tenant_id)
        .bind(&proposal.product_id)
        .bind(&proposal.content)
        .bind(&proposal.image_url)
        .bind(&proposal.seo_alt_text)
        .bind(&proposal.seo_meta_description)
        .bind(&proposal.status)
        .bind(proposal.created_at_unix)
        .bind(proposal.updated_at_unix)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    pub async fn list_proposals(&self, tenant_id: &str, status: &str) -> Result<Vec<SocialPostProposal>, Error> {
        let proposals = sqlx::query_as::<_, SocialPostProposal>(
            "SELECT * FROM social_post_proposals WHERE tenant_id = $1 AND status = $2"
        )
        .bind(tenant_id)
        .bind(status)
        .fetch_all(&self.pool)
        .await?;

        Ok(proposals)
    }

    pub async fn get_proposal(&self, tenant_id: &str, proposal_id: &str) -> Result<Option<SocialPostProposal>, Error> {
        let proposal = sqlx::query_as::<_, SocialPostProposal>(
            "SELECT * FROM social_post_proposals WHERE tenant_id = $1 AND id = $2"
        )
        .bind(tenant_id)
        .bind(proposal_id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(proposal)
    }

    pub async fn update_status(&self, tenant_id: &str, proposal_id: &str, new_status: &str, updated_at_unix: i64) -> Result<(), Error> {
        sqlx::query(
            "UPDATE social_post_proposals SET status = $1, updated_at_unix = $2 WHERE tenant_id = $3 AND id = $4"
        )
        .bind(new_status)
        .bind(updated_at_unix)
        .bind(tenant_id)
        .bind(proposal_id)
        .execute(&self.pool)
        .await?;

        Ok(())
    }
}
