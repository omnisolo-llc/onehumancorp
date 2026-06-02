use axum::{Json, response::IntoResponse, http::StatusCode, extract::State};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use crate::hub::Hub;

#[derive(Deserialize, Debug, Clone)]
pub struct CatalogChangeEvent {
    pub product_id: String,
    pub price: Option<f64>,
    pub stock: Option<i32>,
    pub title: Option<String>,
    pub description: Option<String>,
    pub attributes: std::collections::HashMap<String, String>,
}

#[derive(Deserialize, Debug)]
pub struct CatalogSyncRequest {
    pub changes: Vec<CatalogChangeEvent>,
}

#[derive(Serialize)]
pub struct CatalogSyncResponse {
    pub success: bool,
    pub jobs_dispatched: i32,
}

pub async fn catalog_sync_handler(
    State(hub): State<Arc<Hub>>,
    headers: axum::http::HeaderMap,
    Json(payload): Json<CatalogSyncRequest>,
) -> impl IntoResponse {
    let spiffe_id_str = headers.get("x-spiffe-id").and_then(|v| v.to_str().ok()).unwrap_or("");
    let (tenant_id, _) = crate::auth::parse_spiffe_id(spiffe_id_str).unwrap_or(("".to_string(), "".to_string()));

    if tenant_id.is_empty() {
        return (
            StatusCode::UNAUTHORIZED,
            Json(CatalogSyncResponse { success: false, jobs_dispatched: 0 }),
        ).into_response();
    }

    let mut jobs_dispatched = 0;

    for change in &payload.changes {
        // Map attributes for different channels
        let mapped_instagram = map_attributes_instagram(&change.attributes);
        let mapped_pos = map_attributes_pos(&change.attributes);

        // Dispatch sync jobs (simulated)
        if !mapped_instagram.is_empty() {
            jobs_dispatched += 1;
            // publish job to mesh or queue
        }
        if !mapped_pos.is_empty() {
            jobs_dispatched += 1;
            // publish job to mesh or queue
        }

        let event = ::server_ohc::orchestration::TeammateMeshEvent {
            action: "CatalogSynced".to_string(),
            agent_id: "operations".to_string(),
            status: "success".to_string(),
            msg_id: uuid::Uuid::new_v4().to_string(),
            payload: serde_json::json!({
                "product_id": change.product_id,
                "tenant_id": tenant_id,
                "channels_synced": ["instagram", "pos"]
            }).to_string().into_bytes(),
        };
        let _ = hub.publish_teammate_event("mesh:catalog:synced".to_string(), event);
    }

    (
        StatusCode::OK,
        Json(CatalogSyncResponse { success: true, jobs_dispatched }),
    ).into_response()
}

fn map_attributes_instagram(attrs: &std::collections::HashMap<String, String>) -> std::collections::HashMap<String, String> {
    let mut mapped = std::collections::HashMap::new();
    for (k, v) in attrs {
        let key = k.to_lowercase();
        if key == "color" || key == "colour" {
            mapped.insert("fb_product_color".to_string(), v.clone());
        } else if key == "size" {
            mapped.insert("fb_product_size".to_string(), v.clone());
        } else {
            mapped.insert(k.clone(), v.clone());
        }
    }
    mapped
}

fn map_attributes_pos(attrs: &std::collections::HashMap<String, String>) -> std::collections::HashMap<String, String> {
    let mut mapped = std::collections::HashMap::new();
    for (k, v) in attrs {
        mapped.insert(format!("pos_{}", k), v.clone());
    }
    mapped
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_map_attributes_instagram() {
        let mut attrs = HashMap::new();
        attrs.insert("Colour".to_string(), "Crimson".to_string());
        attrs.insert("Size".to_string(), "XL".to_string());

        let mapped = map_attributes_instagram(&attrs);

        assert_eq!(mapped.get("fb_product_color").unwrap(), "Crimson");
        assert_eq!(mapped.get("fb_product_size").unwrap(), "XL");
    }

    #[test]
    fn test_map_attributes_pos() {
        let mut attrs = HashMap::new();
        attrs.insert("Sku".to_string(), "12345".to_string());

        let mapped = map_attributes_pos(&attrs);

        assert_eq!(mapped.get("pos_Sku").unwrap(), "12345");
    }
}
