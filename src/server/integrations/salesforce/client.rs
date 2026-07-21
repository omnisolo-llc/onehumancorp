use reqwest::Client;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SalesforceRecordAttributes {
    #[serde(rename = "type")]
    pub record_type: String,
    pub name: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SalesforceRecord {
    pub id: String,
    #[serde(default)]
    pub attributes: Option<SalesforceRecordAttributes>,
    #[serde(flatten)]
    pub fields: serde_json::Value,
}

#[derive(Debug, Deserialize)]
struct SalesforceQueryResponse {
    #[allow(dead_code)]
    total_size: u32,
    #[allow(dead_code)]
    done: bool,
    #[serde(default)]
    records: Vec<SalesforceRecord>,
}

#[derive(Debug, Deserialize)]
struct SalesforceErrorResponse {
    #[serde(default)]
    message: Option<String>,
    #[serde(default)]
    #[serde(rename = "errorCode")]
    error_code: Option<String>,
}

pub struct SalesforceClient {
    instance_url: String,
    access_token: String,
    http_client: Client,
}

impl SalesforceClient {
    pub fn new(instance_url: String, access_token: String) -> Self {
        Self {
            instance_url: instance_url.trim_end_matches('/').to_string(),
            access_token,
            http_client: Client::new(),
        }
    }

    #[cfg(test)]
    fn with_base_url_for_test(instance_url: String, access_token: String) -> Self {
        Self {
            instance_url: instance_url.trim_end_matches('/').to_string(),
            access_token,
            http_client: Client::new(),
        }
    }

    fn validated_access_token(&self) -> Result<&str, String> {
        let token = self.access_token.trim();
        if token.is_empty() {
            Err("Salesforce access token is required".to_string())
        } else {
            Ok(token)
        }
    }

    fn api_base(&self) -> String {
        format!("{}/services/data/v58.0", self.instance_url)
    }

    async fn check_error_response(&self, resp: reqwest::Response) -> Result<reqwest::Response, String> {
        if !resp.status().is_success() {
            let status = resp.status();
            let text = resp
                .text()
                .await
                .unwrap_or_else(|_| "<unreadable>".to_string());

            if let Ok(err) = serde_json::from_str::<SalesforceErrorResponse>(&text) {
                let msg = err.message.unwrap_or_default();
                let code = err.error_code.unwrap_or_default();
                return Err(format!("Salesforce API error {} [{}]: {}", status, code, msg));
            }

            return Err(format!("Salesforce API error {}: {}", status, text));
        }
        Ok(resp)
    }

    // ── Contacts ──────────────────────────────────────────────────

    pub async fn get_contacts(&self, query: &str, limit: u32) -> Result<Vec<SalesforceRecord>, String> {
        let token = self.validated_access_token()?;
        let soql = if query.is_empty() {
            format!(
                "SELECT Id, FirstName, LastName, Email, Phone, Account.Name FROM Contact LIMIT {}",
                limit
            )
        } else {
            format!("{} LIMIT {}", query.trim_end_matches(';'), limit)
        };

        let url = format!("{}/query", self.api_base());
        let resp = self
            .http_client
            .get(&url)
            .bearer_auth(token)
            .query(&[("q", soql.as_str())])
            .send()
            .await
            .map_err(|e| format!("Network error querying contacts: {}", e))?;

        let resp = self.check_error_response(resp).await?;
        let body: SalesforceQueryResponse = resp
            .json()
            .await
            .map_err(|e| format!("Failed to parse contact query response: {}", e))?;

        Ok(body.records)
    }

    pub async fn create_contact(
        &self,
        first_name: &str,
        last_name: &str,
        email: &str,
        phone: &str,
        company: &str,
    ) -> Result<SalesforceRecord, String> {
        let token = self.validated_access_token()?;
        let url = format!("{}/sobjects/Contact/", self.api_base());

        let payload = serde_json::json!({
            "FirstName": first_name,
            "LastName": last_name,
            "Email": email,
            "Phone": phone,
            "Company": company,
        });

        let resp = self
            .http_client
            .post(&url)
            .bearer_auth(token)
            .header("Content-Type", "application/json")
            .json(&payload)
            .send()
            .await
            .map_err(|e| format!("Network error creating contact: {}", e))?;

        let resp = self.check_error_response(resp).await?;
        let body: serde_json::Value = resp
            .json()
            .await
            .map_err(|e| format!("Failed to parse create contact response: {}", e))?;

        let id = body["id"]
            .as_str()
            .ok_or_else(|| "Response missing id field".to_string())?
            .to_string();

        Ok(SalesforceRecord {
            id,
            attributes: Some(SalesforceRecordAttributes {
                record_type: "Contact".to_string(),
                name: Some(format!("{} {}", first_name, last_name)),
            }),
            fields: serde_json::json!({
                "FirstName": first_name,
                "LastName": last_name,
                "Email": email,
                "Phone": phone,
                "Company": company,
            }),
        })
    }

    pub async fn update_contact(&self, id: &str, fields: &serde_json::Value) -> Result<(), String> {
        let token = self.validated_access_token()?;
        let url = format!("{}/sobjects/Contact/{}", self.api_base(), id);

        let resp = self
            .http_client
            .patch(&url)
            .bearer_auth(token)
            .header("Content-Type", "application/json")
            .json(fields)
            .send()
            .await
            .map_err(|e| format!("Network error updating contact: {}", e))?;

        self.check_error_response(resp).await?;
        Ok(())
    }

    // ── Opportunities ─────────────────────────────────────────────

    pub async fn get_opportunities(
        &self,
        stage: Option<&str>,
        limit: u32,
    ) -> Result<Vec<SalesforceRecord>, String> {
        let token = self.validated_access_token()?;
        let where_clause = match stage {
            Some(s) => format!(" WHERE StageName = '{}'", s),
            None => String::new(),
        };
        let soql = format!(
            "SELECT Id, Name, Amount, StageName, CloseDate, Account.Name FROM Opportunity{} LIMIT {}",
            where_clause, limit
        );

        let url = format!("{}/query", self.api_base());
        let resp = self
            .http_client
            .get(&url)
            .bearer_auth(token)
            .query(&[("q", soql.as_str())])
            .send()
            .await
            .map_err(|e| format!("Network error querying opportunities: {}", e))?;

        let resp = self.check_error_response(resp).await?;
        let body: SalesforceQueryResponse = resp
            .json()
            .await
            .map_err(|e| format!("Failed to parse opportunity query response: {}", e))?;

        Ok(body.records)
    }

    pub async fn create_opportunity(
        &self,
        name: &str,
        account_id: &str,
        amount: f64,
        stage: &str,
        close_date: &str,
    ) -> Result<SalesforceRecord, String> {
        let token = self.validated_access_token()?;
        let url = format!("{}/sobjects/Opportunity/", self.api_base());

        let payload = serde_json::json!({
            "Name": name,
            "AccountId": account_id,
            "Amount": amount,
            "StageName": stage,
            "CloseDate": close_date,
        });

        let resp = self
            .http_client
            .post(&url)
            .bearer_auth(token)
            .header("Content-Type", "application/json")
            .json(&payload)
            .send()
            .await
            .map_err(|e| format!("Network error creating opportunity: {}", e))?;

        let resp = self.check_error_response(resp).await?;
        let body: serde_json::Value = resp
            .json()
            .await
            .map_err(|e| format!("Failed to parse create opportunity response: {}", e))?;

        let id = body["id"]
            .as_str()
            .ok_or_else(|| "Response missing id field".to_string())?
            .to_string();

        Ok(SalesforceRecord {
            id,
            attributes: Some(SalesforceRecordAttributes {
                record_type: "Opportunity".to_string(),
                name: Some(name.to_string()),
            }),
            fields: serde_json::json!({
                "Name": name,
                "AccountId": account_id,
                "Amount": amount,
                "StageName": stage,
                "CloseDate": close_date,
            }),
        })
    }

    // ── Accounts ──────────────────────────────────────────────────

    pub async fn get_accounts(
        &self,
        query: Option<&str>,
        limit: u32,
    ) -> Result<Vec<SalesforceRecord>, String> {
        let token = self.validated_access_token()?;
        let soql = match query {
            Some(q) if !q.is_empty() => format!("{} LIMIT {}", q.trim_end_matches(';'), limit),
            _ => format!(
                "SELECT Id, Name, Industry, Phone, Website FROM Account LIMIT {}",
                limit
            ),
        };

        let url = format!("{}/query", self.api_base());
        let resp = self
            .http_client
            .get(&url)
            .bearer_auth(token)
            .query(&[("q", soql.as_str())])
            .send()
            .await
            .map_err(|e| format!("Network error querying accounts: {}", e))?;

        let resp = self.check_error_response(resp).await?;
        let body: SalesforceQueryResponse = resp
            .json()
            .await
            .map_err(|e| format!("Failed to parse account query response: {}", e))?;

        Ok(body.records)
    }

    // ── Search ────────────────────────────────────────────────────

    pub async fn search(&self, search_term: &str) -> Result<Vec<SalesforceRecord>, String> {
        let token = self.validated_access_token()?;
        let soql = format!("FIND {{{}}}", search_term);

        let url = format!("{}/search/", self.api_base());
        let resp = self
            .http_client
            .get(&url)
            .bearer_auth(token)
            .query(&[("q", soql.as_str())])
            .send()
            .await
            .map_err(|e| format!("Network error performing search: {}", e))?;

        let resp = self.check_error_response(resp).await?;
        let body: serde_json::Value = resp
            .json()
            .await
            .map_err(|e| format!("Failed to parse search response: {}", e))?;

        let records = body["searchRecords"]
            .as_array()
            .map(|arr| {
                arr.iter()
                    .filter_map(|v| serde_json::from_value(v.clone()).ok())
                    .collect()
            })
            .unwrap_or_default();

        Ok(records)
    }

    // ── Describe ──────────────────────────────────────────────────

    pub async fn describe_object(&self, object_name: &str) -> Result<serde_json::Value, String> {
        let token = self.validated_access_token()?;
        let url = format!("{}/sobjects/{}/describe", self.api_base(), object_name);

        let resp = self
            .http_client
            .get(&url)
            .bearer_auth(token)
            .send()
            .await
            .map_err(|e| format!("Network error describing object: {}", e))?;

        let resp = self.check_error_response(resp).await?;
        let body: serde_json::Value = resp
            .json()
            .await
            .map_err(|e| format!("Failed to parse describe response: {}", e))?;

        Ok(body)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::io::{AsyncReadExt, AsyncWriteExt};
    use tokio::net::TcpListener;
    use tokio::sync::oneshot;

    async fn start_server(
        response_body: &'static str,
    ) -> (String, oneshot::Receiver<String>) {
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
    async fn get_contacts_returns_parsed_records() {
        let response = r#"{
            "totalSize": 2,
            "done": true,
            "records": [
                {
                    "id": "003xx000003ABCD",
                    "attributes": {"type": "Contact", "name": "John Doe"},
                    "FirstName": "John",
                    "LastName": "Doe",
                    "Email": "john@example.com"
                },
                {
                    "id": "003xx000003EFGH",
                    "attributes": {"type": "Contact", "name": "Jane Smith"},
                    "FirstName": "Jane",
                    "LastName": "Smith",
                    "Email": "jane@example.com"
                }
            ]
        }"#;
        let (base_url, request_rx) = start_server(response).await;
        let client =
            SalesforceClient::with_base_url_for_test(base_url, "valid-token".to_string());

        let contacts = client.get_contacts("", 10).await.unwrap();
        assert_eq!(contacts.len(), 2);
        assert_eq!(contacts[0].id, "003xx000003ABCD");
        assert_eq!(contacts[0].fields["FirstName"], "John");
        assert_eq!(contacts[1].id, "003xx000003EFGH");
        assert_eq!(contacts[1].fields["LastName"], "Smith");

        let request = request_rx.await.unwrap();
        assert!(request.starts_with("GET /services/data/v58.0/query?"));
        assert!(request.contains("q=SELECT"));
        assert!(
            request.contains("authorization: Bearer valid-token")
                || request.contains("Authorization: Bearer valid-token")
        );
    }

    #[tokio::test]
    async fn create_contact_returns_new_record() {
        let response = r#"{"id": "003xx000003NEW1", "success": true}"#;
        let (base_url, request_rx) = start_server(response).await;
        let client =
            SalesforceClient::with_base_url_for_test(base_url, "test-token".to_string());

        let record = client
            .create_contact("Alice", "Johnson", "alice@test.com", "555-1234", "Acme Corp")
            .await
            .unwrap();
        assert_eq!(record.id, "003xx000003NEW1");
        assert_eq!(record.fields["FirstName"], "Alice");
        assert_eq!(record.fields["Email"], "alice@test.com");

        let request = request_rx.await.unwrap();
        assert!(request.starts_with("POST /services/data/v58.0/sobjects/Contact/"));
        let body = request_body(&request);
        assert_eq!(body["FirstName"], "Alice");
        assert_eq!(body["LastName"], "Johnson");
    }

    #[tokio::test]
    async fn search_returns_matching_records() {
        let response = r#"{
            "searchRecords": [
                {"id": "001xx000003ABC", "Name": "Acme Corp", "attributes": {"type": "Account"}},
                {"id": "003xx000003DEF", "Name": "John Doe", "attributes": {"type": "Contact"}}
            ]
        }"#;
        let (base_url, request_rx) = start_server(response).await;
        let client =
            SalesforceClient::with_base_url_for_test(base_url, "search-token".to_string());

        let results = client.search("Acme").await.unwrap();
        assert_eq!(results.len(), 2);
        assert_eq!(results[0].id, "001xx000003ABC");
        assert_eq!(results[0].fields["Name"], "Acme Corp");

        let request = request_rx.await.unwrap();
        assert!(request.contains("/services/data/v58.0/search/"));
        assert!(request.contains("q=FIND"));
    }

    #[tokio::test]
    async fn handles_salesforce_error_response() {
        let error_body = r#"{"message": "Session expired or invalid", "errorCode": "INVALID_SESSION_ID"}"#;
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let base_url = format!("http://{}", listener.local_addr().unwrap());

        tokio::spawn(async move {
            let (mut stream, _) = listener.accept().await.unwrap();
            let response = format!(
                "HTTP/1.1 401 Unauthorized\r\ncontent-type: application/json\r\ncontent-length: {}\r\nconnection: close\r\n\r\n{}",
                error_body.len(),
                error_body
            );
            stream.write_all(response.as_bytes()).await.unwrap();
        });

        let client =
            SalesforceClient::with_base_url_for_test(base_url, "expired-token".to_string());
        let error = client.get_contacts("", 10).await.unwrap_err();
        assert!(error.contains("INVALID_SESSION_ID"));
        assert!(error.contains("Session expired"));
    }

    #[tokio::test]
    async fn rejects_blank_access_token() {
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let base_url = format!("http://{}", listener.local_addr().unwrap());

        let client =
            SalesforceClient::with_base_url_for_test(base_url, "   ".to_string());
        let error = client.get_contacts("", 10).await.unwrap_err();
        assert_eq!(error, "Salesforce access token is required");
    }
}
