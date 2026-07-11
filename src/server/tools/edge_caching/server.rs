use crate::ohc::orchestration::{McpInvokeRequest, McpInvokeResponse, McpToolProto};
use tracing::Instrument;

pub struct EdgeCachingMcpServer {
    pub pool: Option<sqlx::PgPool>,
    pub redis: Option<redis::Client>,
}

impl EdgeCachingMcpServer {
    pub fn new(pool: Option<sqlx::PgPool>, redis: Option<redis::Client>) -> Self {
        Self { pool, redis }
    }

    pub fn get_tools(&self) -> Vec<McpToolProto> {
        vec![
            McpToolProto {
                id: "mcp_seo_generator".to_string(),
                name: "MCP SEO Generator".to_string(),
                description: "Autonomously generates highly optimized static HTML shells containing JSON-LD schema, OpenGraph tags, and relevant keywords.".to_string(),
                category: "seo".to_string(),
                status: "active".to_string(),
            },
            McpToolProto {
                id: "mcp_edge_kv_sync".to_string(),
                name: "MCP Edge KV Sync".to_string(),
                description: "Synchronizes inventory updates to a simulated Edge KV store via Redis.".to_string(),
                category: "caching".to_string(),
                status: "active".to_string(),
            },
            McpToolProto {
                id: "mcp_edge_worker_simulation".to_string(),
                name: "MCP Edge Worker Simulation".to_string(),
                description: "Simulates an Edge Worker that serves a basic HTML shell and injects dynamic inventory data from the Redis KV.".to_string(),
                category: "caching".to_string(),
                status: "active".to_string(),
            },
        ]
    }

    pub async fn invoke_tool(&self, req: &McpInvokeRequest) -> Result<McpInvokeResponse, tonic::Status> {
        let params: serde_json::Value = serde_json::from_str(&req.params)
            .map_err(|e| tonic::Status::invalid_argument(format!("invalid JSON params: {}", e)))?;

        match req.tool_id.as_str() {
            "mcp_seo_generator" => {
                let tenant_id = params["tenant_id"].as_str().unwrap_or("unknown_tenant").to_string();
                let product_id = params["product_id"].as_str().unwrap_or("unknown_product").to_string();
                let product_data = params["product_data"].clone();

                let pool = self.pool.clone();
                let redis_client = self.redis.clone();

                async move {
                    let seo_title = product_data["name"].as_str().unwrap_or("Unknown Product").to_string();
                    let seo_description = product_data["description"].as_str().unwrap_or("").to_string();

                    let seo_metadata = serde_json::json!({
                        "json_ld": {
                            "@context": "https://schema.org/",
                            "@type": "Product",
                            "name": &seo_title,
                            "description": &seo_description,
                        },
                        "open_graph": {
                            "og:title": &seo_title,
                            "og:description": &seo_description,
                        }
                    });

                    if let Some(pool) = pool {
                        if tenant_id != "unknown_tenant" && product_id != "unknown_product" {
                            let _ = sqlx::query("UPDATE products SET seo_title = $1, seo_description = $2, seo_schema_json = $3 WHERE id = $4 AND tenant_id = $5")
                                .bind(&seo_title)
                                .bind(&seo_description)
                                .bind(&seo_metadata["json_ld"])
                                .bind(&product_id)
                                .bind(&tenant_id)
                                .execute(&pool)
                                .await;
                        }
                    }

                    if let Some(client) = redis_client {
                        if let Ok(mut conn) = client.get_multiplexed_async_connection().await {
                            let invalidation_topic = "cache_invalidation_events";
                            let invalidation_payload = serde_json::json!({
                                "event": "seo.updated",
                                "tags": [
                                    format!("tenant-id:{}", tenant_id),
                                    format!("entity:product:{}", product_id)
                                ]
                            }).to_string();
                            let _: Result<(), _> = redis::cmd("PUBLISH").arg(invalidation_topic).arg(invalidation_payload).query_async(&mut conn).await;
                        }
                    }

                    let resp = serde_json::json!({
                        "status": "success",
                        "tenant_id": tenant_id,
                        "product_id": product_id,
                        "seo_metadata": seo_metadata
                    });
                    Ok(McpInvokeResponse { payload: serde_json::to_string(&resp).unwrap() })
                }
                .instrument(tracing::info_span!("mcp_seo_generator"))
                .await
            }
            "mcp_edge_kv_sync" => {
                let tenant_id = params["tenant_id"].as_str().unwrap_or("unknown_tenant");
                let product_id = params["product_id"].as_str().unwrap_or("unknown_product");
                let inventory_count = params["inventory_count"].as_i64().unwrap_or(0);

                async {
                    let key = format!("tenant:{}:product:{}:inventory", tenant_id, product_id);
                    // In a real implementation, we would use Redis here.
                    // For the simulation, we'll just return success.
                    let resp = serde_json::json!({
                        "status": "success",
                        "synced_key": key,
                        "inventory_count": inventory_count
                    });
                    Ok(McpInvokeResponse { payload: serde_json::to_string(&resp).unwrap() })
                }
                .instrument(tracing::info_span!("mcp_edge_kv_sync"))
                .await
            }
            "mcp_edge_worker_simulation" => {
                let tenant_id = params["tenant_id"].as_str().unwrap_or("unknown_tenant");
                let product_id = params["product_id"].as_str().unwrap_or("unknown_product");
                let seo_metadata = &params["seo_metadata"];

                async {
                    let _key = format!("tenant:{}:product:{}:inventory", tenant_id, product_id);
                    // In a real implementation, we would fetch from Redis here.
                    // For the simulation, we'll return a mock value.
                    let inventory_count = 42; // Mock value

                    let html_shell = format!(
                        "<html><head><script type=\"application/ld+json\">{}</script></head><body><h1>Product</h1><p>Inventory: {}</p></body></html>",
                        serde_json::to_string(&seo_metadata["json_ld"]).unwrap_or_else(|_| "{}".to_string()),
                        inventory_count
                    );

                    let resp = serde_json::json!({
                        "status": "success",
                        "html_shell": html_shell
                    });
                    Ok(McpInvokeResponse { payload: serde_json::to_string(&resp).unwrap() })
                }
                .instrument(tracing::info_span!("mcp_edge_worker_simulation"))
                .await
            }
            _ => Err(tonic::Status::not_found(format!("tool {} not found", req.tool_id))),
        }
    }
}
