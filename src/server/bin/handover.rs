use std::env;
use sqlx::postgres::PgPoolOptions;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        eprintln!("Usage: handover <mission_id> <blocker_message>");
        std::process::exit(1);
    }
    let mission_id = &args[1];
    let blocker_message = &args[2];

    let database_url = env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgres://postgres:postgres@localhost:5432/ohc".to_string());
    let pool = PgPoolOptions::new()
        .connect(&database_url)
        .await?;

    let result = sqlx::query(
        "UPDATE agent_missions
         SET status = 'blocked',
             mission_log = CASE WHEN mission_log IS NULL OR mission_log = '' THEN $1 ELSE mission_log || '\n' || $1 END,
             updated_at = CURRENT_TIMESTAMP
         WHERE id = $2"
    )
    .bind(blocker_message)
    .bind(mission_id)
    .execute(&pool)
    .await?;

    if result.rows_affected() > 0 {
        println!("Mission '{}' successfully handed over and blocked.", mission_id);
    } else {
        println!("Mission '{}' not found.", mission_id);
    }

    Ok(())
}
