use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use sqlx::Row;
use chrono::{DateTime, Utc};
use std::collections::HashMap;




pub struct SipDB {
    pool: PgPool,
    org_id: String,
}

impl SipDB {
    pub fn new(pool: PgPool, org_id: String) -> Self {
        SipDB {
            pool,
            org_id,
        }
    }











    pub async fn prune_stale_missions(&self, age_threshold: chrono::Duration) -> Result<(), sqlx::Error> {
        let stuck_threshold = Utc::now() - chrono::Duration::hours(1);
        let fail_threshold = Utc::now() - age_threshold;
        
        // 1. Mark stagnant PENDING missions as STUCK after 1 hour
        sqlx::query("UPDATE agent_missions SET status = 'STUCK' WHERE (status = 'PENDING' OR status = 'BURSTING') AND updated_at < $1 AND organization_id = $2")
            .bind(stuck_threshold)
            .bind(&self.org_id)
            .execute(&self.pool)
            .await?;
            
        // 1b. Immediately requeue STUCK missions
        sqlx::query("UPDATE agent_missions SET status = 'PENDING', updated_at = CURRENT_TIMESTAMP WHERE status = 'STUCK' AND organization_id = $1")
            .bind(&self.org_id)
            .execute(&self.pool)
            .await?;
            
        // 2. Mark missions as FAILED if they exceed the absolute age threshold
        sqlx::query("UPDATE agent_missions SET status = 'FAILED' WHERE (status = 'PENDING' OR status = 'STUCK' OR status = 'BURSTING') AND created_at < $1 AND organization_id = $2")
            .bind(fail_threshold)
            .bind(&self.org_id)
            .execute(&self.pool)
            .await?;
            
        // 3. Remove COMPLETED, or very old FAILED missions
        sqlx::query("WITH cte AS (SELECT id FROM agent_missions WHERE (status = 'COMPLETED' OR ((status = 'FAILED' OR status = 'STUCK' OR status = 'BURSTING') AND created_at < $1)) AND organization_id = $2 LIMIT 1000) DELETE FROM agent_missions WHERE id IN (SELECT id FROM cte)")
            .bind(fail_threshold)
            .bind(&self.org_id)
            .execute(&self.pool)
            .await?;
            
        Ok(())
    }






    pub async fn upsert_mission(&self, mission_id: &str, status: &str, payload: &str, force_local: bool) -> Result<(), sqlx::Error> {
        let mut tx = self.pool.begin().await?;

        let row = sqlx::query("SELECT id FROM agent_missions WHERE id = $1 AND organization_id = $2 FOR UPDATE SKIP LOCKED")
            .bind(mission_id)
            .bind(&self.org_id)
            .fetch_optional(&mut *tx)
            .await?;

        if let Some(r) = row {
            let existing_id: String = r.get("id");
            if !existing_id.is_empty() && force_local {
                sqlx::query("UPDATE agent_missions SET status = $1, payload = $2, updated_at = CURRENT_TIMESTAMP WHERE id = $3 AND organization_id = $4")
                    .bind(status)
                    .bind(payload)
                    .bind(mission_id)
                    .bind(&self.org_id)
                    .execute(&mut *tx)
                    .await?;
            }
        } else {
            let row_check = sqlx::query("SELECT id FROM agent_missions WHERE id = $1 AND organization_id = $2")
                .bind(mission_id)
                .bind(&self.org_id)
                .fetch_optional(&mut *tx)
                .await?;

            if let Some(_) = row_check {
                 if force_local {
                     sqlx::query("UPDATE agent_missions SET status = $1, payload = $2, updated_at = CURRENT_TIMESTAMP WHERE id = $3 AND organization_id = $4")
                         .bind(status)
                         .bind(payload)
                         .bind(mission_id)
                         .bind(&self.org_id)
                         .execute(&mut *tx)
                         .await?;
                 }
            } else {
                 sqlx::query("INSERT INTO agent_missions (id, status, payload, created_at, updated_at, organization_id) VALUES ($1, $2, $3, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP, $4) ON CONFLICT(id) DO NOTHING")
                     .bind(mission_id)
                     .bind(status)
                     .bind(payload)
                     .bind(&self.org_id)
                     .execute(&mut *tx)
                     .await?;
            }
        }

        tx.commit().await?;
        Ok(())
    }
}
