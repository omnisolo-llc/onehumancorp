#[cfg(test)]
mod tests {
    use crate::db::{DB, DbStore};
    use chrono::{Utc, Duration};
    use sqlx::Row;
    use sqlx::sqlite::SqlitePoolOptions;
    use sqlx::postgres::PgPoolOptions;

    #[tokio::test]
    async fn test_mission_stuck_transition() {
        let pool = SqlitePoolOptions::new()
            .connect("sqlite::memory:")
            .await
            .unwrap();

        let db = DB {
            pool: PgPoolOptions::new().connect_lazy("postgres://localhost/dummy").unwrap(),
            store: DbStore::Sqlite(pool.clone()),
        };

        db.run_migrations().await.unwrap();

        let stuck_id = "test_stuck";
        // 40 mins ago is > 30 mins (timeout_secs/2)
        let old_time = Utc::now() - Duration::minutes(40);

        sqlx::query("INSERT INTO agent_missions (id, status, payload, updated_at, tenant_id) VALUES (?, 'PENDING', '{}', ?, 'system')")
            .bind(stuck_id)
            .bind(old_time.to_rfc3339())
            .execute(&pool)
            .await
            .unwrap();

        let pending_id = "test_pending";
        sqlx::query("INSERT INTO agent_missions (id, status, payload, updated_at, tenant_id) VALUES (?, 'PENDING', '{}', ?, 'system')")
            .bind(pending_id)
            .bind(Utc::now().to_rfc3339())
            .execute(&pool)
            .await
            .unwrap();

        // Run cleanup (threshold 3600s, so stuck at 1800s)
        db.cleanup_stagnant_missions(3600).await.unwrap();

        let status_stuck: String = sqlx::query("SELECT status FROM agent_missions WHERE id = ?").bind(stuck_id).fetch_one(&pool).await.unwrap().get(0);
        let status_pending: String = sqlx::query("SELECT status FROM agent_missions WHERE id = ?").bind(pending_id).fetch_one(&pool).await.unwrap().get(0);

        assert_eq!(status_stuck, "STUCK");
        assert_eq!(status_pending, "PENDING");

        // Now make it pass fail threshold (70 mins > 60 mins)
        let very_old_time = Utc::now() - Duration::minutes(70);
        sqlx::query("UPDATE agent_missions SET updated_at = ? WHERE id = ?")
            .bind(very_old_time.to_rfc3339())
            .bind(stuck_id)
            .execute(&pool)
            .await
            .unwrap();

        db.cleanup_stagnant_missions(3600).await.unwrap();
        let status_failed: String = sqlx::query("SELECT status FROM agent_missions WHERE id = ?").bind(stuck_id).fetch_one(&pool).await.unwrap().get(0);
        assert_eq!(status_failed, "FAILED");
    }
}
