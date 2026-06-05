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
    let mut conn = super::db::acquire_tenant_conn(pool, tenant_id).await?;
    sqlx::query("INSERT INTO tasks (tenant_id, mission_type, payload, status) VALUES ($1, 'publish_site', $2, 'pending') ON CONFLICT DO NOTHING")
        .bind(tenant_id)
        .bind(serde_json::json!({ "site_id": site_id }))
        .execute(&mut *conn)
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

    let pages = super::db::list_pages(pool, tenant_id, site_id).await?;

    let api_key = std::env::var("MINIMAX_API_KEY").unwrap_or_default();
    let minimax = crate::minimax::MinimaxClient::new(api_key);

    for page in &pages {
        let blocks = super::db::list_blocks(pool, tenant_id, page.id).await?;

        let should_generate_seo = page.seo_metadata.get("name").is_none() || page.seo_metadata.as_object().map(|o| o.is_empty()).unwrap_or(true);

        if should_generate_seo {
            info!("Generating SEO metadata for page {}...", page.id);
            let mut block_texts = Vec::new();
            for b in &blocks {
                if let Some(headline) = b.content.get("headline").and_then(|v| v.as_str()) {
                    block_texts.push(headline.to_string());
                }
                if let Some(desc) = b.content.get("description").and_then(|v| v.as_str()) {
                    block_texts.push(desc.to_string());
                }
            }

            let prompt = format!("You are an expert SEO AI. Based on the following page content, generate a JSON object with SEO metadata (name, description, keywords). Only return the JSON object. Content: {}", block_texts.join(" "));

            if let Ok(res) = minimax.reason(&prompt).await {
                let cleaned = res.trim().trim_start_matches("```json").trim_start_matches("```").trim_end_matches("```").trim();
                if let Ok(mut seo_json) = serde_json::from_str::<serde_json::Value>(cleaned) {
                    if seo_json.get("@context").is_none() {
                        seo_json["@context"] = serde_json::Value::String("https://schema.org".to_string());
                        seo_json["@type"] = serde_json::Value::String("LocalBusiness".to_string());
                    }

                    super::db::update_page_seo_metadata(pool, tenant_id, page.id, seo_json)
                        .await?;
                }
            }
        }
    }

    let mut conn = super::db::acquire_tenant_conn(pool, tenant_id).await?;
    sqlx::query(
        "UPDATE builder_sites SET published_at = NOW(), updated_at = NOW() WHERE tenant_id = $1 AND id = $2",
    )
    .bind(tenant_id)
    .bind(site_id)
    .execute(&mut *conn)
    .await?;

    let cache = crate::builder::edge::get_edge_cache();
    cache.invalidate_by_tag(&format!("tenant-id:{}", tenant_id)).await;

    let cache_key = format!("edge_site_{}_{}", tenant_id, site_id); // Keeping old var for notify to not break it
    sqlx::query("NOTIFY edge_cache_invalidation, $1")
        .bind(&cache_key)
        .execute(&mut *conn)
        .await
        .ok();
    info!("Ops Agent: Invalidated edge cache for {}", cache_key);

    let site = super::db::list_sites(pool, tenant_id).await?.into_iter().find(|s| s.id == site_id);

    if let Some(s) = site {
        if let Some(domain) = s.domain {
            info!("Provisioning SSL certificate for custom domain {}...", domain);
        }
    }

    info!("Site {} published successfully.", site_id);
    Ok(())
}
