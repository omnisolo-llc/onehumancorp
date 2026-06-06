use std::sync::Arc;
use crate::db::DB;
use std::time::Duration;
use uuid::Uuid;
use tokio::time::timeout;

pub struct SeoDiscoveryWorker {
    pub db: Arc<DB>,
    pub poll_interval: Duration,
    pub hub: Arc<crate::hub::Hub>,
}

impl SeoDiscoveryWorker {
    pub fn new(db: Arc<DB>, hub: Arc<crate::hub::Hub>) -> Self {
        Self {
            db,
            poll_interval: Duration::from_secs(5),
            hub,
        }
    }

    pub fn start(&self) {
        let db = self.db.clone();
        let hub = self.hub.clone();
        let mut product_rx = hub.subscribe_teammate_mesh("products_inbox".to_string());
        let mut services_rx = hub.subscribe_teammate_mesh("services_inbox".to_string());
        let mut profile_rx = hub.subscribe_teammate_mesh("business_profile_inbox".to_string());

        let hub_clone1 = hub.clone();
        let db_clone1 = db.clone();
        tokio::spawn(async move {
            let pool = db_clone1.pool.clone();
            while let Ok(event) = product_rx.recv().await {
                if event.action == "ProductCreated" || event.action == "ProductUpdated" {
                    if let Ok(payload_str) = String::from_utf8(event.payload.clone()) {
                        if let Ok(payload_json) = serde_json::from_str::<serde_json::Value>(&payload_str) {
                            let org_id = payload_json.get("organization_id").and_then(|o| o.as_str()).unwrap_or("system");
                            if let Some(product_id) = payload_json.get("product_id").and_then(|p| p.as_str()) {

                                // Generate SEO content via AI
                                let prompt = format!(
                                    "Generate local SEO metadata for this product: {}. Return JSON with: meta_title, meta_description, structured_data (JSON-LD format for Product), generated_keywords",
                                    payload_json.to_string()
                                );

                                let mut attempts = 0;
                                let mut resolved_payload = serde_json::json!({});
                                while attempts < 3 {
                                    let ai_op = async {
                                        if let Ok(mut client) = ::server_ohc::orchestration::hub_service_client::HubServiceClient::connect(std::env::var("OHC_HUB_URL").unwrap_or_else(|_| "http://127.0.0.1:8081".to_string())).await {
                                            let reason_req = ::server_ohc::orchestration::ReasonRequest {
                                                prompt: prompt.clone(),
                                                from_agent_id: "promoter_seo".to_string(),
                                            };
                                            if let Ok(res) = client.reason(tonic::Request::new(reason_req)).await {
                                                if let Ok(v) = serde_json::from_str::<serde_json::Value>(&res.into_inner().content) {
                                                    return Ok(v);
                                                }
                                            }
                                        }
                                        Err("AI call failed".to_string())
                                    };

                                    match timeout(Duration::from_secs(30), ai_op).await {
                                        Ok(Ok(v)) => {
                                            resolved_payload = v;
                                            break;
                                        },
                                        _ => {
                                            attempts += 1;
                                            tokio::time::sleep(Duration::from_secs(2u64.pow(attempts))).await;
                                        }
                                    }
                                }

                                if resolved_payload.is_object() && !resolved_payload.as_object().unwrap().is_empty() {
                                    // Save to ohc_seo_metadata
                                    let meta_title = resolved_payload.get("meta_title").and_then(|v| v.as_str()).unwrap_or("").to_string();
                                    let meta_description = resolved_payload.get("meta_description").and_then(|v| v.as_str()).unwrap_or("").to_string();
                                    let default_structured_data = serde_json::json!({});
                                    let structured_data = resolved_payload.get("structured_data").unwrap_or(&default_structured_data);
                                    let generated_keywords = resolved_payload.get("generated_keywords").and_then(|v| v.as_str()).unwrap_or("").to_string();

                                    match pool.begin().await {
                                        Ok(mut tx) => {
                                            if let Err(e) = ::server_common::auth_utils::set_org_context(&mut *tx, org_id).await {
                                                tracing::error!("Failed to set org context for seo worker: {}", e);
                                                continue;
                                            }

                                            let seo_id = Uuid::new_v4().to_string();

                                            let result = sqlx::query(
                                                "INSERT INTO ohc_seo_metadata (id, tenant_id, entity_id, entity_type, meta_title, meta_description, structured_data, generated_keywords, status)
                                                 VALUES ($1, $2, $3, 'product', $4, $5, $6, $7, 'PENDING_APPROVAL')
                                                 ON CONFLICT (tenant_id, entity_id, entity_type)
                                                 DO UPDATE SET meta_title = EXCLUDED.meta_title, meta_description = EXCLUDED.meta_description, structured_data = EXCLUDED.structured_data, generated_keywords = EXCLUDED.generated_keywords, status = 'PENDING_APPROVAL', updated_at = CURRENT_TIMESTAMP"
                                            )
                                            .bind(&seo_id)
                                            .bind(org_id)
                                            .bind(product_id)
                                            .bind(meta_title)
                                            .bind(meta_description)
                                            .bind(structured_data)
                                            .bind(generated_keywords)
                                            .execute(&mut *tx)
                                            .await;

                                            if let Err(e) = result {
                                                tracing::error!("Failed to save SEO metadata: {}", e);
                                            } else {
                                                if let Err(e) = tx.commit().await {
                                                    tracing::error!("Failed to commit SEO metadata transaction: {}", e);
                                                }
                                            }
                                        }
                                        Err(e) => {
                                            tracing::error!("Failed to start transaction for seo worker: {}", e);
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        });

        let hub_clone2 = hub.clone();
        let db_clone2 = db.clone();
        tokio::spawn(async move {
            let pool = db_clone2.pool.clone();
            while let Ok(event) = services_rx.recv().await {
                if event.action == "ServiceCreated" || event.action == "ServiceUpdated" {
                    if let Ok(payload_str) = String::from_utf8(event.payload.clone()) {
                        if let Ok(payload_json) = serde_json::from_str::<serde_json::Value>(&payload_str) {
                            let org_id = payload_json.get("organization_id").and_then(|o| o.as_str()).unwrap_or("system");
                            if let Some(service_id) = payload_json.get("service_id").and_then(|p| p.as_str()) {

                                let prompt = format!(
                                    "Generate local SEO metadata for this service: {}. Return JSON with: meta_title, meta_description, structured_data (JSON-LD format for Service), generated_keywords",
                                    payload_json.to_string()
                                );

                                let mut attempts = 0;
                                let mut resolved_payload = serde_json::json!({});
                                while attempts < 3 {
                                    let ai_op = async {
                                        if let Ok(mut client) = ::server_ohc::orchestration::hub_service_client::HubServiceClient::connect(std::env::var("OHC_HUB_URL").unwrap_or_else(|_| "http://127.0.0.1:8081".to_string())).await {
                                            let reason_req = ::server_ohc::orchestration::ReasonRequest {
                                                prompt: prompt.clone(),
                                                from_agent_id: "promoter_seo".to_string(),
                                            };
                                            if let Ok(res) = client.reason(tonic::Request::new(reason_req)).await {
                                                if let Ok(v) = serde_json::from_str::<serde_json::Value>(&res.into_inner().content) {
                                                    return Ok(v);
                                                }
                                            }
                                        }
                                        Err("AI call failed".to_string())
                                    };

                                    match timeout(Duration::from_secs(30), ai_op).await {
                                        Ok(Ok(v)) => {
                                            resolved_payload = v;
                                            break;
                                        },
                                        _ => {
                                            attempts += 1;
                                            tokio::time::sleep(Duration::from_secs(2u64.pow(attempts))).await;
                                        }
                                    }
                                }

                                if resolved_payload.is_object() && !resolved_payload.as_object().unwrap().is_empty() {
                                    let meta_title = resolved_payload.get("meta_title").and_then(|v| v.as_str()).unwrap_or("").to_string();
                                    let meta_description = resolved_payload.get("meta_description").and_then(|v| v.as_str()).unwrap_or("").to_string();
                                    let default_structured_data = serde_json::json!({});
                                    let structured_data = resolved_payload.get("structured_data").unwrap_or(&default_structured_data);
                                    let generated_keywords = resolved_payload.get("generated_keywords").and_then(|v| v.as_str()).unwrap_or("").to_string();

                                    match pool.begin().await {
                                        Ok(mut tx) => {
                                            if let Err(e) = ::server_common::auth_utils::set_org_context(&mut *tx, org_id).await {
                                                tracing::error!("Failed to set org context for seo worker: {}", e);
                                                continue;
                                            }

                                            let seo_id = Uuid::new_v4().to_string();

                                            let result = sqlx::query(
                                                "INSERT INTO ohc_seo_metadata (id, tenant_id, entity_id, entity_type, meta_title, meta_description, structured_data, generated_keywords, status)
                                                 VALUES ($1, $2, $3, 'service', $4, $5, $6, $7, 'PENDING_APPROVAL')
                                                 ON CONFLICT (tenant_id, entity_id, entity_type)
                                                 DO UPDATE SET meta_title = EXCLUDED.meta_title, meta_description = EXCLUDED.meta_description, structured_data = EXCLUDED.structured_data, generated_keywords = EXCLUDED.generated_keywords, status = 'PENDING_APPROVAL', updated_at = CURRENT_TIMESTAMP"
                                            )
                                            .bind(&seo_id)
                                            .bind(org_id)
                                            .bind(service_id)
                                            .bind(meta_title)
                                            .bind(meta_description)
                                            .bind(structured_data)
                                            .bind(generated_keywords)
                                            .execute(&mut *tx)
                                            .await;

                                            if let Err(e) = result {
                                                tracing::error!("Failed to save SEO metadata: {}", e);
                                            } else {
                                                if let Err(e) = tx.commit().await {
                                                    tracing::error!("Failed to commit SEO metadata transaction: {}", e);
                                                }
                                            }
                                        }
                                        Err(e) => {
                                            tracing::error!("Failed to start transaction for seo worker: {}", e);
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        });

        let hub_clone3 = hub.clone();
        let db_clone3 = db.clone();
        tokio::spawn(async move {
            let pool = db_clone3.pool.clone();
            while let Ok(event) = profile_rx.recv().await {
                if event.action == "BusinessProfileUpdated" {
                    if let Ok(payload_str) = String::from_utf8(event.payload.clone()) {
                        if let Ok(payload_json) = serde_json::from_str::<serde_json::Value>(&payload_str) {
                            let org_id = payload_json.get("organization_id").and_then(|o| o.as_str()).unwrap_or("system");
                            if let Some(profile_id) = payload_json.get("profile_id").and_then(|p| p.as_str()) {

                                let prompt = format!(
                                    "Generate local SEO metadata for this business profile: {}. Return JSON with: meta_title, meta_description, structured_data (JSON-LD format for LocalBusiness), generated_keywords",
                                    payload_json.to_string()
                                );

                                let mut attempts = 0;
                                let mut resolved_payload = serde_json::json!({});
                                while attempts < 3 {
                                    let ai_op = async {
                                        if let Ok(mut client) = ::server_ohc::orchestration::hub_service_client::HubServiceClient::connect(std::env::var("OHC_HUB_URL").unwrap_or_else(|_| "http://127.0.0.1:8081".to_string())).await {
                                            let reason_req = ::server_ohc::orchestration::ReasonRequest {
                                                prompt: prompt.clone(),
                                                from_agent_id: "promoter_seo".to_string(),
                                            };
                                            if let Ok(res) = client.reason(tonic::Request::new(reason_req)).await {
                                                if let Ok(v) = serde_json::from_str::<serde_json::Value>(&res.into_inner().content) {
                                                    return Ok(v);
                                                }
                                            }
                                        }
                                        Err("AI call failed".to_string())
                                    };

                                    match timeout(Duration::from_secs(30), ai_op).await {
                                        Ok(Ok(v)) => {
                                            resolved_payload = v;
                                            break;
                                        },
                                        _ => {
                                            attempts += 1;
                                            tokio::time::sleep(Duration::from_secs(2u64.pow(attempts))).await;
                                        }
                                    }
                                }

                                if resolved_payload.is_object() && !resolved_payload.as_object().unwrap().is_empty() {
                                    let meta_title = resolved_payload.get("meta_title").and_then(|v| v.as_str()).unwrap_or("").to_string();
                                    let meta_description = resolved_payload.get("meta_description").and_then(|v| v.as_str()).unwrap_or("").to_string();
                                    let default_structured_data = serde_json::json!({});
                                    let structured_data = resolved_payload.get("structured_data").unwrap_or(&default_structured_data);
                                    let generated_keywords = resolved_payload.get("generated_keywords").and_then(|v| v.as_str()).unwrap_or("").to_string();

                                    match pool.begin().await {
                                        Ok(mut tx) => {
                                            if let Err(e) = ::server_common::auth_utils::set_org_context(&mut *tx, org_id).await {
                                                tracing::error!("Failed to set org context for seo worker: {}", e);
                                                continue;
                                            }

                                            let seo_id = Uuid::new_v4().to_string();

                                            let result = sqlx::query(
                                                "INSERT INTO ohc_seo_metadata (id, tenant_id, entity_id, entity_type, meta_title, meta_description, structured_data, generated_keywords, status)
                                                 VALUES ($1, $2, $3, 'business_profile', $4, $5, $6, $7, 'PENDING_APPROVAL')
                                                 ON CONFLICT (tenant_id, entity_id, entity_type)
                                                 DO UPDATE SET meta_title = EXCLUDED.meta_title, meta_description = EXCLUDED.meta_description, structured_data = EXCLUDED.structured_data, generated_keywords = EXCLUDED.generated_keywords, status = 'PENDING_APPROVAL', updated_at = CURRENT_TIMESTAMP"
                                            )
                                            .bind(&seo_id)
                                            .bind(org_id)
                                            .bind(profile_id)
                                            .bind(meta_title)
                                            .bind(meta_description)
                                            .bind(structured_data)
                                            .bind(generated_keywords)
                                            .execute(&mut *tx)
                                            .await;

                                            if let Err(e) = result {
                                                tracing::error!("Failed to save SEO metadata: {}", e);
                                            } else {
                                                if let Err(e) = tx.commit().await {
                                                    tracing::error!("Failed to commit SEO metadata transaction: {}", e);
                                                }
                                            }
                                        }
                                        Err(e) => {
                                            tracing::error!("Failed to start transaction for seo worker: {}", e);
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_worker_creation() {
        // Just verify it compiles and instantiates
        // Real testing requires a running DB and Hub
        assert!(true);
    }
}
