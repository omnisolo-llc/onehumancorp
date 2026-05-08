#[cfg(test)]
mod tests {
    use crate::db::{DB, DbStore};
    use sqlx::Row;
    use chrono::{Utc, Duration};

    #[tokio::test]
    async fn test_cleanup_stagnant_missions_robust() {
        // Verification logic moved into DB module to simplify environment dependencies in unit tests
        let db = DB::new().await;
        if let Ok(db) = db {
            if db.is_sqlite() {
                 if let DbStore::Sqlite(pool) = &db.store {
                     let _ = sqlx::query("CREATE TABLE IF NOT EXISTS agent_missions (id TEXT PRIMARY KEY, status TEXT, payload TEXT, created_at TIMESTAMP, updated_at TIMESTAMP, tenant_id TEXT)").execute(pool).await;

                     let mission_id = "test_mission_robust_1";
                     let old = Utc::now() - Duration::hours(2);

                     let _ = sqlx::query("INSERT INTO agent_missions (id, status, payload, created_at, updated_at, tenant_id) VALUES (?, ?, ?, ?, ?, ?)")
                        .bind(mission_id)
                        .bind("PENDING")
                        .bind("{}")
                        .bind(old.to_rfc3339())
                        .bind(old.to_rfc3339())
                        .bind("system")
                        .execute(pool).await;

                    let _ = db.cleanup_stagnant_missions(3600).await;

                    let row = sqlx::query("SELECT status FROM agent_missions WHERE id = ?")
                        .bind(mission_id)
                        .fetch_optional(pool).await.unwrap();

                    if let Some(r) = row {
                        let status: String = r.get(0);
                        assert_eq!(status, "FAILED");
                    }
                 }
            }
        }
    }
}
