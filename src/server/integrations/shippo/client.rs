use serde::{Deserialize, Serialize};
use serde_json::json;
use std::{collections::HashMap, time::Duration};

const MAX_PARCEL_VALUE: f64 = 100_000.0;
const MAX_TRANSACTION_RESPONSE_BYTES: usize = 1024 * 1024;
const MAX_RECONCILIATION_PAGES: usize = 10;
const TRANSACTIONS_PER_PAGE: usize = 100;
const REQUEST_TIMEOUT: Duration = Duration::from_secs(15);
const RECONCILIATION_TIMEOUT: Duration = Duration::from_secs(30);

fn safe_identifier(value: &str) -> bool {
    !value.is_empty()
        && value.len() <= 128
        && value
            .bytes()
            .all(|b| b.is_ascii_alphanumeric() || b == b'_' || b == b'-')
}

fn parcel_dimensions(dimensions: &str) -> Result<(String, String, String), String> {
    let values = dimensions
        .split(['x', 'X'])
        .map(str::trim)
        .map(str::parse::<f64>)
        .collect::<Result<Vec<_>, _>>()
        .map_err(|_| "parcel dimensions must contain three positive numbers".to_string())?;
    let [length, width, height] = values.as_slice() else {
        return Err("parcel dimensions must contain length, width, and height".to_string());
    };
    if !values
        .iter()
        .all(|value| value.is_finite() && *value > 0.0 && *value <= MAX_PARCEL_VALUE)
    {
        return Err("parcel dimensions must contain three positive numbers".to_string());
    }
    Ok((length.to_string(), width.to_string(), height.to_string()))
}

fn trusted_label_url(raw_url: &str) -> Option<String> {
    let url = reqwest::Url::parse(raw_url).ok()?;
    let host = url.host_str()?;
    let trusted_host = host == "goshippo.com"
        || host.ends_with(".goshippo.com")
        || host == "shippo-delivery.s3.amazonaws.com"
        || host == "shippo-delivery-east.s3.amazonaws.com"
        || host == "shippo-delivery-west.s3.amazonaws.com";
    (url.scheme() == "https"
        && trusted_host
        && url.username().is_empty()
        && url.password().is_none())
    .then(|| url.to_string())
}

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
    pub tracking_number: Option<String>,
    pub carrier: Option<String>,
    #[serde(rename = "transactionId")]
    pub transaction_id: String,
    pub test: bool,
}

/// Provider evidence for an already admitted durable purchase intent.
/// An absent label, including an entirely empty observation, never permits another purchase.
#[derive(Debug, Clone, Default)]
pub struct Observation {
    pub transaction_id: Option<String>,
    pub label: Option<PurchaseLabelResponse>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShippoTrackingStatus {
    pub tracking_number: String,
    pub carrier: String,
    pub status: String,
    pub status_details: String,
}

pub struct ShippoClient {
    pub api_key: String,
    http_client: reqwest::Client,
    api_base: String,
}

impl ShippoClient {
    pub fn new(api_key: String) -> Self {
        ShippoClient {
            api_key,
            http_client: reqwest::Client::builder()
                .redirect(reqwest::redirect::Policy::none())
                .retry(reqwest::retry::never())
                .timeout(REQUEST_TIMEOUT)
                .connect_timeout(Duration::from_secs(5))
                .build()
                .expect("Shippo HTTP client initialization failed"),
            api_base: Self::api_base(),
        }
    }

    fn api_base() -> String {
        std::env::var("SHIPPO_API_BASE")
            .unwrap_or_else(|_| "https://api.goshippo.com".to_string())
            .trim_end_matches('/')
            .to_string()
    }

    fn validate_credentials(&self) -> Result<(), String> {
        let token = self.api_key.trim();
        if token.is_empty()
            || token.contains("dummy")
            || token.contains("mock")
            || token.contains("fake")
        {
            return Err("Shippo API token is required".to_string());
        }
        Ok(())
    }

    fn configured_address(var_name: &str) -> Result<serde_json::Value, String> {
        let raw = std::env::var(var_name)
            .map_err(|_| format!("{var_name} is required to request live Shippo rates"))?;
        serde_json::from_str(&raw)
            .map_err(|e| format!("{var_name} must be valid Shippo address JSON: {e}"))
    }

