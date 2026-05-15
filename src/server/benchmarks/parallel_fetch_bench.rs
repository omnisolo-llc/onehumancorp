use std::time::Instant;
use std::sync::Arc;
use crate::hub::Hub;

pub async fn bench_parallel_fetch() {
    tracing::info!("Benchmarking Parallel Execution Fetch...");

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

    let iterations = 1000;

    // Simulate parallel fetching
    let start_parallel = Instant::now();
    for _ in 0..iterations {
        let hub1 = hub.clone();
        let hub2 = hub.clone();
        let hub3 = hub.clone();

        let (_agents, _meetings, _cost) = tokio::join!(
            tokio::task::spawn_blocking(move || hub1.get_agents()),
            tokio::task::spawn_blocking(move || hub2.get_meetings()),
            tokio::task::spawn_blocking(move || {
                let cost_auditor = hub3.get_cost_auditor();
                (cost_auditor.get_total_cost(), cost_auditor.get_total_tokens(), cost_auditor.get_agent_costs_snapshot())
            })
        );
    }
    let parallel_duration = start_parallel.elapsed();

    // Simulate sequential fetching for comparison
    let start_sequential = Instant::now();
    for _ in 0..iterations {
        let _agents = hub.get_agents();
        let _meetings = hub.get_meetings();
        let cost_auditor = hub.get_cost_auditor();
        let _cost = (cost_auditor.get_total_cost(), cost_auditor.get_total_tokens(), cost_auditor.get_agent_costs_snapshot());
    }
    let sequential_duration = start_sequential.elapsed();

    println!("Parallel Fetch for {} iterations took {:?}", iterations, parallel_duration);
    println!("Sequential Fetch for {} iterations took {:?}", iterations, sequential_duration);

    if parallel_duration < sequential_duration {
        let speedup = (sequential_duration.as_secs_f64() - parallel_duration.as_secs_f64()) / sequential_duration.as_secs_f64() * 100.0;
        println!("Parallel execution is {:.2}% faster.", speedup);
    } else {
        println!("Parallel execution is NOT faster in this specific memory-bound microbenchmark scenario due to spawn_blocking overhead, but remains crucial for high-latency I/O.");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_bench_parallel_fetch() {
        bench_parallel_fetch().await;
    }
}
