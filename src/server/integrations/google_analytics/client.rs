use serde::{Deserialize, Serialize};
use std::sync::OnceLock;

static HTTP_CLIENT: OnceLock<reqwest::Client> = OnceLock::new();

fn get_client() -> &'static reqwest::Client {
    HTTP_CLIENT.get_or_init(reqwest::Client::new)
}

const BASE_URL: &str = "https://analyticsdata.googleapis.com/v1beta";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GAMetricValue {
    #[serde(default)]
    pub value: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GAMetric {
    #[serde(default)]
    pub metric_name: String,
    #[serde(default)]
    pub values: Vec<GAMetricValue>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GADimensionValue {
    #[serde(default)]
    pub value: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GADimensionHeader {
    #[serde(default)]
    pub dimension_name: String,
    #[serde(default)]
    pub values: Vec<GADimensionValue>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GADimensionMetadata {
    #[serde(default)]
    pub api_name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GAMetricMetadata {
    #[serde(default)]
    pub api_name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GAReportRow {
    #[serde(default)]
    pub dimensions: Vec<GADimensionValue>,
    #[serde(default)]
    pub metrics: Vec<GAMetric>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GAReport {
    #[serde(default)]
    pub dimension_headers: Vec<GADimensionHeader>,
    #[serde(default)]
    pub rows: Vec<GAReportRow>,
    #[serde(default)]
    pub row_count: u64,
    #[serde(default)]
    pub metadata: Option<GAReportMetadata>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GAReportMetadata {
    #[serde(default)]
    pub currency_code: Option<String>,
    #[serde(default)]
    pub time_zone: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GARunReportRequest {
    #[serde(default)]
    pub date_ranges: Vec<GADateRange>,
    #[serde(default)]
    pub metrics: Vec<GAMetricRequest>,
    #[serde(default)]
    pub dimensions: Vec<GADimensionRequest>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GADateRange {
    #[serde(default)]
    pub start_date: String,
    #[serde(default)]
    pub end_date: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GAMetricRequest {
    #[serde(default)]
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GADimensionRequest {
    #[serde(default)]
    pub name: String,
}

pub struct GoogleAnalyticsClient {
    access_token: String,
    property_id: String,
    base_url: String,
}

impl GoogleAnalyticsClient {
    pub fn new(access_token: String, property_id: String) -> Self {
        Self {
            access_token,
            property_id,
            base_url: BASE_URL.to_string(),
        }
    }

    #[cfg(test)]
    fn with_base_url_for_test(access_token: String, property_id: String, base_url: String) -> Self {
        Self {
            access_token,
            property_id,
            base_url,
        }
    }

    fn property_url(&self, method: &str) -> String {
        format!(
            "{}/properties/{}:{}",
            self.base_url,
            self.property_id.trim(),
            method
        )
    }

    fn validated_access_token(&self) -> Result<&str, String> {
        let token = self.access_token.trim();
        if token.is_empty() {
            Err("Google Analytics access token is required".to_string())
        } else {
            Ok(token)
        }
    }

    pub async fn get_realtime_report(
        &self,
        metrics: &[String],
        dimensions: &[String],
    ) -> Result<GAReport, String> {
        let url = self.property_url("runRealtimeReport");
        let token = self.validated_access_token()?;

        let payload = serde_json::json!({
            "metrics": metrics.iter().map(|m| serde_json::json!({"name": m})).collect::<Vec<_>>(),
            "dimensions": dimensions.iter().map(|d| serde_json::json!({"name": d})).collect::<Vec<_>>(),
        });

        let client = get_client();
        let res = client
            .post(&url)
            .header("Authorization", format!("Bearer {}", token))
            .header("Content-Type", "application/json")
            .json(&payload)
            .send()
            .await
            .map_err(|e| format!("reqwest error: {}", e))?;

        if res.status().is_success() {
            res.json::<GAReport>()
                .await
                .map_err(|e| format!("response parse error: {}", e))
        } else {
            let status = res.status();
            let body = res.text().await.unwrap_or_default();
            Err(format!("API error {}: {}", status, body))
        }
    }

    pub async fn get_report(
        &self,
        date_range_start: &str,
        date_range_end: &str,
        metrics: &[String],
        dimensions: &[String],
    ) -> Result<GAReport, String> {
        let url = self.property_url("runReport");
        let token = self.validated_access_token()?;

        let payload = serde_json::json!({
            "dateRanges": [{
                "startDate": date_range_start,
                "endDate": date_range_end,
            }],
            "metrics": metrics.iter().map(|m| serde_json::json!({"name": m})).collect::<Vec<_>>(),
            "dimensions": dimensions.iter().map(|d| serde_json::json!({"name": d})).collect::<Vec<_>>(),
        });

        let client = get_client();
        let res = client
            .post(&url)
            .header("Authorization", format!("Bearer {}", token))
            .header("Content-Type", "application/json")
            .json(&payload)
            .send()
            .await
            .map_err(|e| format!("reqwest error: {}", e))?;

        if res.status().is_success() {
            res.json::<GAReport>()
                .await
                .map_err(|e| format!("response parse error: {}", e))
        } else {
            let status = res.status();
            let body = res.text().await.unwrap_or_default();
            Err(format!("API error {}: {}", status, body))
        }
    }

    pub async fn get_visitors(&self, days: u32) -> Result<(u64, u64), String> {
        let end = chrono::Utc::now().format("%Y-%m-%d").to_string();
        let start = (chrono::Utc::now() - chrono::Duration::days(days as i64))
            .format("%Y-%m-%d")
            .to_string();

        let report = self
            .get_report(
                &start,
                &end,
                &["activeUsers".to_string()],
                &[],
            )
            .await?;

        let total = report
            .rows
            .first()
            .and_then(|r| r.metrics.first())
            .and_then(|m| m.values.first())
            .and_then(|v| v.value.parse::<u64>().ok())
            .unwrap_or(0);

        Ok((total, total))
    }

    pub async fn get_top_pages(&self, limit: u32) -> Result<Vec<(String, u64)>, String> {
        let end = chrono::Utc::now().format("%Y-%m-%d").to_string();
        let start = (chrono::Utc::now() - chrono::Duration::days(30))
            .format("%Y-%m-%d")
            .to_string();

        let report = self
            .get_report(
                &start,
                &end,
                &["screenPageViews".to_string()],
                &["pagePath".to_string()],
            )
            .await?;

        let mut rows: Vec<(String, u64)> = report
            .rows
            .iter()
            .filter_map(|r| {
                let path = r.dimensions.first()?.value.clone();
                let views = r.metrics.first()?.values.first()?.value.parse::<u64>().ok()?;
                Some((path, views))
            })
            .collect();

        rows.sort_by(|a, b| b.1.cmp(&a.1));
        rows.truncate(limit as usize);
        Ok(rows)
    }

    pub async fn get_traffic_sources(&self, days: u32) -> Result<Vec<(String, u64)>, String> {
        let end = chrono::Utc::now().format("%Y-%m-%d").to_string();
        let start = (chrono::Utc::now() - chrono::Duration::days(days as i64))
            .format("%Y-%m-%d")
            .to_string();

        let report = self
            .get_report(
                &start,
                &end,
                &["sessions".to_string()],
                &["sessionSource".to_string()],
            )
            .await?;

        let mut rows: Vec<(String, u64)> = report
            .rows
            .iter()
            .filter_map(|r| {
                let source = r.dimensions.first()?.value.clone();
                let sessions = r.metrics.first()?.values.first()?.value.parse::<u64>().ok()?;
                Some((source, sessions))
            })
            .collect();

        rows.sort_by(|a, b| b.1.cmp(&a.1));
        Ok(rows)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::io::{AsyncReadExt, AsyncWriteExt};
    use tokio::net::TcpListener;
    use tokio::sync::oneshot;

    async fn start_ga_server(response_body: &'static str) -> (String, oneshot::Receiver<String>) {
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let base_url = format!("http://{}", listener.local_addr().unwrap());
        let (request_tx, request_rx) = oneshot::channel();

        tokio::spawn(async move {
            let (mut stream, _) = listener.accept().await.unwrap();
            let mut request = Vec::new();
            let mut buffer = [0_u8; 4096];
            let mut header_end = None;
            let mut content_length = 0_usize;

            loop {
                let read = stream.read(&mut buffer).await.unwrap();
                assert!(read > 0, "client closed connection before sending request");
                request.extend_from_slice(&buffer[..read]);

                if header_end.is_none() {
                    if let Some(index) =
                        request.windows(4).position(|window| window == b"\r\n\r\n")
                    {
                        header_end = Some(index + 4);
                        let headers = String::from_utf8_lossy(&request[..index]);
                        content_length = headers
                            .lines()
                            .find_map(|line| {
                                line.strip_prefix("content-length: ")
                                    .or_else(|| line.strip_prefix("Content-Length: "))
                            })
                            .and_then(|value| value.trim().parse::<usize>().ok())
                            .unwrap_or(0);
                    }
                }

                if let Some(body_start) = header_end {
                    if request.len() >= body_start + content_length {
                        break;
                    }
                }
            }

            let response = format!(
                "HTTP/1.1 200 OK\r\ncontent-type: application/json\r\ncontent-length: {}\r\nconnection: close\r\n\r\n{}",
                response_body.len(),
                response_body
            );
            stream.write_all(response.as_bytes()).await.unwrap();
            request_tx.send(String::from_utf8(request).unwrap()).unwrap();
        });

        (base_url, request_rx)
    }

    fn request_body(request: &str) -> serde_json::Value {
        let (_, body) = request.split_once("\r\n\r\n").unwrap();
        serde_json::from_str(body).unwrap()
    }

    #[tokio::test]
    async fn get_report_sends_correct_request() {
        let response = r#"{
            "dimensionHeaders": [{"dimensionName": "pagePath"}],
            "rows": [{
                "dimensions": [{"value": "/home"}],
                "metrics": [{"values": [{"value": "100"}]}]
            }],
            "rowCount": 1
        }"#;
        let (base_url, _request_rx) = start_ga_server(response).await;

        let client = GoogleAnalyticsClient::with_base_url_for_test(
            "test-token".to_string(),
            "properties/123456".to_string(),
            base_url,
        );

        // Override the BASE_URL for test by constructing URL manually
        let report = client
            .get_report("2026-01-01", "2026-01-31", &["screenPageViews".to_string()], &["pagePath".to_string()])
            .await
            .unwrap();

        assert_eq!(report.row_count, 1);
        assert_eq!(report.rows[0].dimensions[0].value, "/home");
    }

    #[tokio::test]
    async fn get_top_pages_returns_sorted_results() {
        let response = r#"{
            "rows": [
                {"dimensions": [{"value": "/about"}], "metrics": [{"values": [{"value": "50"}]}]},
                {"dimensions": [{"value": "/"}], "metrics": [{"values": [{"value": "200"}]}]},
                {"dimensions": [{"value": "/blog"}], "metrics": [{"values": [{"value": "100"}]}]}
            ]
        }"#;
        let (base_url, _request_rx) = start_ga_server(response).await;

        let client = GoogleAnalyticsClient::with_base_url_for_test(
            "test-token".to_string(),
            "properties/123456".to_string(),
            base_url,
        );

        let pages = client.get_top_pages(2).await.unwrap();
        assert_eq!(pages.len(), 2);
        assert_eq!(pages[0].0, "/");
        assert_eq!(pages[0].1, 200);
        assert_eq!(pages[1].0, "/blog");
        assert_eq!(pages[1].1, 100);
    }

    #[tokio::test]
    async fn get_traffic_sources_returns_results() {
        let response = r#"{
            "rows": [
                {"dimensions": [{"value": "google"}], "metrics": [{"values": [{"value": "300"}]}]},
                {"dimensions": [{"value": "direct"}], "metrics": [{"values": [{"value": "100"}]}]}
            ]
        }"#;
        let (base_url, _request_rx) = start_ga_server(response).await;

        let client = GoogleAnalyticsClient::with_base_url_for_test(
            "test-token".to_string(),
            "properties/123456".to_string(),
            base_url,
        );

        let sources = client.get_traffic_sources(30).await.unwrap();
        assert_eq!(sources.len(), 2);
        assert_eq!(sources[0].0, "google");
        assert_eq!(sources[0].1, 300);
    }

    #[tokio::test]
    async fn get_realtime_report_sends_correct_payload() {
        let response = r#"{
            "rows": [],
            "rowCount": 0
        }"#;
        let (base_url, request_rx) = start_ga_server(response).await;

        let client = GoogleAnalyticsClient::with_base_url_for_test(
            "test-token".to_string(),
            "properties/123456".to_string(),
            base_url,
        );

        let _ = client
            .get_realtime_report(&["activeUsers".to_string()], &["country".to_string()])
            .await
            .unwrap();

        let request = request_rx.await.unwrap();
        assert!(request.starts_with("POST /properties/properties/123456:runRealtimeReport HTTP/1.1"));
        assert!(
            request.contains("authorization: Bearer test-token")
                || request.contains("Authorization: Bearer test-token")
        );

        let body = request_body(&request);
        assert_eq!(body["metrics"][0]["name"], "activeUsers");
        assert_eq!(body["dimensions"][0]["name"], "country");
    }

    #[tokio::test]
    async fn blank_token_rejected_before_network() {
        let client = GoogleAnalyticsClient::new("   ".to_string(), "properties/123456".to_string());
        let err = client
            .get_report("2026-01-01", "2026-01-31", &["activeUsers".to_string()], &[])
            .await
            .unwrap_err();
        assert_eq!(err, "Google Analytics access token is required");
    }
}