    pub async fn fetch_rates(
        &self,
        weight: f64,
        dimensions: &str,
    ) -> Result<Vec<ShippoRate>, String> {
        self.validate_credentials()?;
        if weight <= 0.0 {
            return Err("shipment weight must be positive".to_string());
        }

        let (length, width, height) = parcel_dimensions(dimensions)?;

        let address_from = Self::configured_address("SHIPPO_ADDRESS_FROM_JSON")?;
        let address_to = Self::configured_address("SHIPPO_ADDRESS_TO_JSON")?;
        let parcel = json!({
            "length": length,
            "width": width,
            "height": height,
            "distance_unit": "in",
            "weight": weight.to_string(),
            "mass_unit": "oz",
        });
        let payload = json!({
            "address_from": address_from,
            "address_to": address_to,
            "parcels": [parcel],
            "async": false,
        });

        let resp = self
            .http_client
            .post(format!("{}/shipments", self.api_base))
            .header(
                "Authorization",
                format!("ShippoToken {}", self.api_key.trim()),
            )
            .header("Content-Type", "application/json")
            .header("SHIPPO-API-VERSION", "2018-02-08")
            .json(&payload)
            .send()
            .await
            .map_err(|e| format!("Shippo shipment request failed: {e}"))?;

        let status = resp.status();
        let body: serde_json::Value = resp
            .json()
            .await
            .map_err(|e| format!("Shippo rates response was not JSON: {e}"))?;
        if !status.is_success() {
            return Err(format!("Shippo rates API error {status}: {body}"));
        }

        let rates = body
            .get("rates")
            .and_then(|v| v.as_array())
            .ok_or_else(|| "Shippo rates response missing rates".to_string())?;

        Ok(rates
            .iter()
            .filter_map(|rate| {
                let id = rate
                    .get("object_id")
                    .or_else(|| rate.get("id"))
                    .and_then(|v| v.as_str())?;
                let carrier = rate
                    .get("provider")
                    .and_then(|v| v.as_str())
                    .unwrap_or("Shippo");
                let service = rate
                    .get("servicelevel")
                    .and_then(|v| v.get("name"))
                    .and_then(|v| v.as_str())
                    .or_else(|| rate.get("service").and_then(|v| v.as_str()))
                    .unwrap_or("Service");
                let amount = rate
                    .get("amount")
                    .and_then(|v| v.as_str())
                    .unwrap_or_default();
                let days = rate
                    .get("estimated_days")
                    .and_then(|v| v.as_u64())
                    .unwrap_or_default() as u32;
                Some(ShippoRate {
                    id: id.to_string(),
                    carrier: carrier.to_string(),
                    service: service.to_string(),
                    amount: amount.to_string(),
                    days,
                })
            })
            .collect())
    }

    fn transaction_request(&self, method: reqwest::Method, url: String) -> reqwest::RequestBuilder {
        self.http_client
            .request(method, url)
            .header(
                "Authorization",
                format!("ShippoToken {}", self.api_key.trim()),
            )
            .header("Content-Type", "application/json")
            .header("SHIPPO-API-VERSION", "2018-02-08")
    }

    async fn transaction_response(
        request: reqwest::RequestBuilder,
    ) -> Result<serde_json::Value, String> {
        let mut response = request.send().await.map_err(|_| {
            "Shippo transaction request failed; reconcile before retrying".to_string()
        })?;
        // Never expose raw provider response bodies, request URLs, or transport errors.
        if !response.status().is_success() {
            return Err("Shippo transaction was not confirmed; reconcile before retrying".into());
        }
        if response
            .content_length()
            .is_some_and(|size| size > MAX_TRANSACTION_RESPONSE_BYTES as u64)
        {
            return Err("Shippo transaction response exceeded the size limit".into());
        }
        let mut bytes = Vec::new();
        while let Some(chunk) = response
            .chunk()
            .await
            .map_err(|_| "Shippo transaction response was incomplete".to_string())?
        {
            if chunk.len() > MAX_TRANSACTION_RESPONSE_BYTES.saturating_sub(bytes.len()) {
                return Err("Shippo transaction response exceeded the size limit".into());
            }
            bytes.extend_from_slice(&chunk);
        }
        serde_json::from_slice(&bytes)
            .map_err(|_| "Shippo transaction response was not valid JSON".to_string())
    }

