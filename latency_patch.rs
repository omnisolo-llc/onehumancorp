use std::time::Instant;
use std::sync::Arc;
use crate::queue::{TaskQueue, MemoryTaskQueue, Job, PostgresTaskQueue};
use chrono::Utc;
use uuid::Uuid;

// Phase 1: Baseline / Phase 2: Parallel Fetching Optimization & Batching
pub async fn bench_queue_latency() {
    println!("Benchmarking Latency...");

    // 1. Cloud Mode - Postgres
    println!("--- Cloud Mode (Postgres) ---");
    let database_url = std::env::var("DATABASE_URL").unwrap_or_else(|_| "postgres://localhost/dummy".to_string());

    if database_url != "postgres://localhost/dummy" {
        let pool_res = sqlx::postgres::PgPoolOptions::new()
            .after_release(|conn, _meta| { Box::pin(async move { use sqlx::Executor; conn.execute("DISCARD ALL").await?; Ok(true) }) })
            .connect(&database_url).await;

        if let Ok(pg_pool) = pool_res {
            let pg_queue = Arc::new(PostgresTaskQueue::new(pg_pool));
            bench_queue("Postgres", pg_queue).await;
        } else {

        }
    } else {

    }

    // 2. Standalone Mode - Memory
    println!("--- Standalone Mode (Memory) ---");
    let mem_queue = Arc::new(MemoryTaskQueue::new());
    bench_queue("Memory", mem_queue).await;
}
