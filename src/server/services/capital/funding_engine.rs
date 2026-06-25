use sqlx::PgPool;
use sqlx::Row;

pub struct FundingOpportunity {
    pub id: String,
    pub tenant_id: String,
    pub grant_name: String,
    pub amount: f64,
    pub draft_proposal_text: Option<String>,
    pub status: String,
    pub deadline: Option<sqlx::types::chrono::DateTime<sqlx::types::chrono::Utc>>,
}

pub struct FundingEngine {
    pool: PgPool,
}

impl FundingEngine {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn get_opportunities(&self, tenant_id: &str) -> Result<Vec<FundingOpportunity>, sqlx::Error> {
        let rows = sqlx::query(
            "SELECT id, tenant_id, grant_name, CAST(amount AS DOUBLE PRECISION) as amount, draft_proposal_text, status, deadline FROM funding_opportunities WHERE tenant_id = $1"
        )
        .bind(tenant_id)
        .fetch_all(&self.pool)
        .await?;

        let mut opportunities = Vec::new();
        for row in rows {
            opportunities.push(FundingOpportunity {
                id: row.try_get("id")?,
                tenant_id: row.try_get("tenant_id")?,
                grant_name: row.try_get("grant_name")?,
                amount: row.try_get("amount")?,
                draft_proposal_text: row.try_get("draft_proposal_text")?,
                status: row.try_get("status")?,
                deadline: row.try_get("deadline")?,
            });
        }
        Ok(opportunities)
    }

    pub async fn approve_opportunity(&self, tenant_id: &str, opportunity_id: &str) -> Result<bool, sqlx::Error> {
        let result = sqlx::query(
            "UPDATE funding_opportunities SET status = 'Approved' WHERE id = $1 AND tenant_id = $2"
        )
        .bind(opportunity_id)
        .bind(tenant_id)
        .execute(&self.pool)
        .await?;

        Ok(result.rows_affected() > 0)
    }
}