    fn validate_intent_inputs(&self, rate_id: &str, metadata: &str) -> Result<(), String> {
        self.validate_credentials()?;
        if !safe_identifier(rate_id) {
            return Err("Shippo rate identity is invalid".into());
        }
        let valid_metadata = metadata.len() <= 100
            && metadata.strip_prefix("ohc_shipping_").is_some_and(|id| {
                uuid::Uuid::parse_str(id).is_ok_and(|parsed| parsed.hyphenated().to_string() == id)
            });
        if !valid_metadata {
            return Err("Shippo purchase correlation is invalid".into());
        }
        Ok(())
    }

    fn transaction_identity(body: &serde_json::Value) -> Result<(&str, &str, &str, bool), String> {
        let id = body
            .get("object_id")
            .and_then(serde_json::Value::as_str)
            .filter(|value| safe_identifier(value))
            .ok_or("Shippo transaction identity is invalid")?;
        let metadata = body
            .get("metadata")
            .and_then(serde_json::Value::as_str)
            .filter(|value| value.len() <= 100 && !value.chars().any(char::is_control))
            .ok_or("Shippo transaction correlation is invalid")?;
        let rate = body
            .get("rate")
            .and_then(|value| {
                value
                    .as_str()
                    .or_else(|| value.get("object_id").and_then(serde_json::Value::as_str))
            })
            .filter(|value| safe_identifier(value))
            .ok_or("Shippo transaction rate is invalid")?;
        let is_test = body
            .get("test")
            .and_then(serde_json::Value::as_bool)
            .ok_or("Shippo transaction mode is invalid")?;
        Ok((id, metadata, rate, is_test))
    }

    fn observe_transaction(
        body: &serde_json::Value,
        rate_id: &str,
        metadata: &str,
        is_test: bool,
        transaction_id: Option<&str>,
    ) -> Result<Observation, String> {
        let (id, observed_metadata, rate, observed_test) = Self::transaction_identity(body)?;
        if observed_metadata != metadata
            || rate != rate_id
            || observed_test != is_test
            || transaction_id.is_some_and(|expected| expected != id)
        {
            return Err("Shippo transaction does not match the purchase intent".into());
        }
        // WAITING, ERROR, unknown statuses, and malformed labels retain only matched identity.
        Ok(Observation {
            transaction_id: Some(id.to_owned()),
            label: Self::parse_label_response(body).ok(),
        })
    }

    /// Submit exactly once, only after the caller durably admits this intent.
    /// Shippo metadata is correlation, not a provider-supported idempotency key.
    pub async fn purchase_for_intent(
        &self,
        rate_id: &str,
        metadata: &str,
        is_test: bool,
    ) -> Result<Observation, String> {
        self.validate_intent_inputs(rate_id, metadata)?;
        let request = self.transaction_request(reqwest::Method::POST, format!("{}/transactions", self.api_base))
            .json(&json!({"rate": rate_id, "metadata": metadata, "async": false, "label_file_type": "PDF"}));
        let body = Self::transaction_response(request).await?;
        Self::observe_transaction(&body, rate_id, metadata, is_test, None)
    }

