use sqlx::PgPool;
use uuid::Uuid;
use super::cro::CroEngine;
use std::time::Duration;

pub async fn start_cro_evaluation_loop(pool: PgPool) {
    let engine = CroEngine::new(pool.clone());

    loop {
        if let Ok(tenants) = sqlx::query_as::<_, (Uuid,)>("SELECT DISTINCT tenant_id FROM cro_experiments WHERE status = 'running'")
        .fetch_all(&pool)
        .await {
            for (tenant_id,) in tenants {
                if let Err(e) = engine.evaluate_experiments(tenant_id).await {
                    tracing::error!("Error evaluating CRO experiments for tenant {}: {:?}", tenant_id, e);
                }
            }
        }

        tokio::time::sleep(Duration::from_secs(3600)).await;
    }
}
