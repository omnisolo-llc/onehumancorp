use axum::{
    routing::{get, post},
    Router, Json, extract::Query,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DomainRecord {
    pub domain_name: String,
    pub status: String,
    pub price_cents: Option<i64>,
    pub registration_date: Option<String>,
    pub expiry_date: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SSLConfiguration {
    pub domain_name: String,
    pub status: String,
    pub issuer: String,
    pub issued_at: Option<String>,
}


#[derive(Deserialize)]
pub struct SearchQuery {
    pub domain: String,
}

pub async fn search_domain(Query(query): Query<SearchQuery>) -> Json<serde_json::Value> {
    let domain = query.domain.to_lowercase();
    let is_available = !domain.contains("taken");
    let price_cents = if is_available { Some(1200) } else { None };

    let record = DomainRecord {
        domain_name: domain.clone(),
        status: if is_available { "available".to_string() } else { "unavailable".to_string() },
        price_cents,
        registration_date: None,
        expiry_date: None,
    };

    Json(serde_json::json!({
        "success": true,
        "domain": record
    }))
}

#[derive(Deserialize)]
pub struct PurchaseRequest {
    pub domain: String,
}

pub async fn purchase_domain(Json(req): Json<PurchaseRequest>) -> Json<serde_json::Value> {
    let domain = req.domain.to_lowercase();

    // Simulate purchase
    let record = DomainRecord {
        domain_name: domain.clone(),
        status: "registered".to_string(),
        price_cents: Some(1200),
        registration_date: Some("2024-05-30T00:00:00Z".to_string()),
        expiry_date: Some("2025-05-30T00:00:00Z".to_string()),
    };

    Json(serde_json::json!({
        "success": true,
        "domain": record,
        "message": "Domain purchased successfully."
    }))
}

#[derive(Deserialize)]
pub struct ConfigureRequest {
    pub domain: String,
}

pub async fn configure_dns(Json(req): Json<ConfigureRequest>) -> Json<serde_json::Value> {
    let domain = req.domain.to_lowercase();

    // Simulate DNS configuration and SSL provisioning
    let ssl = SSLConfiguration {
        domain_name: domain.clone(),
        status: "active".to_string(),
        issuer: "Let's Encrypt".to_string(),
        issued_at: Some("2024-05-30T00:01:00Z".to_string()),
    };

    Json(serde_json::json!({
        "success": true,
        "ssl": ssl,
        "message": "DNS configured and SSL provisioned successfully."
    }))
}

pub fn router<S: Clone + Send + Sync + 'static>() -> Router<S> {
    Router::new()
        .route("/search", get(search_domain))
        .route("/purchase", post(purchase_domain))
        .route("/configure", post(configure_dns))
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::extract::Query;
    use axum::Json;

    #[tokio::test]
    async fn test_search_domain_available() {
        let query = Query(SearchQuery { domain: "example.com".to_string() });
        let response = search_domain(query).await;

        assert_eq!(response.get("success").unwrap().as_bool(), Some(true));

        let domain_record = response.get("domain").unwrap();
        assert_eq!(domain_record.get("status").unwrap().as_str(), Some("available"));
        assert_eq!(domain_record.get("price_cents").unwrap().as_i64(), Some(1200));
    }

    #[tokio::test]
    async fn test_search_domain_taken() {
        let query = Query(SearchQuery { domain: "taken.com".to_string() });
        let response = search_domain(query).await;

        assert_eq!(response.get("success").unwrap().as_bool(), Some(true));

        let domain_record = response.get("domain").unwrap();
        assert_eq!(domain_record.get("status").unwrap().as_str(), Some("unavailable"));
        assert_eq!(domain_record.get("price_cents").unwrap().as_i64(), None);
    }

    #[tokio::test]
    async fn test_purchase_domain() {
        let req = Json(PurchaseRequest { domain: "example.com".to_string() });
        let response = purchase_domain(req).await;

        assert_eq!(response.get("success").unwrap().as_bool(), Some(true));

        let domain_record = response.get("domain").unwrap();
        assert_eq!(domain_record.get("status").unwrap().as_str(), Some("registered"));
        assert_eq!(domain_record.get("price_cents").unwrap().as_i64(), Some(1200));
    }

    #[tokio::test]
    async fn test_configure_dns() {
        let req = Json(ConfigureRequest { domain: "example.com".to_string() });
        let response = configure_dns(req).await;

        assert_eq!(response.get("success").unwrap().as_bool(), Some(true));

        let ssl_record = response.get("ssl").unwrap();
        assert_eq!(ssl_record.get("status").unwrap().as_str(), Some("active"));
        assert_eq!(ssl_record.get("issuer").unwrap().as_str(), Some("Let's Encrypt"));
    }
}
