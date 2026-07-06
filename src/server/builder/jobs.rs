use sqlx::PgPool;
use uuid::Uuid;
use tracing::info;
use serde::{Deserialize, Serialize};

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
) -> Result<(), String> {
    info!("Starting publish process for site {}", site_id);

    let pages = super::db::list_pages(pool, tenant_id, site_id)
        .await
        .map_err(|e| e.to_string())?;

    let api_key = std::env::var("MINIMAX_API_KEY").unwrap_or_default();
    let minimax = crate::minimax::MinimaxClient::new(api_key);

    for page in &pages {
        let blocks = super::db::list_blocks(pool, tenant_id, page.id)
            .await
            .map_err(|e| e.to_string())?;

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

            let prompt = format!("You are an expert SEO AI. Based on the following page content, generate a JSON object with SEO metadata for Generative Engine Optimization (GEO). The JSON must include 'name' (title), 'keywords', and a rich 'description' acting as a natural language summary optimized for AI search engines like ChatGPT and Gemini. Only return the JSON object. Content: {}", block_texts.join(" "));

            let mut attempts = 0;
            let mut ai_call_succeeded = false;
            let mut ai_res = String::new();
            while attempts < 3 {
                match tokio::time::timeout(std::time::Duration::from_secs(60), minimax.reason(&prompt)).await {
                    Ok(Ok(res)) => {
                        ai_res = res;
                        ai_call_succeeded = true;
                        break;
                    },
                    _ => {
                        attempts += 1;
                        tokio::time::sleep(std::time::Duration::from_secs(2u64.pow(attempts))).await;
                    }
                }
            }

            if ai_call_succeeded {
                let cleaned = ai_res.trim().trim_start_matches("```json").trim_start_matches("```").trim_end_matches("```").trim();
                if let Ok(mut seo_json) = serde_json::from_str::<serde_json::Value>(cleaned) {
                    if seo_json.get("@context").is_none() {
                        seo_json["@context"] = serde_json::Value::String("https://schema.org".to_string());
                        seo_json["@type"] = serde_json::Value::String("LocalBusiness".to_string());
                    }

                    super::db::update_page_seo_metadata(pool, tenant_id, page.id, seo_json)
                        .await
                        .map_err(|e| e.to_string())?;
                }
            }
        }
    }

    let mut conn = super::db::acquire_tenant_conn(pool, tenant_id)
        .await
        .map_err(|e| e.to_string())?;
    sqlx::query(
        "UPDATE builder_sites SET published_at = NOW(), updated_at = NOW() WHERE tenant_id = $1 AND id = $2",
    )
    .bind(tenant_id)
    .bind(site_id)
    .execute(&mut *conn)
    .await
    .map_err(|e| e.to_string())?;

    let cache = crate::builder::edge::get_edge_cache();
    cache.invalidate_by_tag(&format!("tenant-id:{}", tenant_id)).await;

    let cache_key = format!("edge_site_{}_{}", tenant_id, site_id); // Keeping old var for notify to not break it
    sqlx::query("NOTIFY edge_cache_invalidation, $1")
        .bind(&cache_key)
        .execute(&mut *conn)
        .await
        .ok();
    info!("Ops Agent: Invalidated edge cache for {}", cache_key);

    // Agentic SEO Pre-rendering: Proactively regenerate cache and push directly to edge
    let cache_key_full = format!("edge_site_{}_{}_en-US", tenant_id, site_id);
    match crate::builder::edge::regenerate_cache(pool.clone(), tenant_id, site_id, cache_key_full.clone(), cache.clone()).await {
        Ok(_) => info!("Agentic SEO Pre-rendering: Successfully pre-rendered and pushed to edge cache: {}", cache_key_full),
        Err(e) => tracing::error!("Agentic SEO Pre-rendering: Failed to pre-render edge cache for {}: {}", cache_key_full, e),
    }

    let site = super::db::list_sites(pool, tenant_id)
        .await
        .map_err(|e| e.to_string())?
        .into_iter()
        .find(|s| s.id == site_id);

    if let Some(s) = site {
        if let Some(domain) = s.domain {
            if !domain.ends_with(".ohc.store") {
                let config = CdnPublishConfig::from_env()?;
                let receipt = provision_cdn_and_ssl(&config, tenant_id, site_id, &domain).await?;
                info!(
                    "Provisioned CDN route {} and SSL certificate status {} for {}",
                    receipt.cdn_route_id, receipt.ssl_status, domain
                );
            }
        }
    }

    info!("Site {} published successfully.", site_id);
    Ok(())
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CdnPublishConfig {
    pub api_url: String,
    pub api_token: String,
    pub zone_id: String,
    pub edge_origin: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct CdnPublishPayload {
    pub tenant_id: String,
    pub site_id: String,
    pub domain: String,
    pub origin_url: String,
    pub ssl_mode: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct CdnPublishReceipt {
    pub cdn_route_id: String,
    pub ssl_status: String,
}

impl CdnPublishConfig {
    pub fn from_env() -> Result<Self, String> {
        let api_url = required_env("OHC_CDN_API_URL")?;
        let api_token = required_env("OHC_CDN_API_TOKEN")?;
        let zone_id = required_env("OHC_CDN_ZONE_ID")?;
        let edge_origin = required_env("OHC_EDGE_ORIGIN_URL")?;
        Ok(Self {
            api_url,
            api_token,
            zone_id,
            edge_origin,
        })
    }
}

fn required_env(key: &str) -> Result<String, String> {
    std::env::var(key)
        .map(|value| value.trim().to_string())
        .ok()
        .filter(|value| !value.is_empty())
        .ok_or_else(|| format!("{key} is required for CDN/SSL publish automation"))
}

pub fn cdn_publish_payload(
    config: &CdnPublishConfig,
    tenant_id: Uuid,
    site_id: Uuid,
    domain: &str,
) -> CdnPublishPayload {
    CdnPublishPayload {
        tenant_id: tenant_id.to_string(),
        site_id: site_id.to_string(),
        domain: domain.to_string(),
        origin_url: format!(
            "{}/api/v1/builder/edge/{}/{}",
            config.edge_origin.trim_end_matches('/'),
            tenant_id,
            site_id
        ),
        ssl_mode: "managed".to_string(),
    }
}

async fn provision_cdn_and_ssl(
    config: &CdnPublishConfig,
    tenant_id: Uuid,
    site_id: Uuid,
    domain: &str,
) -> Result<CdnPublishReceipt, String> {
    let payload = cdn_publish_payload(config, tenant_id, site_id, domain);
    let url = format!(
        "{}/zones/{}/sites",
        config.api_url.trim_end_matches('/'),
        config.zone_id
    );
    let response = reqwest::Client::new()
        .post(url)
        .bearer_auth(&config.api_token)
        .json(&payload)
        .send()
        .await
        .map_err(|e| format!("Failed to call CDN publish API: {e}"))?;

    if !response.status().is_success() {
        let status = response.status();
        let body = response.text().await.unwrap_or_default();
        return Err(format!("CDN publish API failed with status {status}: {body}"));
    }

    response
        .json::<CdnPublishReceipt>()
        .await
        .map_err(|e| format!("Failed to parse CDN publish response: {e}"))
}

#[cfg(test)]
mod publish_tests {
    use super::*;

    #[test]
    fn cdn_publish_payload_targets_edge_storefront_route() {
        let tenant_id = Uuid::parse_str("11111111-1111-1111-1111-111111111111").unwrap();
        let site_id = Uuid::parse_str("22222222-2222-2222-2222-222222222222").unwrap();
        let config = CdnPublishConfig {
            api_url: "https://cdn.example.test".to_string(),
            api_token: "token".to_string(),
            zone_id: "zone".to_string(),
            edge_origin: "https://app.example.test/".to_string(),
        };

        let payload = cdn_publish_payload(&config, tenant_id, site_id, "maya.example.com");

        assert_eq!(payload.domain, "maya.example.com");
        assert_eq!(payload.ssl_mode, "managed");
        assert_eq!(
            payload.origin_url,
            "https://app.example.test/api/v1/builder/edge/11111111-1111-1111-1111-111111111111/22222222-2222-2222-2222-222222222222"
        );
    }

    #[test]
    fn required_env_fails_closed_when_key_is_absent() {
        let result = required_env("OHC_TEST_CDN_KEY_THAT_SHOULD_NOT_EXIST");
        assert!(result.unwrap_err().contains("OHC_TEST_CDN_KEY_THAT_SHOULD_NOT_EXIST"));
    }
}
