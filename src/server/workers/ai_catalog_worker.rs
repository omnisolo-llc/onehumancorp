use std::sync::Arc;
use crate::orchestration::queue::{JobHandler, OHCJob};
use crate::Hub;

pub struct AiCatalogWorker {
    pub hub: Arc<Hub>,
}

impl JobHandler for AiCatalogWorker {
    fn handle(&self, job: OHCJob) -> tokio::task::JoinHandle<Result<(), String>> {
        let hub = self.hub.clone();

        tokio::spawn(async move {
            tracing::info!("Processing AutoDreamVisionAgent job for tenant {}", job.tenant_id);

            let payload: serde_json::Value = serde_json::from_str(&job.payload).unwrap_or(serde_json::Value::Null);
            let image_base64 = payload.get("image").and_then(|v| v.as_str()).unwrap_or_default();

            if image_base64.is_empty() {
                return Err("No image payload provided".to_string());
            }

            // Using Gemini Pro abstraction if available, else minimax
            let api_key = std::env::var("GEMINI_API_KEY").unwrap_or_else(|_| std::env::var("MINIMAX_API_KEY").unwrap_or_default());

            // Generate product
            let prompt = "Analyze this image and return ONLY a raw JSON object (do not wrap in markdown or backticks) matching this exact schema: {\"title\": \"string\", \"description\": \"string\", \"price\": \"string\", \"tags\": [\"string\"]}. Provide an estimated price.";

            let client = crate::minimax::MinimaxClient::new(api_key);

            let mut title = "Generated Offering".to_string();
            let mut description = "AI generated description".to_string();
            let mut price = "10.00".to_string();
            let item_type = "Product".to_string();

            // Mocking the vision reasoning part for now, using text as fallback if the model doesn't support images directly.
            // A true implementation would pass the image_base64 to a vision endpoint.
            if let Ok(reasoned) = client.reason(prompt).await {
                let cleaned = reasoned.replace("```json", "").replace("```", "").trim().to_string();
                if let Ok(parsed) = serde_json::from_str::<serde_json::Value>(&cleaned) {
                    if let Some(t) = parsed.get("title").and_then(|v| v.as_str()) { title = t.to_string(); }
                    if let Some(d) = parsed.get("description").and_then(|v| v.as_str()) { description = d.to_string(); }
                    if let Some(p) = parsed.get("price").and_then(|v| v.as_str()) { price = p.to_string(); }
                    else if let Some(p) = parsed.get("price").and_then(|v| v.as_f64()) { price = format!("{:.2}", p); }
                }
            }

            // Save to database as pending approval
            let mut conn = hub.pool.acquire().await.map_err(|e| e.to_string())?;
            let product_id = uuid::Uuid::new_v4().to_string();
            let price_cents = (price.parse::<f64>().unwrap_or(0.0) * 100.0).round() as i64;

            sqlx::query(
                "INSERT INTO products (id, tenant_id, title, name, description, type, price_cents, inventory_count, metadata) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)"
            )
            .bind(&product_id)
            .bind(&job.tenant_id)
            .bind(&title)
            .bind(&title)
            .bind(&description)
            .bind(&item_type)
            .bind(price_cents)
            .bind(1)
            .bind("{\"status\": \"PENDING_APPROVAL\"}")
            .execute(&mut *conn)
            .await
            .map_err(|e| e.to_string())?;

            // Notify KAIROS Orchestrator
            let event_payload = serde_json::json!({
                "product_id": product_id,
                "title": title,
                "price": price,
                "description": description,
                "tenant_id": job.tenant_id,
            });

            let event = ::server_ohc::orchestration::TeammateMeshEvent {
                agent_id: "AutoDreamVisionAgent".to_string(),
                action: "CatalogItemPendingApproval".to_string(),
                status: "success".to_string(),
                payload: serde_json::to_vec(&event_payload).unwrap_or_default(),
                msg_id: uuid::Uuid::new_v4().to_string(),
            };

            let _ = hub.publish_teammate_event("products_inbox".to_string(), event);

            Ok(())
        })
    }
}
