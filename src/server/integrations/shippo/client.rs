use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShippoRate {
    pub id: String,
    pub carrier: String,
    pub service: String,
    pub amount: String,
    pub days: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PurchaseLabelResponse {
    pub success: bool,
    #[serde(rename = "labelUrl")]
    pub label_url: String,
    #[serde(rename = "trackingNumber")]
    pub tracking_number: String,
    pub carrier: String,
}

pub struct ShippoClient {
    pub api_key: String,
}

impl ShippoClient {
    pub fn new(api_key: String) -> Self {
        ShippoClient { api_key }
    }

    pub async fn fetch_rates(&self, _weight: f64, _dimensions: &str) -> Result<Vec<ShippoRate>, String> {
        Ok(vec![
            ShippoRate {
                id: "rate_usps_1".to_string(),
                carrier: "USPS".to_string(),
                service: "Priority Mail".to_string(),
                amount: "8.50".to_string(),
                days: 2,
            },
            ShippoRate {
                id: "rate_usps_2".to_string(),
                carrier: "USPS".to_string(),
                service: "First-Class Mail".to_string(),
                amount: "4.20".to_string(),
                days: 4,
            },
            ShippoRate {
                id: "rate_ups_1".to_string(),
                carrier: "UPS".to_string(),
                service: "Ground".to_string(),
                amount: "9.75".to_string(),
                days: 3,
            },
        ])
    }

    pub async fn purchase_label(&self, rate_id: &str) -> Result<PurchaseLabelResponse, String> {
        let carrier = if rate_id.contains("ups") { "UPS".to_string() } else { "USPS".to_string() };
        Ok(PurchaseLabelResponse {
            success: true,
            label_url: "https://api.goshippo.com/v1/mock_label.pdf".to_string(),
            tracking_number: format!("1Z999999999999999{}", 123),
            carrier,
        })
    }
}
