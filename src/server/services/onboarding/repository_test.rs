#[cfg(test)]
mod tests {
    use crate::services::onboarding::repository::OnboardingRepository;
    use sqlx::sqlite::SqlitePoolOptions;
    use serde_json::json;

    #[tokio::test]
    async fn test_sqlite_repository_persistence() {
        let pool = SqlitePoolOptions::new()
            .connect("sqlite::memory:")
            .await
            .unwrap();

        sqlx::query("CREATE TABLE onboarding_state (tenant_id TEXT, user_id TEXT, current_step INTEGER, state_json TEXT, created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP, updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP, PRIMARY KEY (tenant_id, user_id))")
            .execute(&pool)
            .await
            .unwrap();

        let repo = OnboardingRepository::new_sqlite(pool);

        let state = json!({"foo": "bar"});
        repo.save_state("t1", "o1", "u1", 1, state.clone()).await.unwrap();

        let retrieved = repo.get_state("t1").await.unwrap().unwrap();
        assert_eq!(retrieved["foo"], "bar");

        repo.delete_state("t1").await.unwrap();
        let none = repo.get_state("t1").await.unwrap();
        assert!(none.is_none());
    }
}
