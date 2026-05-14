use std::sync::Arc;
use std::time::Instant;
use uuid::Uuid;
use tonic::Request;
use ::server_ohc::app::GetDashboardRequest;
use ::server_ohc::app::dashboard_service_server::DashboardService;

pub async fn run_dashboard_performance_suite() {
    tracing::info!("Starting Dashboard Performance Suite");
    let database_url = std::env::var("DATABASE_URL").unwrap_or_else(|_| "sqlite::memory:".to_string());

    // We are generating heavy mock data for benchmarks.
    let pool = sqlx::sqlite::SqlitePoolOptions::new()
        .acquire_timeout(std::time::Duration::from_secs(1))
        .connect(&database_url).await.unwrap();

    sqlx::query("CREATE TABLE IF NOT EXISTS products (id TEXT, organization_id TEXT, title TEXT, type TEXT, price REAL)").execute(&pool).await.unwrap();
    sqlx::query("CREATE TABLE IF NOT EXISTS orders (id TEXT, tenant_id TEXT, total_amount REAL, status TEXT)").execute(&pool).await.unwrap();
    sqlx::query("CREATE TABLE IF NOT EXISTS tenants (tenant_id TEXT, business_name TEXT, tier TEXT)").execute(&pool).await.unwrap();

    // Heavy mock data insert
    let mut builder = sqlx::QueryBuilder::new("INSERT INTO products (id, organization_id, title, type, price) ");
    let mut products = Vec::new();
    for i in 0..1000 {
        products.push((format!("prod_{}", i), "system".to_string(), format!("Test Product {}", i), "physical".to_string(), 100.0));
    }
    builder.push_values(products.into_iter(), |mut b, (id, org_id, title, t, price)| {
        b.push_bind(id).push_bind(org_id).push_bind(title).push_bind(t).push_bind(price);
    });
    builder.build().execute(&pool).await.unwrap();

    // Measure fetch
    let start = Instant::now();
    let _ = sqlx::query("SELECT * FROM products WHERE organization_id = 'system'").fetch_all(&pool).await.unwrap();
    tracing::info!("Dashboard Heavy Mock Fetch Time: {} us", start.elapsed().as_micros());
}
