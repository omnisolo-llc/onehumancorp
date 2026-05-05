use sqlx::PgPool;
use uuid::Uuid;
use tracing::info;

pub async fn enqueue_publish_site_job(
    pool: &PgPool,
    tenant_id: Uuid,
    site_id: Uuid,
) -> Result<(), sqlx::Error> {
    let job_id = Uuid::new_v4().to_string();
    let payload = serde_json::json!({ "site_id": site_id, "action": "publish_site" });

    let job = crate::queue::SubAgentJob {
        id: job_id.clone(),
        organization_id: tenant_id.to_string(),
        parent_task_id: "".to_string(),
        payload,
        status: "QUEUED".to_string(),
        worker_id: None,
        created_at: chrono::Utc::now(),
        updated_at: chrono::Utc::now(),
    };

    let qm = crate::queue::QueueManager::new(pool.clone());
    qm.enqueue(job).await?;
    info!("Enqueued publish_site job {} for site {}", job_id, site_id);
    Ok(())
}

pub async fn execute_publish_site_job(
    pool: &PgPool,
    tenant_id: Uuid,
    site_id: Uuid,
) -> Result<(), sqlx::Error> {
    info!("Starting publish process for site {}", site_id);

    // 1. Fetch site and pages
    let _pages = super::db::list_pages(pool, tenant_id, site_id).await?;

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
    .execute(pool)
    .await?;

    // 5. Mock SSL Provisioning (if custom domain)
    let sites = super::db::list_sites(pool, tenant_id).await?;
    let site = sites.into_iter().find(|s| s.id == site_id);

    if let Some(s) = site {
        if let Some(domain) = s.domain {
            info!("Provisioning SSL certificate for custom domain {}...", domain);
        }
    }

    info!("Site {} published successfully.", site_id);
    Ok(())
}
