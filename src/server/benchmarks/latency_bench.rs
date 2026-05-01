use std::time::Instant;
use std::sync::Arc;
use crate::queue::{TaskQueue, MemoryTaskQueue, Job, PostgresTaskQueue};
use chrono::Utc;
use uuid::Uuid;

// Phase 1: Baseline / Phase 2: Parallel Fetching Optimization & Batching
pub async fn bench_dashboard_snapshot() {
    println!("Benchmarking Dashboard Snapshot Fetching...");
    let (tx, _rx) = tokio::sync::mpsc::channel(100);

    let database_url = std::env::var("DATABASE_URL").unwrap_or_else(|_| "postgres://localhost/dummy".to_string());

    if database_url == "postgres://localhost/dummy" {
        println!("Skipping bench_dashboard_snapshot due to missing db connection (dummy url)");
        return;
    }

    let pool_res = sqlx::postgres::PgPoolOptions::new()
        .before_acquire(|conn, _meta| {
            Box::pin(async move {
                use sqlx::Executor;
                conn.execute("SET app.current_tenant = 'system'").await?;
                Ok(true)
            })
        })
        .connect(&database_url).await;

    let pg_pool = match pool_res {
        Ok(p) => p,
        Err(e) => {
            println!("Skipping bench_dashboard_snapshot due to missing db connection: {}", e);
            return;
        }
    };

    let hub = Arc::new(crate::hub::Hub::new(tx, pg_pool));

    let iterations = 100;
    let mut fetch_times = Vec::new();

    for i in 0..50 {
        hub.register_agent(crate::ohc::orchestration::Agent {
            id: format!("agent-{}", i),
            name: format!("Agent {}", i),
            role: "test".to_string(),
            organization_id: "system".to_string(),
            status: "IDLE".to_string(),
            provider_type: "builtin".to_string(),
        });
    }

    for _ in 0..iterations {
        let start = Instant::now();

        let hub_clone1 = hub.clone();
        let hub_clone2 = hub.clone();

        let (_, _) = tokio::join!(
            tokio::task::spawn_blocking(move || { let _ = hub_clone1.get_agents(); }),
            tokio::task::spawn_blocking(move || { let _ = hub_clone2.get_meetings(); })
        );

        fetch_times.push(start.elapsed().as_micros());
    }

    fetch_times.sort();
    println!("Parallel Fetch: p50: {} us, p95: {} us, p99: {} us", fetch_times[iterations / 2], fetch_times[(iterations as f32 * 0.95) as usize], fetch_times[(iterations as f32 * 0.99) as usize]);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[ignore] // Requires live database for Hub initialization
    async fn test_bench_dashboard_snapshot() {
        bench_dashboard_snapshot().await;
    }
}
