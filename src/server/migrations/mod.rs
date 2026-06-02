#[cfg(test)]
mod test_067_agent_missions_cloud_mission_id {
    use sqlx::postgres::PgPoolOptions;

    #[tokio::test]
    async fn test_migration_067_agent_missions_cloud_mission_id() {
        let database_url = match std::env::var("OHC_DATABASE_URL") {
            Ok(url) => url,
            Err(_) => return,
        };

        if database_url.starts_with("sqlite") {
            return; // Postgres-specific test
        }

        let pool = PgPoolOptions::new()
            .connect(&database_url)
            .await
            .unwrap();

        // Verify the column exists
        let row = sqlx::query(
            "SELECT column_name
             FROM information_schema.columns
             WHERE table_name='agent_missions' and column_name='cloud_mission_id';"
        )
        .fetch_optional(&pool)
        .await
        .unwrap();

        assert!(row.is_some(), "cloud_mission_id column should exist on agent_missions table");
    }
}
