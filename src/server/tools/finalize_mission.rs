use sqlx::PgPool;
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let database_url = env::var("DATABASE_URL").unwrap_or_else(|_| "postgres://postgres:postgres@localhost:5432/ohc".to_string());

    if database_url.starts_with("postgres") {
        let pool = PgPool::connect(&database_url).await?;

        let mission_id = "mission_mcp_telemetry_mesh_001";

        let result = sqlx::query(
            "UPDATE agent_missions SET status = 'COMPLETED', updated_at = CURRENT_TIMESTAMP WHERE id = $1"
        )
        .bind(mission_id)
        .execute(&pool)
        .await?;

        if result.rows_affected() > 0 {
            println!("Successfully marked mission {} as COMPLETED", mission_id);
        } else {
            println!("Mission {} not found or already updated", mission_id);
        }
    } else {
        println!("Not a postgres URL, skipping mission update in script");
    }
    Ok(())
}