    /// Read-only recovery. Absence or incomplete evidence never authorizes another POST.
    pub async fn reconcile_for_intent(
        &self,
        rate_id: &str,
        metadata: &str,
        is_test: bool,
        transaction_id: Option<&str>,
    ) -> Result<Observation, String> {
        self.validate_intent_inputs(rate_id, metadata)?;
        if transaction_id.is_some_and(|id| !safe_identifier(id)) {
            return Err("Shippo transaction identity is invalid".into());
        }
        tokio::time::timeout(RECONCILIATION_TIMEOUT, async {
            if let Some(id) = transaction_id {
                let request = self.transaction_request(
                    reqwest::Method::GET,
                    format!("{}/transactions/{id}", self.api_base),
                );
                let body = Self::transaction_response(request).await?;
                return Self::observe_transaction(&body, rate_id, metadata, is_test, Some(id));
            }

            let mut seen = HashMap::<String, serde_json::Value>::new();
            let mut observation = Observation::default();
            for page in 1..=MAX_RECONCILIATION_PAGES {
                // Treat next only as a continuation signal. Never follow a provider-supplied URL.
                let request = self
                    .transaction_request(
                        reqwest::Method::GET,
                        format!("{}/transactions", self.api_base),
                    )
                    .query(&[
                        ("rate", rate_id.to_owned()),
                        ("page", page.to_string()),
                        ("results", TRANSACTIONS_PER_PAGE.to_string()),
                    ]);
                let body = Self::transaction_response(request).await?;
                let results = body
                    .get("results")
                    .and_then(serde_json::Value::as_array)
                    .filter(|items| items.len() <= TRANSACTIONS_PER_PAGE)
                    .ok_or("Shippo transaction list was malformed")?;
                let has_next = match body.get("next") {
                    Some(serde_json::Value::Null) => false,
                    Some(serde_json::Value::String(next))
                        if !next.is_empty()
                            && reqwest::Url::parse(next)
                                .is_ok_and(|url| matches!(url.scheme(), "http" | "https")) =>
                    {
                        true
                    }
                    _ => return Err("Shippo transaction pagination was malformed".to_string()),
                };
                if has_next && results.is_empty() {
                    return Err("Shippo transaction list was incomplete".into());
                }
                for transaction in results {
                    let (id, observed_metadata, rate, observed_test) =
                        Self::transaction_identity(transaction)?;
                    if let Some(previous) = seen.get(id) {
                        if previous != transaction {
                            return Err("Shippo transaction evidence was contradictory".into());
                        }
                        continue;
                    }
                    seen.insert(id.to_owned(), transaction.clone());
                    if observed_metadata != metadata || rate != rate_id || observed_test != is_test
                    {
                        continue;
                    }
                    if observation.transaction_id.is_some() {
                        return Err("Shippo purchase has multiple matching transactions".into());
                    }
                    observation =
                        Self::observe_transaction(transaction, rate_id, metadata, is_test, None)?;
                }
                if !has_next {
                    // This is a bounded observation, not proof of provider-side absence or a snapshot.
                    return Ok(observation);
                }
            }
            Err("Shippo transaction reconciliation exceeded the page limit".into())
        })
        .await
        .map_err(|_| "Shippo transaction reconciliation timed out".to_string())?
    }

    // Standalone transport API retained for existing unmounted integration callers.
    pub async fn purchase_label(&self, rate_id: &str) -> Result<PurchaseLabelResponse, String> {
        self.validate_credentials()?;
        if rate_id.trim().is_empty() {
            return Err("Shippo rate id is required".to_string());
        }
        let request = self
            .transaction_request(
                reqwest::Method::POST,
                format!("{}/transactions", self.api_base),
            )
            .json(&json!({"rate": rate_id, "async": false, "label_file_type": "PDF"}));
        let body = Self::transaction_response(request).await?;
        Self::parse_label_response(&body)
    }

    pub fn parse_label_response(body: &serde_json::Value) -> Result<PurchaseLabelResponse, String> {
        if body.get("status").and_then(|v| v.as_str()) != Some("SUCCESS") {
            return Err("Shippo did not confirm label creation; reconcile before retrying".into());
        }
        // Some supported API versions omit object_state. A contradictory state
        // is never accepted as a successful receipt.
        if body
            .get("object_state")
            .is_some_and(|v| v.as_str() != Some("VALID"))
        {
            return Err("Shippo returned an invalid transaction state".into());
        }
        let transaction_id = body
            .get("object_id")
            .and_then(|v| v.as_str())
            .filter(|value| safe_identifier(value))
            .ok_or("Shippo label response missing transaction identity")?
            .to_owned();
        let test = body
            .get("test")
            .and_then(|v| v.as_bool())
            .ok_or("Shippo label response missing mode")?;
        let label_url = body
            .get("label_url")
            .and_then(|v| v.as_str())
            .and_then(trusted_label_url)
            .ok_or("Shippo label response returned an untrusted label URL")?;
        let optional_text = |key: &str| -> Result<Option<String>, String> {
            match body.get(key) {
                None | Some(serde_json::Value::Null) => Ok(None),
                Some(serde_json::Value::String(value)) if value.is_empty() => Ok(None),
                Some(serde_json::Value::String(value))
                    if value.len() <= 256 && !value.chars().any(char::is_control) =>
                {
                    Ok(Some(value.clone()))
                }
                _ => Err(format!("Shippo returned invalid {key}")),
            }
        };
        let tracking_number = optional_text("tracking_number")?;
        // Never infer a carrier token from the display name or substitute Shippo.
        let carrier = optional_text("tracking_carrier")?;
        Ok(PurchaseLabelResponse {
            success: true,
            label_url,
            tracking_number,
            carrier,
            transaction_id,
            test,
        })
    }

