#[cfg(test)]
mod tests {
    use super::super::*;
    use chrono::Utc;
    use sqlx::Row;

    #[tokio::test]
    async fn test_prune_stale_missions_marks_super_stale_as_stale() {
        let database_url = match std::env::var("OHC_DATABASE_URL") {
            Ok(val) => val,
            Err(_) => {
                tracing::warn!("Skipping test_prune_stale_missions_marks_super_stale_as_stale: OHC_DATABASE_URL not set");
                return;
            }
        };

        if let Ok(pool) = sqlx::postgres::PgPoolOptions::new()
            .connect(&database_url)
            .await
        {
            let sip_db = sip::SipDB::new(pool.clone(), "test_org_stale".to_string());

            sqlx::query(
                "CREATE TABLE IF NOT EXISTS agent_missions (
                    id TEXT PRIMARY KEY,
                    status TEXT NOT NULL,
                    payload TEXT NOT NULL,
                    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
                    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
                    tenant_id TEXT,
                    mission_log TEXT
                )"
            )
            .execute(&pool)
            .await
            .unwrap();

            let super_old_time = Utc::now() - chrono::Duration::hours(25);
            sqlx::query("INSERT INTO agent_missions (id, status, payload, tenant_id, created_at, updated_at) VALUES ($1, $2, $3, $4, $5, $6) ON CONFLICT DO NOTHING")
                .bind("super_stale_mission")
                .bind("PENDING")
                .bind("{}")
                .bind("test_org_stale")
                .bind(super_old_time.naive_utc())
                .bind(Utc::now().naive_utc())
                .execute(&pool)
                .await
                .unwrap();

            let res = sip_db.prune_stale_missions(chrono::Duration::hours(48)).await;
            assert!(res.is_ok());

            let row = sqlx::query("SELECT status FROM agent_missions WHERE id = 'super_stale_mission'")
                .fetch_one(&pool)
                .await
                .unwrap();
            let status: String = row.get("status");
            assert_eq!(status, "STALE");

            sqlx::query("DELETE FROM agent_missions WHERE tenant_id = 'test_org_stale'").execute(&pool).await.unwrap();
        }
    }
}
