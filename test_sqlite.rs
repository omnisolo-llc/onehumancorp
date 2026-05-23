use sqlx::{SqlitePool, sqlite::SqlitePoolOptions};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect("sqlite::memory:")
        .await?;

    sqlx::query(
        "CREATE VIRTUAL TABLE IF NOT EXISTS agent_memory USING fts5(
            content,
            tags,
            created_at UNINDEXED
        )"
    )
    .execute(&pool)
    .await?;

    sqlx::query("INSERT INTO agent_memory (content, tags, created_at) VALUES (?, ?, CURRENT_TIMESTAMP)")
        .bind("The secret code is 42")
        .bind(serde_json::to_string(&vec!["secret".to_string()]).unwrap())
        .execute(&pool)
        .await?;

    let rows = sqlx::query_as::<_, (String,)>("SELECT content FROM agent_memory WHERE agent_memory MATCH ? ORDER BY rank LIMIT ?")
        .bind("secret")
        .bind(10i64)
        .fetch_all(&pool)
        .await?;

    for row in rows {
        println!("{}", row.0);
    }

    Ok(())
}
