use std::time::{Instant, Duration};
use std::sync::Arc;
use crate::hub::Hub;
use ::server_ohc::app::GetDashboardRequest;

pub async fn bench_caching() {
    tracing::info!("Benchmarking Caching Strategy...");

    let db_url = std::env::var("DATABASE_URL").unwrap_or_else(|_| "sqlite::memory:".to_string());

    let db = if db_url.starts_with("sqlite") {
        let pool = sqlx::sqlite::SqlitePoolOptions::new()
            .connect(&db_url).await.unwrap();
        let pg_pool = sqlx::PgPool::connect_lazy("postgres://localhost/dummy").unwrap();
        crate::db::DB { pool: pg_pool, store: crate::db::DbStore::Sqlite(pool) }
    } else {
        let pool = sqlx::postgres::PgPoolOptions::new()
            .connect(&db_url).await.unwrap();
        crate::db::DB { pool: pool.clone(), store: crate::db::DbStore::Postgres }
    };

    let (tx, _rx) = tokio::sync::mpsc::channel(100);
    let hub = Arc::new(Hub::new(tx, db.pool.clone()));

    // Warm up cache
    hub.get_agents();

    let iterations = 10000;

    let start_cached = Instant::now();
    for _ in 0..iterations {
        let _agents = hub.get_agents();
    }
    let cached_duration = start_cached.elapsed();

    // Bypass cache
    let start_uncached = Instant::now();
    for _ in 0..iterations {
        // Access a method that triggers a fresh operation if possible, or invalidate cache first.
        // We will just fetch bypass cache manually or fire an agent to invalidate
        hub.fire_agent("non_existent_agent"); // This public method calls invalidate_agent_cache internally
        let _agents = hub.get_agents();
    }
    let uncached_duration = start_uncached.elapsed();

    println!("Cached Fetch for {} iterations took {:?}", iterations, cached_duration);
    println!("Uncached Fetch for {} iterations took {:?}", iterations, uncached_duration);

    if cached_duration < uncached_duration {
        let speedup = (uncached_duration.as_secs_f64() - cached_duration.as_secs_f64()) / uncached_duration.as_secs_f64() * 100.0;
        println!("Caching Strategy is {:.2}% faster.", speedup);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_bench_caching() {
        bench_caching().await;
    }
}
