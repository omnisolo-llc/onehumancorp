use sqlx::{PgPool, Error};
use crate::services::loyalty::data_models::ReferralLinkage;
use rand::RngCore;

pub struct ReferralEngine {
    pool: PgPool,
}

impl ReferralEngine {
    pub fn new(pool: PgPool) -> Self {
        ReferralEngine { pool }
    }

    pub async fn generate_referral_code(&self, tenant_id: &str, customer_id: &str) -> Result<String, Error> {
        let mut rng = rand::thread_rng();
        let bytes: [u8; 4] = rng.next_u32().to_le_bytes();
        let code = hex::encode(bytes);

        let query = r#"
            INSERT INTO referral_codes (id, tenant_id, customer_id, referral_code)
            VALUES (gen_random_uuid()::text, $1, $2, $3)
            ON CONFLICT (referral_code) DO NOTHING
        "#;

        sqlx::query(query)
            .bind(tenant_id)
            .bind(customer_id)
            .bind(&code)
            .execute(&self.pool)
            .await?;

        Ok(code)
    }

    pub async fn get_referral_linkage(&self, tenant_id: &str, customer_id: &str) -> Result<Option<ReferralLinkage>, Error> {
        let linkage = sqlx::query_as!(
            ReferralLinkage,
            r#"
            SELECT customer_id, tenant_id, referral_code, 0 as referred_count
            FROM referral_codes
            WHERE tenant_id = $1 AND customer_id = $2
            "#,
            tenant_id,
            customer_id
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(linkage)
    }
}
