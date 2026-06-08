use sqlx::{Pool, Postgres, Row};
use std::sync::Arc;
use tokio::time::{sleep, Duration};

pub async fn run_seo_pre_render_worker(db: Arc<crate::db::DB>) {
    let pool = db.pool.clone();

    loop {
        if let Err(e) = process_next_job(&pool).await {
            tracing::error!("Error in SEO pre-render worker: {:?}", e);
        }
        sleep(Duration::from_secs(5)).await;
    }
}

async fn process_next_job(pool: &Pool<Postgres>) -> Result<(), sqlx::Error> {
    let mut tx = pool.begin().await?;

    let row_opt = sqlx::query(
        r#"
        SELECT id, tenant_id, path
        FROM pre_render_jobs
        WHERE status = 'PENDING'
        FOR UPDATE SKIP LOCKED
        LIMIT 1
        "#
    )
    .fetch_optional(&mut *tx)
    .await?;

    if let Some(row) = row_opt {
        let job_id: uuid::Uuid = row.get("id");
        let tenant_id: String = row.get("tenant_id");
        let path: String = row.get("path");

        sqlx::query("UPDATE pre_render_jobs SET status = 'PROCESSING', locked_at = CURRENT_TIMESTAMP WHERE id = $1")
        .bind(job_id)
        .execute(&mut *tx)
        .await?;

        tx.commit().await?;

        // 3. Implement the worker that fetches dynamic content, injects AI-generated SEO metadata,
        // renders static HTML, and uploads it to an edge-accessible store (e.g., GCS/S3).
        tracing::info!("Pre-rendering SEO for tenant {}, path {}", tenant_id, path);

        // Mock generation and upload...
        let html_content = format!("<html><head><title>{}</title></head><body>Content for {}</body></html>", tenant_id, path);
        tracing::info!("Uploading rendered content to edge store: {}", html_content);

        sqlx::query("UPDATE pre_render_jobs SET status = 'COMPLETED' WHERE id = $1")
        .bind(job_id)
        .execute(pool)
        .await?;
    } else {
        tx.rollback().await?;
    }

    Ok(())
}
