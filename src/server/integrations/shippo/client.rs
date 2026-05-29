use reqwest::Client;
use serde_json::json;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct ShippoRate {
    #[allow(dead_code)]
    object_id: String,
    provider: String,
    amount: String,
}

#[derive(Deserialize, Debug)]
struct ShippoRatesResponse {
    rates: Vec<ShippoRate>,
}

pub struct ShippoClient {
    pub api_key: String,
    http_client: Client,
}

impl ShippoClient {
    pub fn new(api_key: String) -> Self {
        ShippoClient {
            api_key,
            http_client: Client::new(),
        }
    }

    pub async fn fetch_rates(&self, weight: f64, _dimensions: &str) -> Result<Vec<String>, String> {
        let url = "https://api.goshippo.com/shipments/";

        let payload = json!({
            "address_to": {
                "name": "Customer",
                "country": "US"
            },
            "address_from": {
                "name": "Store",
                "country": "US"
            },
            "parcels": [{
                "weight": weight.to_string(),
                "distance_unit": "in",
                "mass_unit": "lb"
            }]
        });

        let res = self.http_client.post(url)
            .header("Authorization", format!("ShippoToken {}", self.api_key))
            .json(&payload)
            .send()
            .await;

        match res {
            Ok(resp) => {
                if resp.status().is_success() {
                    let data: ShippoRatesResponse = resp.json().await.map_err(|e| e.to_string())?;
                    let formatted_rates = data.rates.into_iter()
                        .map(|r| format!("{} - ${}", r.provider, r.amount))
                        .collect();
                    Ok(formatted_rates)
                } else {
                    Err(format!("Shippo API error: {}", resp.status()))
                }
            }
            Err(e) => Err(format!("Network error: {}", e)),
        }
    }

    pub async fn purchase_label(&self, rate_id: &str) -> Result<String, String> {
        let url = "https://api.goshippo.com/transactions/";

        let payload = json!({
            "rate": rate_id,
            "label_file_type": "PDF",
            "async": false
        });

        let res = self.http_client.post(url)
            .header("Authorization", format!("ShippoToken {}", self.api_key))
            .json(&payload)
            .send()
            .await;

        match res {
            Ok(resp) => {
                if resp.status().is_success() {
                    // Extracting label_url from real response schema
                    #[derive(Deserialize)]
                    struct TxnResp { label_url: String }
                    let data: TxnResp = resp.json().await.map_err(|e| e.to_string())?;
                    Ok(data.label_url)
                } else {
                    Err(format!("Shippo API error: {}", resp.status()))
                }
            }
            Err(e) => Err(format!("Network error: {}", e)),
        }
    }
}
