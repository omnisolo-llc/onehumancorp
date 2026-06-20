use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ShippingRate {
    pub carrier: String,
    pub service_level: String,
    pub cost_cents: i64,
    pub estimated_days: u32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ShippingLabel {
    pub tracking_number: String,
    pub label_url: String,
    pub carrier: String,
    pub cost_cents: i64,
}

pub struct MockCarrierClient;

impl MockCarrierClient {
    pub async fn get_rates(&self, _tenant_id: &str, _weight_grams: i64) -> Result<Vec<ShippingRate>, String> {
        Ok(vec![
            ShippingRate {
                carrier: "USPS".to_string(),
                service_level: "Priority".to_string(),
                cost_cents: 850,
                estimated_days: 3,
            },
            ShippingRate {
                carrier: "FedEx".to_string(),
                service_level: "Express".to_string(),
                cost_cents: 2400,
                estimated_days: 1,
            },
            ShippingRate {
                carrier: "UPS".to_string(),
                service_level: "Ground".to_string(),
                cost_cents: 1100,
                estimated_days: 4,
            },
        ])
    }

    pub async fn generate_label(&self, _tenant_id: &str, carrier: &str) -> Result<ShippingLabel, String> {
        let tracking_number = format!("1Z9999999999999999{}", rand::random::<u16>());
        let cost_cents = match carrier {
            "FedEx" => 2400,
            "UPS" => 1100,
            _ => 850, // default USPS
        };

        Ok(ShippingLabel {
            tracking_number,
            label_url: "https://mock-carrier.com/label/12345.pdf".to_string(),
            carrier: carrier.to_string(),
            cost_cents,
        })
    }
}
