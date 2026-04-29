use std::time::Instant;
use sqlx::{Pool, Postgres, Sqlite};
use chrono::Utc;

pub struct BenchmarkResult {
    pub name: String,
    pub mode: String,
    pub p50_ms: f64,
    pub p95_ms: f64,
    pub p99_ms: f64,
    pub avg_ms: f64,
    pub samples: usize,
}

// Macro or generic approach to handle both db types
pub async fn run_hybrid_latency_benchmark_pg(pool: Pool<Postgres>) -> Vec<BenchmarkResult> {
    let mode = "Cloud".to_string();
    println!("Running Hybrid Latency Benchmark in {} mode...", mode);

    let mut results = Vec::new();

    // 1. Complex DB Query Latency (Postgres)
    let mut latencies = Vec::new();
    let samples = 100;
    for _ in 0..samples {
        let start = Instant::now();
        let _ = sqlx::query("SELECT COUNT(*) FROM tasks WHERE status = 'PENDING'").execute(&pool).await;
        latencies.push(start.elapsed().as_micros() as f64 / 1000.0);
    }
    results.push(calculate_percentiles(latencies, "Complex DB Query Latency", &mode));

    // 2. AI Job Dispatch Latency (Postgres)
    let mut latencies = Vec::new();
    for i in 0..samples {
        let start = Instant::now();
        let payload = serde_json::json!({"test": i}).to_string();
        let _ = sqlx::query("INSERT INTO sub_agent_queue (id, organization_id, parent_task_id, payload, status, scheduled_at, created_at, updated_at) VALUES ($1, $2, $3, $4, $5, $6, $7, $8)")
            .bind(format!("job-pg-{}", i)).bind("org-1").bind("parent-1").bind(payload).bind("PENDING").bind(Utc::now()).bind(Utc::now()).bind(Utc::now())
            .execute(&pool).await;

        let _ = sqlx::query("SELECT id FROM sub_agent_queue WHERE status = 'PENDING' LIMIT 1").fetch_optional(&pool).await;
        latencies.push(start.elapsed().as_micros() as f64 / 1000.0);
    }
    let _ = sqlx::query("DELETE FROM sub_agent_queue WHERE organization_id = 'org-1'").execute(&pool).await;
    results.push(calculate_percentiles(latencies, "AI Job Dispatch Latency", &mode));

    results
}

pub async fn run_hybrid_latency_benchmark_sqlite(pool: Pool<Sqlite>) -> Vec<BenchmarkResult> {
    let mode = "Standalone".to_string();
    println!("Running Hybrid Latency Benchmark in {} mode...", mode);

    let mut results = Vec::new();

    // 1. Complex DB Query Latency (SQLite)
    let mut latencies = Vec::new();
    let samples = 100;
    for _ in 0..samples {
        let start = Instant::now();
        let _ = sqlx::query("SELECT COUNT(*) FROM tasks WHERE status = 'PENDING'").execute(&pool).await;
        latencies.push(start.elapsed().as_micros() as f64 / 1000.0);
    }
    results.push(calculate_percentiles(latencies, "Complex DB Query Latency", &mode));

    // 2. AI Job Dispatch Latency (SQLite)
    let mut latencies = Vec::new();
    for i in 0..samples {
        let start = Instant::now();
        let payload = serde_json::json!({"test": i}).to_string();
        let _ = sqlx::query("INSERT INTO sub_agent_queue (id, organization_id, parent_task_id, payload, status, scheduled_at, created_at, updated_at) VALUES ($1, $2, $3, $4, $5, $6, $7, $8)")
            .bind(format!("job-sq-{}", i)).bind("org-1").bind("parent-1").bind(payload).bind("PENDING").bind(Utc::now().to_rfc3339()).bind(Utc::now().to_rfc3339()).bind(Utc::now().to_rfc3339())
            .execute(&pool).await;

        let _ = sqlx::query("SELECT id FROM sub_agent_queue WHERE status = 'PENDING' LIMIT 1").fetch_optional(&pool).await;
        latencies.push(start.elapsed().as_micros() as f64 / 1000.0);
    }
    let _ = sqlx::query("DELETE FROM sub_agent_queue WHERE organization_id = 'org-1'").execute(&pool).await;
    results.push(calculate_percentiles(latencies, "AI Job Dispatch Latency", &mode));

    results
}

pub fn calculate_percentiles(mut latencies: Vec<f64>, name: &str, mode: &str) -> BenchmarkResult {
    latencies.sort_by(|a, b| a.partial_cmp(b).unwrap());

    let samples = latencies.len();
    if samples == 0 {
        return BenchmarkResult {
            name: name.to_string(), mode: mode.to_string(),
            p50_ms: 0.0, p95_ms: 0.0, p99_ms: 0.0, avg_ms: 0.0, samples: 0,
        };
    }

    let p50_idx = (samples as f64 * 0.50) as usize;
    let p95_idx = (samples as f64 * 0.95) as usize;
    let p99_idx = (samples as f64 * 0.99) as usize;

    let p50_ms = latencies.get(p50_idx).copied().unwrap_or(0.0);
    let p95_ms = latencies.get(p95_idx).copied().unwrap_or(0.0);
    let p99_ms = latencies.get(p99_idx).copied().unwrap_or(0.0);

    let avg_ms = latencies.iter().sum::<f64>() / samples as f64;

    BenchmarkResult {
        name: name.to_string(),
        mode: mode.to_string(),
        p50_ms,
        p95_ms,
        p99_ms,
        avg_ms,
        samples,
    }
}
