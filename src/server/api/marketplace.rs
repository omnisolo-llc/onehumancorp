use axum::{
    extract::{Query, State},
    response::IntoResponse,
    Json,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use crate::hub::Hub;
use ::server_ohc::orchestration::org_service_server::OrgService;

#[derive(Deserialize)]
pub struct MarketplaceQuery {
    #[serde(default)]
    pub q: String,
}

#[derive(Serialize)]
pub struct AgentItem {
    pub id: String,
    pub name: String,
    pub description: String,
    pub author: String,
    pub version: String,
    pub endpoint: String,
}

pub async fn marketplace_handler(
    State(hub): State<Arc<Hub>>,
    Query(params): Query<MarketplaceQuery>,
) -> impl IntoResponse {
    let service = crate::services::org::service::MyOrgService::new(hub.clone());

    let request = tonic::Request::new(::server_ohc::orchestration::EmptyRequest {});
    let response = match service.get_marketplace_items(request).await {
        Ok(res) => res.into_inner(),
        Err(_) => return Json(vec![] as Vec<AgentItem>).into_response(),
    };

    let query_lower = params.q.to_lowercase();

    let mut filtered_items = Vec::new();
    for item in response.items {
        if item.name.to_lowercase().contains(&query_lower)
            || item.description.to_lowercase().contains(&query_lower)
            || query_lower.is_empty()
        {
            filtered_items.push(AgentItem {
                id: item.id,
                name: item.name,
                description: item.description,
                author: item.author,
                version: "1.0.0".to_string(), // Default version as it's not in proto
                endpoint: format!("https://marketplace.example.com/agents/{}", item.r#type),
            });
        }
    }

    Json(filtered_items).into_response()
}
