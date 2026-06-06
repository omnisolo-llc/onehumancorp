use reqwest::Client;
use serde::{Deserialize, Serialize};

pub struct ShippoClient {
    pub api_key: String,
    client: Client,
    pub base_url: String, // added for testing
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Address {
    pub name: String,
    pub street1: String,
    pub city: String,
    pub state: String,
    pub zip: String,
    pub country: String,
}

#[derive(Serialize, Clone, Debug)]
pub struct Parcel {
    pub length: String,
    pub width: String,
    pub height: String,
    pub distance_unit: String,
    pub weight: String,
    pub mass_unit: String,
}

#[derive(Serialize, Clone, Debug)]
pub struct ShipmentRequest {
    pub address_from: Address,
    pub address_to: Address,
    pub parcels: Vec<Parcel>,
    pub async_input: bool,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Rate {
    pub object_id: String,
    pub provider: String,
    pub servicelevel: ServiceLevel,
    pub amount: String,
    pub currency: String,
    pub estimated_days: Option<i32>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct ServiceLevel {
    pub name: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct ShipmentResponse {
    pub rates: Vec<Rate>,
}

#[derive(Serialize, Clone, Debug)]
pub struct TransactionRequest {
    pub rate: String,
    pub label_file_type: String,
    pub async_input: bool,
}

#[derive(Deserialize, Debug, Clone)]
pub struct TransactionResponse {
    pub status: String,
    pub label_url: String,
    pub tracking_number: String,
    pub tracking_url_provider: String,
}

impl ShippoClient {
    pub fn new(api_key: String) -> Self {
        ShippoClient {
            api_key,
            client: Client::new(),
            base_url: "https://api.goshippo.com".to_string(),
        }
    }

    pub async fn fetch_rates(&self, weight: f64, dimensions: &str, address_from: Address, address_to: Address) -> Result<Vec<Rate>, String> {
        let parts: Vec<&str> = dimensions.split('x').collect();
        let (length, width, height) = if parts.len() == 3 {
            (parts[0], parts[1], parts[2])
        } else {
            ("10", "8", "6") // Default fallback
        };

        let req = ShipmentRequest {
            address_from,
            address_to,
            parcels: vec![Parcel {
                length: length.to_string(),
                width: width.to_string(),
                height: height.to_string(),
                distance_unit: "in".to_string(),
                weight: weight.to_string(),
                mass_unit: "oz".to_string(),
            }],
            async_input: false,
        };

        let url = format!("{}/v1/shipments/", self.base_url);
        let res = self
            .client
            .post(&url)
            .header("Authorization", format!("ShippoToken {}", self.api_key))
            .json(&req)
            .send()
            .await
            .map_err(|e| e.to_string())?;

        if !res.status().is_success() {
            let error_text = res.text().await.unwrap_or_default();
            tracing::error!("Shippo fetch_rates error: {}", error_text);
            return Err("Failed to fetch rates from Shippo".to_string());
        }

        let shipment: ShipmentResponse = res.json().await.map_err(|e| e.to_string())?;
        Ok(shipment.rates)
    }

    pub async fn purchase_label(&self, rate_id: &str) -> Result<TransactionResponse, String> {
        let req = TransactionRequest {
            rate: rate_id.to_string(),
            label_file_type: "PDF".to_string(),
            async_input: false,
        };

        let url = format!("{}/v1/transactions/", self.base_url);
        let res = self
            .client
            .post(&url)
            .header("Authorization", format!("ShippoToken {}", self.api_key))
            .json(&req)
            .send()
            .await
            .map_err(|e| e.to_string())?;

        if !res.status().is_success() {
            let error_text = res.text().await.unwrap_or_default();
            tracing::error!("Shippo purchase_label error: {}", error_text);
            return Err("Failed to purchase label from Shippo".to_string());
        }

        let transaction: TransactionResponse = res.json().await.map_err(|e| e.to_string())?;
        Ok(transaction)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_deserialize_shipment_response() {
        let json = r#"{
            "rates": [
                {
                    "object_id": "rate_123",
                    "provider": "USPS",
                    "servicelevel": {
                        "name": "Priority Mail"
                    },
                    "amount": "5.50",
                    "currency": "USD",
                    "estimated_days": 2
                }
            ]
        }"#;

        let response: ShipmentResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.rates.len(), 1);
        assert_eq!(response.rates[0].object_id, "rate_123");
        assert_eq!(response.rates[0].provider, "USPS");
        assert_eq!(response.rates[0].servicelevel.name, "Priority Mail");
        assert_eq!(response.rates[0].amount, "5.50");
        assert_eq!(response.rates[0].currency, "USD");
        assert_eq!(response.rates[0].estimated_days, Some(2));
    }

    #[tokio::test]
    async fn test_deserialize_transaction_response() {
        let json = r#"{
            "status": "SUCCESS",
            "label_url": "https://example.com/label.pdf",
            "tracking_number": "1Z123",
            "tracking_url_provider": "UPS"
        }"#;

        let response: TransactionResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.status, "SUCCESS");
        assert_eq!(response.label_url, "https://example.com/label.pdf");
        assert_eq!(response.tracking_number, "1Z123");
        assert_eq!(response.tracking_url_provider, "UPS");
    }

    #[tokio::test]
    async fn test_client_payload_serialize() {
        let address = Address {
            name: "Test".to_string(),
            street1: "123 Test St".to_string(),
            city: "San Francisco".to_string(),
            state: "CA".to_string(),
            zip: "94105".to_string(),
            country: "US".to_string(),
        };

        let req = ShipmentRequest {
            address_from: address.clone(),
            address_to: address.clone(),
            parcels: vec![Parcel {
                length: "10".to_string(),
                width: "8".to_string(),
                height: "6".to_string(),
                distance_unit: "in".to_string(),
                weight: "16".to_string(),
                mass_unit: "oz".to_string(),
            }],
            async_input: false,
        };

        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains("123 Test St"));
        assert!(json.contains("parcels"));
        assert!(json.contains("10"));
    }
}
