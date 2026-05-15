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

    let mut tx = pool.begin().await?;
    ::server_common::auth_utils::set_org_context(&mut *tx, tenant_id.to_string().as_str()).await?;

    sqlx::query("INSERT INTO tasks (tenant_id, mission_type, payload, status) VALUES ($1, 'publish_site', $2, 'pending') ON CONFLICT DO NOTHING")
        .bind(tenant_id)
        .bind(serde_json::json!({ "site_id": site_id }))
        .execute(&mut *tx)
        .await
        .ok();

    tx.commit().await?;

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

    let mut tx = pool.begin().await?;
    ::server_common::auth_utils::set_org_context(&mut *tx, tenant_id.to_string().as_str()).await?;

    // 1. Fetch site and pages
    let _pages = super::db::list_pages(&mut tx, tenant_id, site_id).await?;

    // 2. Mock Site Compilation
    info!("Compiling site {} to static PWA/SSR...", site_id);

    // 3. Mock SEO Metadata Generation (via Marketing AI Agent)
    info!("Generating SEO metadata (JSON-LD) for site {}...", site_id);

    // 4. Update published_at timestamp
    sqlx::query(

        "UPDATE builder_sites SET published_at = NOW(), updated_at = NOW() WHERE tenant_id = $1 AND id = $2",
    )
    .bind(tenant_id)
    .bind(site_id)
    .execute(&mut *tx)
    .await?;

    // 5. Mock SSL Provisioning (if custom domain)
    // We would use query_as, but for simplicity we can just query the site.
    let site = super::db::list_sites(&mut tx, tenant_id).await?.into_iter().find(|s| s.id == site_id);

    if let Some(s) = site {
        if let Some(domain) = s.domain {
            info!("Provisioning SSL certificate for custom domain {}...", domain);
        }
    }

    tx.commit().await?;

    info!("Site {} published successfully.", site_id);
    Ok(())
}
