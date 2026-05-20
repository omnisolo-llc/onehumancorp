use sqlx::PgPool;
use uuid::Uuid;
use tracing::info;

pub async fn enqueue_publish_site_job(
    pool: &PgPool,
    tenant_id: Uuid,
    site_id: Uuid,
) -> Result<(), sqlx::Error> {
    // Instead of simple spawn, we should use a job queue, but using spawn since queue impl takes custom payload
    // In a real implementation this would enqueue to a PostgreSQL table for processing via SKIP LOCKED pattern.
    // For now we persist a record and simulate processing.
    sqlx::query("INSERT INTO tasks (tenant_id, mission_type, payload, status) VALUES ($1, 'publish_site', $2, 'pending') ON CONFLICT DO NOTHING")
        .bind(tenant_id)
        .bind(serde_json::json!({ "site_id": site_id }))
        .execute(pool)
        .await
        .ok();

    let pool_clone = pool.clone();
    tokio::spawn(async move {
        match execute_publish_site_job(&pool_clone, tenant_id, site_id).await {
            Ok(_) => info!("Successfully published site {}", site_id),
            Err(e) => tracing::error!("Failed to publish site {}: {:?}", site_id, e),
        }
    });
    Ok(())
}

async fn execute_publish_site_job(
    pool: &PgPool,
    tenant_id: Uuid,
    site_id: Uuid,
) -> Result<(), sqlx::Error> {
    info!("Starting publish process for site {}", site_id);

    // 1. Fetch site and pages
    let pages = super::db::list_pages(pool, tenant_id, site_id).await?;

    // 2. Site Compilation (Placeholder for real build step)
    info!("Compiling site {} to static PWA/SSR...", site_id);

    // 3. Dynamic SEO Metadata Generation (via Marketing AI Agent)
    let api_key = std::env::var("MINIMAX_API_KEY").unwrap_or_default();
    let client = crate::minimax::MinimaxClient::new(api_key);

    for page in pages {
        if page.seo_metadata.is_null() || page.seo_metadata == serde_json::json!({}) {
            info!("Generating SEO metadata (JSON-LD) for page {}...", page.id);
            let prompt = format!(
                "Generate JSON-LD SEO metadata for a web page titled '{}'. \
                Output ONLY valid JSON.", page.title
            );
            if let Ok(response) = client.reason(&prompt).await {
                if let Ok(json_ld) = serde_json::from_str::<serde_json::Value>(&response) {
                    sqlx::query("UPDATE builder_pages SET seo_metadata = $1 WHERE id = $2")
                        .bind(&json_ld)
                        .bind(page.id)
                        .execute(pool)
                        .await?;
                }
            }
        }
    }

    // 4. Update published_at timestamp
    sqlx::query(
        "UPDATE builder_sites SET published_at = NOW(), updated_at = NOW() WHERE tenant_id = $1 AND id = $2",
    )
    .bind(tenant_id)
    .bind(site_id)
    .execute(pool)
    .await?;

    // 5. SSL Provisioning (Placeholder for real provision step)
    let site = super::db::list_sites(pool, tenant_id).await?.into_iter().find(|s| s.id == site_id);

    if let Some(s) = site {
        if let Some(domain) = s.domain {
            info!("Provisioning SSL certificate for custom domain {}...", domain);
        }
    }

    info!("Site {} published successfully.", site_id);
    Ok(())
}
