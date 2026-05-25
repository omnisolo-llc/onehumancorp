#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;
    use tokio::sync::mpsc;
    use crate::api::health::health_handler;
    use crate::hub::Hub;
    use axum::extract::State;

    #[tokio::test]
    async fn test_hybrid_health_probe_mapping() {
        let db_url = std::env::var("DATABASE_URL").unwrap_or_else(|_| "sqlite::memory:".to_string());
        if !db_url.starts_with("sqlite") && std::env::var("DATABASE_URL").is_err() {
            return;
        }

        let pg_pool = sqlx::postgres::PgPoolOptions::new()
            .connect_lazy("postgres://dummy")
            .unwrap();

        let (tx, _) = mpsc::channel(100);
        let hub = Arc::new(Hub::new(tx, pg_pool));

        let res = health_handler(State(hub.clone())).await;

        // Assert that unsynced_missions is mapped in the returned JSON
        assert!(res.get("unsynced_missions").is_some());
        assert!(res.get("stuck_missions").is_some());
        assert!(res.get("status").is_some());
    }
}
