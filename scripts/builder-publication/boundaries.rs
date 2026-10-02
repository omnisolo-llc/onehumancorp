pub mod minimax {
    pub struct MinimaxClient;
    impl MinimaxClient {
        pub fn new(_: String) -> Self {
            Self
        }
        pub async fn reason(&self, _: &str) -> Result<String, String> {
            panic!("Publication fixtures already have owner SEO; no provider execution allowed")
        }
    }
}
pub mod edge {
    use sqlx::PgPool;
    use std::sync::{Arc, Mutex};
    use uuid::Uuid;
    pub static OBSERVED: Mutex<Vec<bool>> = Mutex::new(Vec::new());
    pub static BINDING: Mutex<Option<(PgPool, Uuid)>> = Mutex::new(None);
    pub struct EdgeCache;
    impl EdgeCache {
        pub async fn invalidate_by_tag(&self, _: &str) {
            let (pool, site) = BINDING.lock().unwrap().clone().unwrap();
            let committed: bool = sqlx::query_scalar(
                "SELECT published_at IS NOT NULL FROM builder_sites WHERE id=$1",
            )
            .bind(site)
            .fetch_one(&pool)
            .await
            .unwrap();
            OBSERVED.lock().unwrap().push(committed);
        }
    }
    pub fn get_edge_cache() -> Arc<EdgeCache> {
        Arc::new(EdgeCache)
    }
    pub async fn regenerate_cache(
        _: PgPool,
        _: Uuid,
        _: Uuid,
        _: String,
        _: Arc<EdgeCache>,
    ) -> Result<(), String> {
        Ok(())
    }
}