    pub async fn fetch_tracking(
        &self,
        carrier: &str,
        tracking_number: &str,
    ) -> Result<ShippoTrackingStatus, String> {
        self.validate_credentials()?;
        if carrier.trim().is_empty() {
            return Err("Carrier is required".to_string());
        }
        if tracking_number.trim().is_empty() {
            return Err("Tracking number is required".to_string());
        }

        let resp = self
            .http_client
            .get(format!(
                "{}/tracks/{}/{}",
                self.api_base, carrier, tracking_number
            ))
            .header(
                "Authorization",
                format!("ShippoToken {}", self.api_key.trim()),
            )
            .header("Content-Type", "application/json")
            .header("SHIPPO-API-VERSION", "2018-02-08")
            .send()
            .await
            .map_err(|e| format!("Shippo tracking request failed: {e}"))?;

        let status = resp.status();
        let body: serde_json::Value = resp
            .json()
            .await
            .map_err(|e| format!("Shippo tracking response was not JSON: {e}"))?;

        if !status.is_success() {
            return Err(format!("Shippo tracking API error {status}: {body}"));
        }

        let tracking_status = body
            .get("tracking_status")
            .and_then(|v| v.as_object())
            .ok_or_else(|| "Shippo tracking response missing tracking_status".to_string())?;

        let status_val = tracking_status
            .get("status")
            .and_then(|v| v.as_str())
            .unwrap_or("UNKNOWN");

        let status_details = tracking_status
            .get("status_details")
            .and_then(|v| v.as_str())
            .unwrap_or("");

        Ok(ShippoTrackingStatus {
            tracking_number: tracking_number.to_string(),
            carrier: carrier.to_string(),
            status: status_val.to_string(),
            status_details: status_details.to_string(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn fetch_rates_requires_real_shippo_credentials() {
        let client = ShippoClient::new("dummy_token".to_string());
        let err = client.fetch_rates(16.0, "10x8x4").await.unwrap_err();
        assert!(err.contains("Shippo API token is required"));
    }

    #[tokio::test]
    async fn purchase_label_requires_real_shippo_credentials() {
        let client = ShippoClient::new("".to_string());
        let err = client.purchase_label("rate_123").await.unwrap_err();
        assert!(err.contains("Shippo API token is required"));
    }

    #[test]
    fn parcel_dimensions_are_split_into_shippo_fields() {
        assert_eq!(
            parcel_dimensions("10x8x6"),
            Ok(("10".to_string(), "8".to_string(), "6".to_string()))
        );
        assert!(parcel_dimensions("10x8").is_err());
    }

    #[test]
    fn label_urls_are_limited_to_https_shippo_delivery_hosts_without_userinfo() {
        assert!(
            trusted_label_url(
                "https://shippo-delivery-east.s3.amazonaws.com/label.pdf?signature=needed"
            )
            .is_some()
        );
        assert!(trusted_label_url("https://app.goshippo.com/labels/label.pdf").is_some());
        assert!(trusted_label_url("https://attacker.example/label.pdf").is_none());
        assert!(
            trusted_label_url("https://shippo-delivery-attacker.s3.amazonaws.com/label.pdf")
                .is_none()
        );
        assert!(trusted_label_url("https://user:password@app.goshippo.com/label.pdf").is_none());
        assert!(trusted_label_url("http://app.goshippo.com/label.pdf").is_none());
    }
}

#[cfg(test)]
mod intent_tests {
    use super::*;
    use std::{collections::VecDeque, sync::Arc};
    use tokio::io::{AsyncReadExt, AsyncWriteExt};
    use tokio::sync::Mutex;

    const METADATA: &str = "ohc_shipping_01234567-89ab-4cde-8fab-0123456789ab";

    fn receipt() -> serde_json::Value {
        json!({
            "metadata": METADATA,
            "rate": "rate_a",
            "test": true,
            "object_id": "transaction_a",
            "status": "SUCCESS",
            "object_state": "VALID",
            "label_url": "https://app.goshippo.com/labels/receipt.pdf"
        })
    }

    fn page(results: Vec<serde_json::Value>, next: serde_json::Value) -> String {
        response(
            "200 OK",
            &json!({"results": results, "next": next, "previous": null}).to_string(),
        )
    }

    fn response(status: &str, body: &str) -> String {
        format!(
            "HTTP/1.1 {status}\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{body}",
            body.len()
        )
    }

    struct Probe {
        base: String,
        requests: Arc<Mutex<Vec<String>>>,
        task: tokio::task::JoinHandle<()>,
    }

    impl Probe {
        async fn start(responses: Vec<String>) -> Self {
            let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
            let base = format!("http://{}", listener.local_addr().unwrap());
            let requests = Arc::new(Mutex::new(Vec::new()));
            let captured = requests.clone();
            let task = tokio::spawn(async move {
                let mut responses: VecDeque<String> = responses.into();
                loop {
                    let (mut stream, _) = listener.accept().await.unwrap();
                    let mut request = Vec::new();
                    loop {
                        let mut chunk = [0; 1024];
                        let count = stream.read(&mut chunk).await.unwrap();
                        if count == 0 {
                            break;
                        }
                        request.extend_from_slice(&chunk[..count]);
                        assert!(
                            request.len() < 64 * 1024,
                            "synthetic request exceeded fixture bound"
                        );
                        if let Some(end) = request.windows(4).position(|w| w == b"\r\n\r\n") {
                            let headers = String::from_utf8_lossy(&request[..end]);
                            let length = headers
                                .lines()
                                .find_map(|line| {
                                    let (name, value) = line.split_once(':')?;
                                    name.eq_ignore_ascii_case("content-length")
                                        .then(|| value.trim().parse::<usize>().unwrap())
                                })
                                .unwrap_or(0);
                            if request.len() >= end + 4 + length {
                                break;
                            }
                        }
                    }
                    captured
                        .lock()
                        .await
                        .push(String::from_utf8(request).unwrap());
                    let reply = responses.pop_front().unwrap_or_else(|| {
                        response("500 Internal Server Error", "unexpected additional request")
                    });
                    // A bounded-body client may close before receiving the entire response.
                    let _ = stream.write_all(reply.as_bytes()).await;
                }
            });
            Self {
                base,
                requests,
                task,
            }
        }

        fn client(&self) -> ShippoClient {
            let mut client = ShippoClient::new("shippo_test_synthetic".to_string());
            client.api_base = self.base.clone();
            client
        }
    }

    impl Drop for Probe {
        fn drop(&mut self) {
            self.task.abort();
        }
    }

    #[test]
    fn observation_accepts_exact_identity_with_string_or_expanded_rate() {
        for rate in [json!("rate_a"), json!({"object_id":"rate_a"})] {
            let mut body = receipt();
            body["rate"] = rate;
            let observed =
                ShippoClient::observe_transaction(&body, "rate_a", METADATA, true, None).unwrap();
            assert_eq!(observed.transaction_id.as_deref(), Some("transaction_a"));
            assert_eq!(observed.label.unwrap().transaction_id, "transaction_a");
        }
    }

    #[test]
    fn observation_rejects_missing_or_foreign_identity_and_unsafe_ids() {
        for (field, value) in [
            ("metadata", json!("another_intent")),
            ("metadata", serde_json::Value::Null),
            ("rate", json!("another_rate")),
            ("rate", json!({"object_id":"another_rate"})),
            ("rate", json!({"id":"rate_a"})),
            ("test", json!(false)),
            ("test", json!("true")),
            ("object_id", json!("../transaction_a")),
            ("object_id", json!("")),
            ("object_id", json!("x".repeat(129))),
        ] {
            let mut body = receipt();
            body[field] = value;
            assert!(
                ShippoClient::observe_transaction(&body, "rate_a", METADATA, true, None).is_err(),
                "accepted invalid {field}"
            );
        }
        assert!(
            ShippoClient::observe_transaction(
                &receipt(),
                "rate_a",
                METADATA,
                true,
                Some("different_transaction")
            )
            .is_err()
        );
    }

    #[test]
    fn observation_retains_matching_id_when_status_or_label_is_unconfirmed() {
        for (field, value) in [
            ("status", json!("WAITING")),
            ("status", json!("ERROR")),
            ("status", serde_json::Value::Null),
            ("object_state", json!("INVALID")),
            ("label_url", json!("https://attacker.example/label.pdf")),
            ("tracking_number", json!({"unexpected":"shape"})),
        ] {
            let mut body = receipt();
            body[field] = value;
            let observed =
                ShippoClient::observe_transaction(&body, "rate_a", METADATA, true, None).unwrap();
            assert_eq!(observed.transaction_id.as_deref(), Some("transaction_a"));
            assert!(observed.label.is_none(), "accepted unconfirmed {field}");
        }
    }

    #[tokio::test]
    async fn intent_purchase_sends_exact_correlation_and_accepts_strict_receipt() {
        let probe = Probe::start(vec![response("200 OK", &receipt().to_string())]).await;
        let observed = probe
            .client()
            .purchase_for_intent("rate_a", METADATA, true)
            .await
            .unwrap();
        assert!(observed.label.is_some());
        let requests = probe.requests.lock().await;
        assert_eq!(requests.len(), 1);
        assert!(requests[0].starts_with("POST /transactions HTTP/1.1\r\n"));
        let (_, body) = requests[0].split_once("\r\n\r\n").unwrap();
        let payload: serde_json::Value = serde_json::from_str(body).unwrap();
        assert_eq!(
            payload,
            json!({"rate":"rate_a","metadata":METADATA,"async":false,"label_file_type":"PDF"})
        );
    }

    #[tokio::test]
    async fn intent_purchase_does_not_follow_redirect_or_expose_provider_body() {
        let probe = Probe::start(vec!["HTTP/1.1 307 Temporary Redirect\r\nLocation: /redirected\r\nContent-Length: 0\r\nConnection: close\r\n\r\n".to_string()]).await;
        assert!(
            probe
                .client()
                .purchase_for_intent("rate_a", METADATA, true)
                .await
                .is_err()
        );
        assert_eq!(
            probe.requests.lock().await.len(),
            1,
            "redirect resubmitted a purchase"
        );
        let probe = Probe::start(vec![response(
            "502 Bad Gateway",
            "private provider details and shippo_test_synthetic",
        )])
        .await;
        let error = probe
            .client()
            .purchase_for_intent("rate_a", METADATA, true)
            .await
            .unwrap_err();
        assert!(!error.contains("private"));
        assert!(!error.contains("shippo_test_synthetic"));
        assert!(!error.contains(&probe.base));
    }

    #[tokio::test]
    async fn intent_purchase_rejects_invalid_inputs_without_sending_a_request() {
        let probe = Probe::start(vec![]).await;
        for (rate, metadata) in [
            ("../rate_a", METADATA),
            ("rate_a", ""),
            ("rate_a", "not_an_opaque_intent"),
        ] {
            assert!(
                probe
                    .client()
                    .purchase_for_intent(rate, metadata, true)
                    .await
                    .is_err()
            );
        }
        assert!(
            probe
                .client()
                .purchase_for_intent("rate_a", &"x".repeat(101), true)
                .await
                .is_err()
        );
        assert!(probe.requests.lock().await.is_empty());
    }

    #[tokio::test]
    async fn reconciliation_by_id_requires_exact_receipt_identity_and_never_falls_back() {
        let probe = Probe::start(vec![response("200 OK", &receipt().to_string())]).await;
        let observed = probe
            .client()
            .reconcile_for_intent("rate_a", METADATA, true, Some("transaction_a"))
            .await
            .unwrap();
        assert!(observed.label.is_some());
        let requests = probe.requests.lock().await;
        assert_eq!(requests.len(), 1);
        assert!(requests[0].starts_with("GET /transactions/transaction_a HTTP/1.1\r\n"));
        drop(requests);
        let probe = Probe::start(vec![response("200 OK", &receipt().to_string())]).await;
        assert!(
            probe
                .client()
                .reconcile_for_intent("rate_a", METADATA, true, Some("different_transaction"))
                .await
                .is_err()
        );
        assert_eq!(probe.requests.lock().await.len(), 1);
    }

    #[tokio::test]
    async fn reconciliation_enumerates_fixed_rate_filtered_pages_before_accepting_receipt() {
        let probe = Probe::start(vec![
            page(
                vec![receipt()],
                json!("https://attacker.example/arbitrary?page=2"),
            ),
            page(vec![receipt()], serde_json::Value::Null),
        ])
        .await;
        let observed = probe
            .client()
            .reconcile_for_intent("rate_a", METADATA, true, None)
            .await
            .unwrap();
        assert!(observed.label.is_some());
        let requests = probe.requests.lock().await;
        assert_eq!(requests.len(), 2);
        for (index, request) in requests.iter().enumerate() {
            assert!(request.starts_with(&format!(
                "GET /transactions?rate=rate_a&page={}&results=100 HTTP/1.1\r\n",
                index + 1
            )));
            assert!(!request.contains("attacker.example"));
        }
    }

    #[tokio::test]
    async fn reconciliation_rejects_multiple_exact_ids_and_contradictory_receipts() {
        for field in ["object_id", "status"] {
            let mut other = receipt();
            other[field] = json!("different");
            let probe =
                Probe::start(vec![page(vec![receipt(), other], serde_json::Value::Null)]).await;
            assert!(
                probe
                    .client()
                    .reconcile_for_intent("rate_a", METADATA, true, None)
                    .await
                    .is_err(),
                "accepted ambiguous {field}"
            );
            assert_eq!(probe.requests.lock().await.len(), 1);
        }
    }

    #[tokio::test]
    async fn reconciliation_rejects_duplicate_id_even_if_other_receipt_has_foreign_identity() {
        for field in ["metadata", "rate", "test"] {
            let mut other = receipt();
            other[field] = if field == "test" {
                json!(false)
            } else {
                json!("foreign")
            };
            for results in [
                vec![receipt(), other.clone()],
                vec![other.clone(), receipt()],
            ] {
                let probe = Probe::start(vec![page(results, serde_json::Value::Null)]).await;
                assert!(
                    probe
                        .client()
                        .reconcile_for_intent("rate_a", METADATA, true, None)
                        .await
                        .is_err(),
                    "accepted conflicting duplicate {field}"
                );
            }
        }
    }

    #[tokio::test]
    async fn intent_purchase_does_not_retry_after_connection_closes_without_response() {
        let probe = Probe::start(vec![String::new()]).await;
        let error = probe
            .client()
            .purchase_for_intent("rate_a", METADATA, true)
            .await
            .unwrap_err();
        assert!(!error.contains(&probe.base));
        assert!(!error.contains("shippo_test_synthetic"));
        assert_eq!(probe.requests.lock().await.len(), 1);
    }

    #[tokio::test]
    async fn reconciliation_empty_and_foreign_results_do_not_authorize_another_purchase() {
        let mut other = receipt();
        other["metadata"] = json!("other_intent");
        for results in [vec![], vec![other]] {
            let probe = Probe::start(vec![page(results, serde_json::Value::Null)]).await;
            let observed = probe
                .client()
                .reconcile_for_intent("rate_a", METADATA, true, None)
                .await
                .unwrap();
            assert!(observed.transaction_id.is_none());
            assert!(observed.label.is_none());
            let requests = probe.requests.lock().await;
            assert_eq!(requests.len(), 1);
            assert!(requests[0].starts_with("GET "));
        }
    }

    #[tokio::test]
    async fn reconciliation_does_not_accept_match_from_incomplete_or_malformed_list() {
        for body in [
            json!({"results":[receipt()]}),
            json!({"results":[receipt()],"next":42}),
            json!({"results":[receipt(),null],"next":null}),
            json!({"results":[receipt(),{"metadata":METADATA}],"next":null}),
            json!({"results":"not_an_array","next":null}),
        ] {
            let probe = Probe::start(vec![response("200 OK", &body.to_string())]).await;
            assert!(
                probe
                    .client()
                    .reconcile_for_intent("rate_a", METADATA, true, None)
                    .await
                    .is_err()
            );
        }
        let probe = Probe::start(vec![
            page(
                vec![receipt()],
                json!("https://api.goshippo.com/transactions?page=2"),
            ),
            response("503 Service Unavailable", "provider error"),
        ])
        .await;
        assert!(
            probe
                .client()
                .reconcile_for_intent("rate_a", METADATA, true, None)
                .await
                .is_err()
        );
    }

    #[tokio::test]
    async fn reconciliation_bounds_page_count_and_response_size() {
        let probe = Probe::start(
            (0..MAX_RECONCILIATION_PAGES + 1)
                .map(|_| {
                    page(
                        vec![receipt()],
                        json!("https://api.goshippo.com/transactions?page=2"),
                    )
                })
                .collect(),
        )
        .await;
        assert!(
            probe
                .client()
                .reconcile_for_intent("rate_a", METADATA, true, None)
                .await
                .is_err()
        );
        assert_eq!(probe.requests.lock().await.len(), MAX_RECONCILIATION_PAGES);
        let probe = Probe::start(vec![response(
            "200 OK",
            &" ".repeat(MAX_TRANSACTION_RESPONSE_BYTES + 1),
        )])
        .await;
        assert!(
            probe
                .client()
                .reconcile_for_intent("rate_a", METADATA, true, None)
                .await
                .is_err()
        );
    }
}
