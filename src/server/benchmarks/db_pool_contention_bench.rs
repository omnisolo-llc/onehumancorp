use std::time::Instant;
use std::sync::Arc;
use tokio::task::JoinHandle;

pub async fn bench_db_pool_contention() {
    tracing::info!("Benchmarking DB Pool Contention (Cloud vs Standalone)...");

    let iterations = 10000;
    let concurrency = 200;

    let database_url = std::env::var("DATABASE_URL").unwrap_or_else(|_| "postgres://localhost/dummy".to_string());

    if database_url.starts_with("postgres") && database_url != "postgres://localhost/dummy" {
        if let Ok(pg_pool) = sqlx::postgres::PgPoolOptions::new()
            .max_connections(20) // Intentionally constrain pool to force contention
            .connect(&database_url).await
        {
            let pg_pool = Arc::new(pg_pool);
            run_pool_benchmark("Postgres Pool (constrained)", pg_pool, iterations, concurrency).await;
        }
    }

    let sqlite_pool = sqlx::sqlite::SqlitePoolOptions::new()
        .max_connections(20)
        .connect("sqlite::memory:").await.unwrap();
    let sqlite_pool = Arc::new(sqlite_pool);
    // Setup dummy table
    let _ = sqlx::query("CREATE TABLE IF NOT EXISTS contention_test (id INTEGER PRIMARY KEY, data TEXT)").execute(&*sqlite_pool).await;

    run_sqlite_pool_benchmark("SQLite In-Memory Pool (constrained)", sqlite_pool, iterations, concurrency).await;
}

async fn run_pool_benchmark(name: &str, pool: Arc<sqlx::PgPool>, total_queries: usize, concurrency: usize) {
    let mut handles: Vec<JoinHandle<()>> = Vec::new();
    let queries_per_worker = total_queries / concurrency;

    let start = Instant::now();
    for _worker_id in 0..concurrency {
        let p = pool.clone();
        handles.push(tokio::spawn(async move {
            for _ in 0..queries_per_worker {
                let _ = sqlx::query("SELECT 1").execute(&*p).await;
            }
        }));
    }

    for handle in handles {
        let _ = handle.await;
    }

    let duration = start.elapsed();
    println!("{}: Executed {} queries across {} concurrent workers in {:?}", name, total_queries, concurrency, duration);
}

async fn run_sqlite_pool_benchmark(name: &str, pool: Arc<sqlx::SqlitePool>, total_queries: usize, concurrency: usize) {
    let mut handles: Vec<JoinHandle<()>> = Vec::new();
    let queries_per_worker = total_queries / concurrency;

    let start = Instant::now();
    for _worker_id in 0..concurrency {
        let p = pool.clone();
        handles.push(tokio::spawn(async move {
            for _ in 0..queries_per_worker {
                let _ = sqlx::query("SELECT 1").execute(&*p).await;
            }
        }));
    }

    for handle in handles {
        let _ = handle.await;
    }

    let duration = start.elapsed();
    println!("{}: Executed {} queries across {} concurrent workers in {:?}", name, total_queries, concurrency, duration);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_bench_db_pool_contention() {
        bench_db_pool_contention().await;
    }
}
