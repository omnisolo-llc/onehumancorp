use crate::orchestration::queue::OHCJob;
use crate::orchestration::queue::OHCJobQueue;
use sqlx::PgPool;
use tracing::{error, info, instrument};
use uuid::Uuid;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct DispatchDeliveryPayload {
    pub organization_id: String,
    pub order_id: String,
    pub pickup_location_lat: f64,
    pub pickup_location_lng: f64,
    pub delivery_location_lat: f64,
    pub delivery_location_lng: f64,
}

pub struct DeliveryDispatchWorker {
    pool: PgPool,
}

use crate::orchestration::queue::worker_pool::JobHandler;

impl DeliveryDispatchWorker {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    #[instrument(skip(self))]
    pub async fn process_job(&self, job: OHCJob) -> Result<(), String> {
        let payload: DispatchDeliveryPayload = serde_json::from_value(job.payload.clone())
            .map_err(|e| format!("Failed to parse payload: {}", e))?;

        info!("Processing delivery dispatch for order {}", payload.order_id);

        let row = sqlx::query(
            r#"
            SELECT flat_fee_cents, min_order_value_cents
            FROM delivery_zones
            WHERE organization_id = $1
            "#,
        )
        .bind(&payload.organization_id)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| format!("Db error looking up delivery zone: {}", e))?;

        let payout_cents = match row {
            Some(r) => r.get::<i64, _>("flat_fee_cents"),
            None => 500, // Default fallback $5.00 if no zone configured
        };

        let job_id = Uuid::new_v4();

        sqlx::query(
            r#"
            INSERT INTO delivery_jobs (
                id, organization_id, order_id, status,
                pickup_location, delivery_location, payout_cents
            )
            VALUES (
                $1, $2, $3, 'AVAILABLE',
                ST_SetSRID(ST_MakePoint($4, $5), 4326),
                ST_SetSRID(ST_MakePoint($6, $7), 4326),
                $8
            )
            "#,
        )
        .bind(job_id)
        .bind(&payload.organization_id)
        .bind(&payload.order_id)
        .bind(payload.pickup_location_lng)
        .bind(payload.pickup_location_lat)
        .bind(payload.delivery_location_lng)
        .bind(payload.delivery_location_lat)
        .bind(payout_cents)
        .execute(&self.pool)
        .await
        .map_err(|e| format!("Db error inserting delivery job: {}", e))?;

        info!("Created available delivery job {} with payout {}", job_id, payout_cents);

        Ok(())
    }
}

impl JobHandler for DeliveryDispatchWorker {
    fn handle(&self, job: OHCJob) -> tokio::task::JoinHandle<Result<(), String>> {
        let pool = self.pool.clone();
        tokio::spawn(async move {
            let worker = DeliveryDispatchWorker::new(pool);
            worker.process_job(job).await
        })
    }
}
