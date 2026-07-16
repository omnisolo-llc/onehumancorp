use sqlx::{PgPool, Row};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CustomerIdentity {
    pub id: Uuid,
    pub tenant_id: String,
    pub channel: String,
    pub identifier: String,
    pub verification_status: String,
    pub trust_score: i32,
    pub last_verified_at: Option<DateTime<Utc>>,
}

pub struct IdentityValidator {
    pool: PgPool,
}

impl IdentityValidator {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn validate_identity(&self, tenant_id: &str, channel: &str, identifier: &str) -> Result<CustomerIdentity, sqlx::Error> {
        let mut tx = self.pool.begin().await?;
        ::server_common::auth_utils::set_org_context(&mut *tx, tenant_id).await?;

        // Check if identity exists
        let row_opt = sqlx::query(
            "SELECT id, tenant_id, channel, identifier, verification_status, trust_score, last_verified_at
             FROM customer_identities
             WHERE tenant_id = $1 AND channel = $2 AND identifier = $3"
        )
        .bind(tenant_id)
        .bind(channel)
        .bind(identifier)
        .fetch_optional(&mut *tx)
        .await?;

        if let Some(row) = row_opt {
            let identity = CustomerIdentity {
                id: row.try_get("id")?,
                tenant_id: row.try_get("tenant_id")?,
                channel: row.try_get("channel")?,
                identifier: row.try_get("identifier")?,
                verification_status: row.try_get("verification_status")?,
                trust_score: row.try_get("trust_score")?,
                last_verified_at: row.try_get("last_verified_at")?,
            };

            // If flagged, maybe we create a fraud alert logic here or handle in the caller
            tx.commit().await?;
            return Ok(identity);
        }

        // Create new identity entry
        let new_id = Uuid::new_v4();
        let new_status = "pending";
        let new_score = 50; // default initial score

        sqlx::query(
            "INSERT INTO customer_identities (id, tenant_id, channel, identifier, verification_status, trust_score)
             VALUES ($1, $2, $3, $4, $5, $6)"
        )
        .bind(new_id)
        .bind(tenant_id)
        .bind(channel)
        .bind(identifier)
        .bind(new_status)
        .bind(new_score)
        .execute(&mut *tx)
        .await?;

        tx.commit().await?;

        Ok(CustomerIdentity {
            id: new_id,
            tenant_id: tenant_id.to_string(),
            channel: channel.to_string(),
            identifier: identifier.to_string(),
            verification_status: new_status.to_string(),
            trust_score: new_score,
            last_verified_at: None,
        })
    }
}
