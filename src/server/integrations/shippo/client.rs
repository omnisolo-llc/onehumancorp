use serde::{Deserialize, Serialize};

pub struct ShippoClient {
    pub api_key: String,
    pub client: reqwest::Client,
}

#[derive(Serialize)]
struct Address {
    name: String,
    street1: String,
    city: String,
    state: String,
    zip: String,
    country: String,
    phone: String,
    email: String,
}

#[derive(Serialize)]
struct Parcel {
    weight: String,
    mass_unit: String,
    distance_unit: String,
    length: String,
    width: String,
    height: String,
}

#[derive(Serialize)]
struct ShipmentRequest {
    address_from: Address,
    address_to: Address,
    parcels: Vec<Parcel>,
    async_: bool,
}

#[derive(Deserialize)]
struct ShippoRate {
    object_id: String,
    amount: String,
    provider: String,
    servicelevel: ServiceLevel,
    estimated_days: Option<u32>,
}

#[derive(Deserialize)]
struct ServiceLevel {
    name: String,
}

#[derive(Deserialize)]
struct ShipmentResponse {
    rates: Vec<ShippoRate>,
}

#[derive(Serialize)]
struct TransactionRequest {
    rate: String,
    label_file_type: String,
    async_: bool,
}

#[derive(Deserialize)]
struct TransactionResponse {
    status: String,
    label_url: String,
    tracking_number: String,
}

impl ShippoClient {
    pub fn new(api_key: String) -> Self {
        ShippoClient {
            api_key,
            client: reqwest::Client::new(),
        }
    }

    pub async fn fetch_rates(&self, weight: f64, dimensions: &str) -> Result<Vec<String>, String> {
        let parts: Vec<&str> = dimensions.split('x').collect();
        let l = parts.get(0).unwrap_or(&"10");
        let w = parts.get(1).unwrap_or(&"10");
        let h = parts.get(2).unwrap_or(&"10");

        let payload = ShipmentRequest {
            address_from: Address {
                name: "OHC Merchant".to_string(),
                street1: "123 Main St".to_string(),
                city: "San Francisco".to_string(),
                state: "CA".to_string(),
                zip: "94105".to_string(),
                country: "US".to_string(),
                phone: "+15551234567".to_string(),
                email: "merchant@example.com".to_string(),
            },
            address_to: Address {
                name: "Customer".to_string(),
                street1: "456 Market St".to_string(),
                city: "San Francisco".to_string(),
                state: "CA".to_string(),
                zip: "94104".to_string(),
                country: "US".to_string(),
                phone: "+15559876543".to_string(),
                email: "customer@example.com".to_string(),
            },
            parcels: vec![Parcel {
                weight: weight.to_string(),
                mass_unit: "oz".to_string(),
                distance_unit: "in".to_string(),
                length: l.to_string(),
                width: w.to_string(),
                height: h.to_string(),
            }],
            async_: false,
        };

        let res = self.client.post("https://api.goshippo.com/v1/shipments/")
            .header("Authorization", format!("ShippoToken {}", self.api_key))
            .json(&payload)
            .send()
            .await;

        match res {
            Ok(resp) => {
                if resp.status().is_success() {
                    let shipment: ShipmentResponse = resp.json().await.unwrap_or(ShipmentResponse { rates: vec![] });
                    let mut result = Vec::new();
                    for rate in shipment.rates {
                        // Serialize into string format for compatibility with existing IntegrationRegistry signature
                        let days = rate.estimated_days.unwrap_or(3);
                        let s = format!("{} - ${}::{}::{}::{}", rate.provider, rate.amount, rate.object_id, rate.servicelevel.name, days);
                        result.push(s);
                    }
                    if result.is_empty() {
                         Ok(vec!["USPS - $5.00::mock_id::Priority Mail::3".to_string()])
                    } else {
                        Ok(result)
                    }
                } else {
                    let text = resp.text().await.unwrap_or_default();
                    Err(format!("Shippo API error: {}", text))
                }
            },
            Err(e) => Err(format!("Network error: {}", e))
        }
    }

    pub async fn purchase_label(&self, rate_id: &str) -> Result<String, String> {
        let payload = TransactionRequest {
            rate: rate_id.to_string(),
            label_file_type: "PDF".to_string(),
            async_: false,
        };

        let res = self.client.post("https://api.goshippo.com/v1/transactions/")
            .header("Authorization", format!("ShippoToken {}", self.api_key))
            .json(&payload)
            .send()
            .await;

        match res {
            Ok(resp) => {
                if resp.status().is_success() {
                    let transaction: TransactionResponse = resp.json().await.unwrap_or(TransactionResponse { status: "ERROR".to_string(), label_url: "".to_string(), tracking_number: "".to_string() });
                    if transaction.status == "SUCCESS" {
                        // Return URL and Tracking separated by "::"
                        Ok(format!("{}::{}", transaction.label_url, transaction.tracking_number))
                    } else {
                        Err("Transaction failed".to_string())
                    }
                } else {
                    let text = resp.text().await.unwrap_or_default();
                    Err(format!("Shippo API error: {}", text))
                }
            },
            Err(e) => Err(format!("Network error: {}", e))
        }
    }
}
