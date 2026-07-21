use reqwest::Client;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct XeroContact {
    #[serde(rename = "contactID")]
    pub contact_id: Option<String>,
    pub name: Option<String>,
    #[serde(rename = "emailAddress")]
    pub email_address: Option<String>,
    #[serde(default)]
    pub phones: Option<serde_json::Value>,
    #[serde(flatten)]
    pub extra: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct XeroInvoice {
    #[serde(rename = "invoiceID")]
    pub invoice_id: Option<String>,
    #[serde(default)]
    pub contact: Option<serde_json::Value>,
    #[serde(rename = "invoiceNumber")]
    pub invoice_number: Option<String>,
    pub status: Option<String>,
    pub total: Option<f64>,
    pub subtotal: Option<f64>,
    pub date: Option<String>,
    #[serde(rename = "dueDate")]
    pub due_date: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct XeroAccount {
    #[serde(rename = "accountID")]
    pub account_id: Option<String>,
    pub name: Option<String>,
    pub code: Option<String>,
    #[serde(rename = "accountType")]
    pub account_type: Option<String>,
    pub status: Option<String>,
    pub balance: Option<f64>,
    #[serde(flatten)]
    pub extra: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct XeroLineItem {
    pub description: String,
    pub quantity: f64,
    pub unit_amount: f64,
    #[serde(rename = "AccountCode")]
    pub account_code: String,
}

#[derive(Debug, Deserialize)]
struct ContactsResponse {
    contacts: Vec<XeroContact>,
}

#[derive(Debug, Deserialize)]
struct InvoicesResponse {
    invoices: Vec<XeroInvoice>,
}

#[derive(Debug, Deserialize)]
struct AccountsResponse {
    accounts: Vec<XeroAccount>,
}

pub struct XeroClient {
    base_url: String,
    access_token: String,
    tenant_id: String,
    http_client: Client,
}

impl XeroClient {
    pub fn new(access_token: String, tenant_id: String) -> Self {
        Self {
            base_url: "https://api.xero.com/api.xro/2.0".to_string(),
            access_token,
            tenant_id,
            http_client: Client::new(),
        }
    }

    #[cfg(test)]
    fn with_base_url_for_test(base_url: String, access_token: String, tenant_id: String) -> Self {
        Self {
            base_url,
            access_token,
            tenant_id,
            http_client: Client::new(),
        }
    }

    fn validated_access_token(&self) -> Result<&str, String> {
        let token = self.access_token.trim();
        if token.is_empty() {
            Err("Xero access token is required".to_string())
        } else {
            Ok(token)
        }
    }

    fn api_base(&self) -> &str {
        &self.base_url
    }

    async fn check_error_response(&self, resp: reqwest::Response) -> Result<reqwest::Response, String> {
        if !resp.status().is_success() {
            let status = resp.status();
            let text = resp
                .text()
                .await
                .unwrap_or_else(|_| "<unreadable>".to_string());
            return Err(format!("Xero API error {}: {}", status, text));
        }
        Ok(resp)
    }

    // ── Contacts ──────────────────────────────────────────────────

    pub async fn get_contacts(&self) -> Result<Vec<XeroContact>, String> {
        let token = self.validated_access_token()?;
        let url = format!("{}/Contacts", self.api_base());
        let resp = self
            .http_client
            .get(&url)
            .bearer_auth(token)
            .header("Xero-Tenant-Id", &self.tenant_id)
            .header("Accept", "application/json")
            .send()
            .await
            .map_err(|e| format!("Network error querying contacts: {}", e))?;

        let resp = self.check_error_response(resp).await?;
        let body: ContactsResponse = resp
            .json()
            .await
            .map_err(|e| format!("Failed to parse contacts response: {}", e))?;

        Ok(body.contacts)
    }

    pub async fn create_contact(
        &self,
        name: &str,
        email: &str,
    ) -> Result<XeroContact, String> {
        let token = self.validated_access_token()?;
        let url = format!("{}/Contacts", self.api_base());

        let payload = serde_json::json!({
            "contacts": [{
                "name": name,
                "emailAddress": email
            }]
        });

        let resp = self
            .http_client
            .put(&url)
            .bearer_auth(token)
            .header("Xero-Tenant-Id", &self.tenant_id)
            .header("Content-Type", "application/json")
            .header("Accept", "application/json")
            .json(&payload)
            .send()
            .await
            .map_err(|e| format!("Network error creating contact: {}", e))?;

        let resp = self.check_error_response(resp).await?;
        let body: ContactsResponse = resp
            .json()
            .await
            .map_err(|e| format!("Failed to parse create contact response: {}", e))?;

        body.contacts
            .into_iter()
            .next()
            .ok_or_else(|| "No contact in response".to_string())
    }

    // ── Invoices ──────────────────────────────────────────────────

    pub async fn get_invoices(&self) -> Result<Vec<XeroInvoice>, String> {
        let token = self.validated_access_token()?;
        let url = format!("{}/Invoices", self.api_base());
        let resp = self
            .http_client
            .get(&url)
            .bearer_auth(token)
            .header("Xero-Tenant-Id", &self.tenant_id)
            .header("Accept", "application/json")
            .send()
            .await
            .map_err(|e| format!("Network error querying invoices: {}", e))?;

        let resp = self.check_error_response(resp).await?;
        let body: InvoicesResponse = resp
            .json()
            .await
            .map_err(|e| format!("Failed to parse invoices response: {}", e))?;

        Ok(body.invoices)
    }

    pub async fn create_invoice(
        &self,
        contact_id: &str,
        line_items: &[XeroLineItem],
    ) -> Result<XeroInvoice, String> {
        let token = self.validated_access_token()?;
        let url = format!("{}/Invoices", self.api_base());

        let items: Vec<serde_json::Value> = line_items
            .iter()
            .map(|li| {
                serde_json::json!({
                    "description": li.description,
                    "quantity": li.quantity,
                    "unitAmount": li.unit_amount,
                    "accountCode": li.account_code,
                })
            })
            .collect();

        let payload = serde_json::json!({
            "invoices": [{
                "contact": { "contactID": contact_id },
                "lineItems": items,
                "type": "ACCREC"
            }]
        });

        let resp = self
            .http_client
            .put(&url)
            .bearer_auth(token)
            .header("Xero-Tenant-Id", &self.tenant_id)
            .header("Content-Type", "application/json")
            .header("Accept", "application/json")
            .json(&payload)
            .send()
            .await
            .map_err(|e| format!("Network error creating invoice: {}", e))?;

        let resp = self.check_error_response(resp).await?;
        let body: InvoicesResponse = resp
            .json()
            .await
            .map_err(|e| format!("Failed to parse create invoice response: {}", e))?;

        body.invoices
            .into_iter()
            .next()
            .ok_or_else(|| "No invoice in response".to_string())
    }

    // ── Accounts ──────────────────────────────────────────────────

    pub async fn get_accounts(&self) -> Result<Vec<XeroAccount>, String> {
        let token = self.validated_access_token()?;
        let url = format!("{}/Accounts", self.api_base());
        let resp = self
            .http_client
            .get(&url)
            .bearer_auth(token)
            .header("Xero-Tenant-Id", &self.tenant_id)
            .header("Accept", "application/json")
            .send()
            .await
            .map_err(|e| format!("Network error querying accounts: {}", e))?;

        let resp = self.check_error_response(resp).await?;
        let body: AccountsResponse = resp
            .json()
            .await
            .map_err(|e| format!("Failed to parse accounts response: {}", e))?;

        Ok(body.accounts)
    }

    // ── Modified Since ────────────────────────────────────────────

    pub async fn get_contacts_modified_since(
        &self,
        since: &str,
    ) -> Result<Vec<XeroContact>, String> {
        let token = self.validated_access_token()?;
        let url = format!(
            "{}/Contacts?where=ModifiedDate>=DateTime('{}')",
            self.api_base(),
            since
        );
        let resp = self
            .http_client
            .get(&url)
            .bearer_auth(token)
            .header("Xero-Tenant-Id", &self.tenant_id)
            .header("Accept", "application/json")
            .send()
            .await
            .map_err(|e| format!("Network error querying modified contacts: {}", e))?;

        let resp = self.check_error_response(resp).await?;
        let body: ContactsResponse = resp
            .json()
            .await
            .map_err(|e| format!("Failed to parse modified contacts response: {}", e))?;

        Ok(body.contacts)
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
            "contacts": [
                {
                    "contactID": "c1",
                    "name": "Acme Corp",
                    "emailAddress": "acme@example.com"
                },
                {
                    "contactID": "c2",
                    "name": "Globex",
                    "emailAddress": "globex@example.com"
                }
            ]
        }"#;
        let (base_url, request_rx) = start_server(response).await;
        let client =
            XeroClient::with_base_url_for_test(base_url, "valid-token".to_string(), "tenant-123".to_string());

        let contacts = client.get_contacts().await.unwrap();
        assert_eq!(contacts.len(), 2);
        assert_eq!(contacts[0].contact_id.as_deref(), Some("c1"));
        assert_eq!(contacts[0].name.as_deref(), Some("Acme Corp"));

        let request = request_rx.await.unwrap();
        assert!(request.starts_with("GET /Contacts"));
        assert!(request.contains("xero-tenant-id: tenant-123"));
        assert!(
            request.contains("authorization: Bearer valid-token")
                || request.contains("Authorization: Bearer valid-token")
        );
    }

    #[tokio::test]
    async fn create_contact_returns_new_record() {
        let response = r#"{
            "contacts": [
                {
                    "contactID": "c3",
                    "name": "New Contact",
                    "emailAddress": "new@test.com"
                }
            ]
        }"#;
        let (base_url, request_rx) = start_server(response).await;
        let client =
            XeroClient::with_base_url_for_test(base_url, "test-token".to_string(), "tenant-456".to_string());

        let contact = client
            .create_contact("New Contact", "new@test.com")
            .await
            .unwrap();
        assert_eq!(contact.contact_id.as_deref(), Some("c3"));
        assert_eq!(contact.name.as_deref(), Some("New Contact"));

        let request = request_rx.await.unwrap();
        assert!(request.starts_with("PUT /Contacts"));
        let body = request_body(&request);
        assert_eq!(body["contacts"][0]["name"], "New Contact");
    }

    #[tokio::test]
    async fn get_invoices_returns_parsed_records() {
        let response = r#"{
            "invoices": [
                {
                    "invoiceID": "i1",
                    "invoiceNumber": "INV-001",
                    "total": 100.0,
                    "status": "AUTHORISED"
                }
            ]
        }"#;
        let (base_url, _request_rx) = start_server(response).await;
        let client =
            XeroClient::with_base_url_for_test(base_url, "test-token".to_string(), "tenant-789".to_string());

        let invoices = client.get_invoices().await.unwrap();
        assert_eq!(invoices.len(), 1);
        assert_eq!(invoices[0].invoice_id.as_deref(), Some("i1"));
        assert_eq!(invoices[0].status.as_deref(), Some("AUTHORISED"));
    }

    #[tokio::test]
    async fn get_accounts_returns_parsed_records() {
        let response = r#"{
            "accounts": [
                {
                    "accountID": "a1",
                    "name": "Sales",
                    "code": "200",
                    "accountType": "REVENUE"
                }
            ]
        }"#;
        let (base_url, _request_rx) = start_server(response).await;
        let client =
            XeroClient::with_base_url_for_test(base_url, "test-token".to_string(), "tenant-abc".to_string());

        let accounts = client.get_accounts().await.unwrap();
        assert_eq!(accounts.len(), 1);
        assert_eq!(accounts[0].account_id.as_deref(), Some("a1"));
        assert_eq!(accounts[0].name.as_deref(), Some("Sales"));
    }

    #[tokio::test]
    async fn handles_xero_error_response() {
        let error_body = r#"{"Type": "Unauthorized", "Title": "Unauthorized", "Status": 401, "detail": "Invalid token"}"#;
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
            XeroClient::with_base_url_for_test(base_url, "expired-token".to_string(), "tenant-err".to_string());
        let error = client.get_contacts().await.unwrap_err();
        assert!(error.contains("Xero API error"));
    }

    #[tokio::test]
    async fn rejects_blank_access_token() {
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let base_url = format!("http://{}", listener.local_addr().unwrap());

        let client =
            XeroClient::with_base_url_for_test(base_url, "   ".to_string(), "tenant-blank".to_string());
        let error = client.get_contacts().await.unwrap_err();
        assert_eq!(error, "Xero access token is required");
    }
}
