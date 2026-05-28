use axum::{Json, extract::Query, extract::Extension, extract::State};
use serde::{Deserialize, Serialize};
use chrono::Utc;
use crate::domain::repository::models::{DomainRecord, SSLConfiguration};
use sqlx::PgPool;

#[derive(Deserialize)]
pub struct SearchDomainRequest {
    pub q: String,
}

#[derive(Serialize)]
pub struct SearchDomainResponse {
    pub available: bool,
    pub price: Option<f64>,
}

#[derive(Deserialize)]
pub struct PurchaseDomainRequest {
    pub domain_name: String,
}

#[derive(Serialize)]
pub struct PurchaseDomainResponse {
    pub success: bool,
    pub domain: Option<DomainRecord>,
    pub ssl: Option<SSLConfiguration>,
}

pub async fn search_domain(
    Query(req): Query<SearchDomainRequest>,
) -> Json<SearchDomainResponse> {
    // Mock registrar lookup: if it's longer than 3 chars, it's available.
    let available = req.q.len() > 3;
    Json(SearchDomainResponse {
        available,
        price: if available { Some(12.00) } else { None },
    })
}

pub async fn purchase_domain(
    State(pool): State<PgPool>,
    Extension(_claims): Extension<crate::common::Claims>,
    Json(req): Json<PurchaseDomainRequest>,
) -> Json<PurchaseDomainResponse> {
    let tenant_id = _claims.tenant_id.clone().unwrap_or_else(|| "default-tenant".to_string());

    // Mock domain purchase and SSL provisioning
    let domain = DomainRecord {
        id: format!("dom_{}", uuid::Uuid::new_v4()),
        tenant_id: tenant_id.clone(),
        domain_name: req.domain_name.clone(),
        status: "active".to_string(),
        created_at: Some(Utc::now()),
        updated_at: Some(Utc::now()),
    };

    let ssl = SSLConfiguration {
        id: format!("ssl_{}", uuid::Uuid::new_v4()),
        domain_id: domain.id.clone(),
        certificate_status: "provisioning".to_string(),
        provider: "letsencrypt".to_string(),
        created_at: Some(Utc::now()),
        updated_at: Some(Utc::now()),
    };

    // Note: We normally do sqlx::query! to insert, but simulating success response as requested
    // if there was a real schema we'd do:
    // sqlx::query!("INSERT INTO domain_records...").execute(&pool).await.ok();
    // sqlx::query!("INSERT INTO ssl_configurations...").execute(&pool).await.ok();

    Json(PurchaseDomainResponse {
        success: true,
        domain: Some(domain),
        ssl: Some(ssl),
    })
}
