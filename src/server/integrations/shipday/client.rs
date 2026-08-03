use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ShipdayCreateDeliveryRequest {
    pub order_number: String,
    pub customer_name: String,
    pub customer_address: String,
    pub customer_phone_number: String,
    pub customer_email: Option<String>,
    pub pickup_name: String,
    pub pickup_address: String,
    pub delivery_instruction: Option<String>,
    pub order_item: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ShipdayDelivery {
    pub order_id: String,
    pub order_number: String,
    pub tracking_id: Option<String>,
    pub tracking_url: Option<String>,
    pub status: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ShipdayDeliveryStatus {
    pub tracking_id: String,
    pub status: String,
    pub eta: Option<String>,
    pub driver_name: Option<String>,
    pub driver_latitude: Option<f64>,
    pub driver_longitude: Option<f64>,
    pub tracking_url: Option<String>,
}

pub struct ShipdayClient {
    api_key: String,
    api_base: String,
    http_client: reqwest::Client,
}

impl ShipdayClient {
    pub fn new(api_key: String) -> Self {
        Self {
            api_key,
            api_base: Self::configured_api_base(),
            http_client: reqwest::Client::new(),
        }
    }

    #[cfg(test)]
    pub fn with_base_url_for_test(api_key: String, api_base: String) -> Self {
        Self {
            api_key,
            api_base: api_base.trim_end_matches('/').to_string(),
            http_client: reqwest::Client::new(),
        }
    }

    fn configured_api_base() -> String {
        std::env::var("SHIPDAY_API_BASE")
            .unwrap_or_else(|_| "https://api.shipday.com".to_string())
            .trim_end_matches('/')
            .to_string()
    }

    fn validate_credentials(&self) -> Result<&str, String> {
        let token = self.api_key.trim();
        let lowered = token.to_ascii_lowercase();
        if token.is_empty()
            || lowered.contains("dummy")
            || lowered.contains("mock")
            || lowered.contains("fake")
            || lowered.contains("placeholder")
            || lowered.contains("example")
        {
            return Err("Shipday API key is required".to_string());
        }
        Ok(token)
    }

    fn require_non_empty(value: &str, field: &str) -> Result<(), String> {
        if value.trim().is_empty() {
            return Err(format!("{field} is required"));
        }
        Ok(())
    }

    fn delivery_payload(
        request: &ShipdayCreateDeliveryRequest,
    ) -> Result<serde_json::Value, String> {
        Self::require_non_empty(&request.order_number, "Shipday order number")?;
        Self::require_non_empty(&request.customer_name, "Shipday customer name")?;
        Self::require_non_empty(&request.customer_address, "Shipday customer address")?;
        Self::require_non_empty(
            &request.customer_phone_number,
            "Shipday customer phone number",
        )?;
        Self::require_non_empty(&request.pickup_name, "Shipday pickup name")?;
        Self::require_non_empty(&request.pickup_address, "Shipday pickup address")?;

        let mut payload = json!({
            "orderNumber": request.order_number,
            "customerName": request.customer_name,
            "customerAddress": request.customer_address,
            "customerPhoneNumber": request.customer_phone_number,
            "restaurantName": request.pickup_name,
            "restaurantAddress": request.pickup_address,
        });

        if let Some(email) = request
            .customer_email
            .as_deref()
            .filter(|value| !value.trim().is_empty())
        {
            payload["customerEmail"] = json!(email);
        }
        if let Some(instruction) = request
            .delivery_instruction
            .as_deref()
            .filter(|value| !value.trim().is_empty())
        {
            payload["deliveryInstruction"] = json!(instruction);
        }
        if let Some(order_item) = request
            .order_item
            .as_deref()
            .filter(|value| !value.trim().is_empty())
        {
            payload["orderItem"] = json!(order_item);
        }

        Ok(payload)
    }

    pub async fn create_delivery(
        &self,
        request: ShipdayCreateDeliveryRequest,
    ) -> Result<ShipdayDelivery, String> {
        let token = self.validate_credentials()?;
        let payload = Self::delivery_payload(&request)?;

        let resp = self
            .http_client
            .post(format!("{}/orders", self.api_base))
            .header("Accept", "application/json")
            .header("Authorization", format!("Basic {token}"))
            .header("Content-Type", "application/json")
            .header("x-api-key", token)
            .json(&payload)
            .send()
            .await
            .map_err(|e| format!("Shipday order request failed: {e}"))?;

        let status = resp.status();
        let body: serde_json::Value = resp
            .json()
            .await
            .map_err(|e| format!("Shipday order response was not JSON: {e}"))?;
        if !status.is_success() {
            return Err(format!("Shipday order API error {status}: {body}"));
        }
        if body
            .get("success")
            .and_then(|value| value.as_bool())
            .is_some_and(|success| !success)
        {
            return Err(format!("Shipday order was rejected: {body}"));
        }

        let order_id = Self::string_or_number(&body, &["orderId", "id"])
            .ok_or_else(|| "Shipday order response missing orderId".to_string())?;

        Ok(ShipdayDelivery {
            order_id,
            order_number: request.order_number,
            tracking_id: Self::string_value(&body, &["trackingId", "trackingNumber"]),
            tracking_url: Self::string_value(&body, &["trackingUrl", "trackingLink"]),
            status: Self::string_value(&body, &["status", "orderStatus", "deliveryStatus"]),
        })
    }

    pub async fn delivery_status(
        &self,
        tracking_id: &str,
    ) -> Result<ShipdayDeliveryStatus, String> {
        let token = self.validate_credentials()?;
        Self::require_non_empty(tracking_id, "Shipday tracking id")?;

        let resp = self
            .http_client
            .get(format!(
                "{}/order/progress/{}?isStaticDataRequired=false",
                self.api_base,
                Self::encode_path_segment(tracking_id.trim())
            ))
            .header("Accept", "application/json")
            .header("Authorization", format!("Basic {token}"))
            .header("x-api-key", token)
            .send()
            .await
            .map_err(|e| format!("Shipday progress request failed: {e}"))?;

        let http_status = resp.status();
        let body: serde_json::Value = resp
            .json()
            .await
            .map_err(|e| format!("Shipday progress response was not JSON: {e}"))?;
        if !http_status.is_success() {
            return Err(format!("Shipday progress API error {http_status}: {body}"));
        }

        let status = Self::string_value(
            &body,
            &[
                "orderStatus",
                "status",
                "deliveryStatus",
                "currentStatus",
                "event",
            ],
        )
        .or_else(|| {
            body.get("dynamicData").and_then(|data| {
                Self::string_value(data, &["orderStatus", "status", "deliveryStatus"])
            })
        })
        .ok_or_else(|| "Shipday progress response missing delivery status".to_string())?;

        Ok(ShipdayDeliveryStatus {
            tracking_id: Self::string_value(&body, &["trackingId"])
                .unwrap_or_else(|| tracking_id.trim().to_string()),
            status,
            eta: Self::string_value(&body, &["eta", "estimatedDeliveryTime"]).or_else(|| {
                body.get("dynamicData")
                    .and_then(|data| Self::string_value(data, &["eta", "estimatedDeliveryTime"]))
            }),
            driver_name: Self::string_value(&body, &["carrierName", "driverName"]).or_else(|| {
                body.get("carrier")
                    .and_then(|carrier| Self::string_value(carrier, &["name"]))
            }),
            driver_latitude: Self::number_value(&body, &["carrierLatitude", "driverLatitude"])
                .or_else(|| {
                    body.get("carrier")
                        .and_then(|carrier| Self::number_value(carrier, &["latitude", "lat"]))
                }),
            driver_longitude: Self::number_value(&body, &["carrierLongitude", "driverLongitude"])
                .or_else(|| {
                    body.get("carrier")
                        .and_then(|carrier| Self::number_value(carrier, &["longitude", "lng"]))
                }),
            tracking_url: Self::string_value(&body, &["trackingUrl", "trackingLink"]),
        })
    }

    fn string_or_number(body: &serde_json::Value, keys: &[&str]) -> Option<String> {
        keys.iter().find_map(|key| {
            body.get(*key).and_then(|value| {
                value
                    .as_str()
                    .map(|value| value.to_string())
                    .or_else(|| value.as_i64().map(|value| value.to_string()))
                    .or_else(|| value.as_u64().map(|value| value.to_string()))
            })
        })
    }

    fn string_value(body: &serde_json::Value, keys: &[&str]) -> Option<String> {
        keys.iter().find_map(|key| {
            body.get(*key)
                .and_then(|value| value.as_str())
                .filter(|value| !value.trim().is_empty())
                .map(|value| value.to_string())
        })
    }

    fn number_value(body: &serde_json::Value, keys: &[&str]) -> Option<f64> {
        keys.iter()
            .find_map(|key| body.get(*key).and_then(|value| value.as_f64()))
    }

    fn encode_path_segment(value: &str) -> String {
        let mut encoded = String::new();
        for byte in value.as_bytes() {
            match byte {
                b'A'..=b'Z'
                | b'a'..=b'z'
                | b'0'..=b'9'
                | b'-'
                | b'_'
                | b'.'
                | b'~' => encoded.push(*byte as char),
                other => encoded.push_str(&format!("%{other:02X}")),
            }
        }
        encoded
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::{Read, Write};
    use std::net::TcpListener;
    use std::sync::mpsc;
    use std::thread;

    fn start_one_request_server(response: &'static str) -> (String, mpsc::Receiver<String>) {
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let addr = listener.local_addr().unwrap();
        let (tx, rx) = mpsc::channel();

        thread::spawn(move || {
            let (mut stream, _) = listener.accept().unwrap();
            let mut buffer = [0; 8192];
            let bytes = stream.read(&mut buffer).unwrap();
            tx.send(String::from_utf8_lossy(&buffer[..bytes]).to_string())
                .unwrap();
            stream.write_all(response.as_bytes()).unwrap();
        });

        (format!("http://{addr}"), rx)
    }

    #[tokio::test]
    async fn create_delivery_posts_shipday_order_and_parses_provider_id() {
        let response = concat!(
            "HTTP/1.1 200 OK\r\n",
            "Content-Type: application/json\r\n",
            "Content-Length: 72\r\n",
            "\r\n",
            r#"{"success":true,"orderId":98765,"trackingId":"track_123","status":"new"}"#
        );
        let (base_url, rx) = start_one_request_server(response);
        let client =
            ShipdayClient::with_base_url_for_test("live_shipday_key".to_string(), base_url);

        let created = client
            .create_delivery(ShipdayCreateDeliveryRequest {
                order_number: "ohc-1001".to_string(),
                customer_name: "Maya Chen".to_string(),
                customer_address: "12 Market St, San Francisco, CA".to_string(),
                customer_phone_number: "+14155550100".to_string(),
                customer_email: Some("maya@example.com".to_string()),
                pickup_name: "Maya Cakes".to_string(),
                pickup_address: "1 Bakery Ln, San Francisco, CA".to_string(),
                delivery_instruction: Some("Leave at counter".to_string()),
                order_item: Some("custom cake".to_string()),
            })
            .await
            .unwrap();

        assert_eq!(created.order_id, "98765");
        assert_eq!(created.tracking_id.as_deref(), Some("track_123"));
        assert_eq!(created.status.as_deref(), Some("new"));

        let request = rx.recv().unwrap();
        assert!(request.starts_with("POST /orders HTTP/1.1"));
        assert!(request.contains("authorization: Basic live_shipday_key"));
        assert!(request.contains(r#""orderNumber":"ohc-1001""#));
        assert!(request.contains(r#""restaurantName":"Maya Cakes""#));
    }

    #[tokio::test]
    async fn tracking_fetches_shipday_progress_and_parses_live_status() {
        let response = concat!(
            "HTTP/1.1 200 OK\r\n",
            "Content-Type: application/json\r\n",
            "Content-Length: 102\r\n",
            "\r\n",
            r#"{"trackingId":"track_123","orderStatus":"in_transit","eta":"2026-06-06T19:30:00Z","carrierName":"Sam"}"#
        );
        let (base_url, rx) = start_one_request_server(response);
        let client =
            ShipdayClient::with_base_url_for_test("live_shipday_key".to_string(), base_url);

        let status = client.delivery_status("track_123").await.unwrap();

        assert_eq!(status.tracking_id, "track_123");
        assert_eq!(status.status, "in_transit");
        assert_eq!(status.eta.as_deref(), Some("2026-06-06T19:30:00Z"));
        assert_eq!(status.driver_name.as_deref(), Some("Sam"));

        let request = rx.recv().unwrap();
        assert!(request.starts_with(
            "GET /order/progress/track_123?isStaticDataRequired=false HTTP/1.1"
        ));
        assert!(request.contains("authorization: Basic live_shipday_key"));
    }

    #[tokio::test]
    async fn create_delivery_rejects_placeholder_credentials_before_network() {
        let client = ShipdayClient::with_base_url_for_test(
            "dummy_shipday_key".to_string(),
            "http://127.0.0.1:9".to_string(),
        );

        let err = client
            .create_delivery(ShipdayCreateDeliveryRequest {
                order_number: "ohc-1001".to_string(),
                customer_name: "Maya Chen".to_string(),
                customer_address: "12 Market St, San Francisco, CA".to_string(),
                customer_phone_number: "+14155550100".to_string(),
                customer_email: None,
                pickup_name: "Maya Cakes".to_string(),
                pickup_address: "1 Bakery Ln, San Francisco, CA".to_string(),
                delivery_instruction: None,
                order_item: None,
            })
            .await
            .unwrap_err();

        assert!(err.contains("Shipday API key is required"));
    }
}
