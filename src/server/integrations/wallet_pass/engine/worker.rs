use std::sync::Arc;
use sqlx::PgPool;
use std::collections::HashMap;
use async_trait::async_trait;
use serde_json::Value;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::engine::builder::PassBuilder;
use crate::engine::signer::PassSigner;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OHCJob {
    pub id: String,
    pub tenant_id: String,
    pub job_type: String,
    pub payload: String,
    pub status: String,
    pub retry_count: i32,
    pub next_retry_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

pub trait JobHandler: Send + Sync {
    fn handle(&self, job: OHCJob) -> tokio::task::JoinHandle<Result<(), String>>;
}


pub struct WalletPassWorker {
    pub pool: Arc<PgPool>,
    pub cert_pem: String,
    pub key_pem: String,
    pub wwdr_pem: String,
}

impl WalletPassWorker {
    pub fn new(pool: Arc<PgPool>, cert_pem: String, key_pem: String, wwdr_pem: String) -> Self {
        Self { pool, cert_pem, key_pem, wwdr_pem }
    }
}

#[async_trait]
impl JobHandler for WalletPassWorker {
    fn handle(&self, job: OHCJob) -> tokio::task::JoinHandle<Result<(), String>> {
        let pool = self.pool.clone();
        let cert_pem = self.cert_pem.clone();
        let key_pem = self.key_pem.clone();
        let wwdr_pem = self.wwdr_pem.clone();

        tokio::spawn(async move {
            let payload: Value = serde_json::from_str(&job.payload).map_err(|e| e.to_string())?;
            let tenant_id = job.tenant_id.clone();

            // Extract necessary payload info
            let customer_id = payload.get("customer_id").and_then(|v| v.as_str()).unwrap_or("unknown").to_string();
            let pass_type = payload.get("pass_type").and_then(|v| v.as_str()).unwrap_or("generic").to_string();

            // Generate Pass Structure
            let mut builder = PassBuilder::new(
                "pass.com.onehumancorp.wallet".to_string(), // In production, read from tenant config
                "TEAMID123".to_string(),
                "OneHumanCorp".to_string(),
                "OHC Pass".to_string(),
                uuid::Uuid::new_v4().to_string(),
                &pass_type,
            )
            .with_colors("rgb(255, 255, 255)", "rgb(0, 0, 0)", "rgb(100, 100, 100)")
            .with_logo_text("OHC Store");

            if pass_type == "booking" {
                builder = builder
                    .add_primary_field("event", "Event", payload.get("event_name").and_then(|v| v.as_str()).unwrap_or("Appointment"))
                    .with_barcode(&customer_id);
            } else if pass_type == "loyalty" {
                builder = builder
                    .add_primary_field("points", "Points", payload.get("points").and_then(|v| v.as_str()).unwrap_or("0"))
                    .with_barcode(&customer_id);
            }

            let pass_json = builder.build().map_err(|e: serde_json::Error| e.to_string())?;

            let mut files = HashMap::new();
            files.insert("pass.json".to_string(), serde_json::to_vec(&pass_json).map_err(|e| e.to_string())?);
            // In a real implementation, we'd fetch tenant logos and icons here
            // files.insert("icon.png".to_string(), icon_data);

            let signer = PassSigner::new(cert_pem, key_pem, wwdr_pem);
            let _zip_data = signer.sign_and_zip(files)?;

            // Store in Database
            let pass_id = uuid::Uuid::new_v4().to_string();
            let mut tx = pool.begin().await.map_err(|e| e.to_string())?;

            // In a real implementation we would also upload zip_data to GCS/S3 here
            // For now, we save metadata

            // Important: We must not bypass RLS, so set the config
            ::server_common::auth_utils::set_org_context(&mut *tx, &tenant_id).await.map_err(|e| e.to_string())?;

            sqlx::query(
                "INSERT INTO wallet_passes (id, tenant_id, customer_id, pass_type, status, pass_data)
                 VALUES ($1, $2, $3, $4, 'active', $5)"
            )
            .bind(&pass_id)
            .bind(&tenant_id)
            .bind(&customer_id)
            .bind(&pass_type)
            .bind(&pass_json)
            .execute(&mut *tx)
            .await
            .map_err(|e| e.to_string())?;

            tx.commit().await.map_err(|e| e.to_string())?;

            tracing::info!("Successfully processed wallet pass {} for tenant {}", pass_id, tenant_id);
            Ok(())
        })
    }
}
