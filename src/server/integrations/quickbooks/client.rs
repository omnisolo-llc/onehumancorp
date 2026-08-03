use reqwest::Client;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct QuickBooksCustomer {
    pub id: Option<String>,
    pub display_name: Option<String>,
    pub given_name: Option<String>,
    pub family_name: Option<String>,
    #[serde(default)]
    pub primary_email_addr: Option<serde_json::Value>,
    #[serde(default)]
    pub primary_phone: Option<serde_json::Value>,
    pub company_name: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct QuickBooksInvoice {
    pub id: Option<String>,
    pub doc_number: Option<String>,
    #[serde(default)]
    pub customer_ref: Option<serde_json::Value>,
    pub total_amt: Option<String>,
    pub balance: Option<String>,
    pub due_date: Option<String>,
    pub txn_date: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct QuickBooksProduct {
    pub id: Option<String>,
    pub name: Option<String>,
    pub sku: Option<String>,
    pub unit_price: Option<String>,
    #[serde(rename = "Type")]
    pub item_type: Option<String>,
    pub description: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct QuickBooksAccount {
    pub id: Option<String>,
    pub name: Option<String>,
    #[serde(rename = "AccountType")]
    pub account_type: Option<String>,
    pub account_sub_type: Option<String>,
    pub balance: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuickBooksLineItem {
    pub description: Option<String>,
    pub amount: String,
    pub detail_type: Option<String>,
    pub sales_item: Option<serde_json::Value>,
}

pub struct QuickBooksClient {
    base_url: String,
    access_token: String,
    http_client: Client,
}

impl QuickBooksClient {
    pub fn new(access_token: String, realm_id: String) -> Self {
        let base_url = format!(
            "https://quickbooks.api.intuit.com/v3/company/{}",
            realm_id
        );
        Self {
            base_url,
            access_token,
            http_client: Client::new(),
        }
    }

    #[cfg(test)]
    fn with_base_url_for_test(base_url: String, access_token: String) -> Self {
        Self {
            base_url,
            access_token,
            http_client: Client::new(),
        }
    }

    fn validated_access_token(&self) -> Result<&str, String> {
        let token = self.access_token.trim();
        if token.is_empty() {
            Err("QuickBooks access token is required".to_string())
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
            return Err(format!("QuickBooks API error {}: {}", status, text));
        }
        Ok(resp)
    }

    // ── Customers ─────────────────────────────────────────────────

    pub async fn get_customers(&self, max_results: u32) -> Result<Vec<QuickBooksCustomer>, String> {
        let token = self.validated_access_token()?;
        let query = format!(
            "SELECT * FROM Customer MAXRESULTS {}",
            max_results
        );
        let url = format!("{}/query", self.api_base());
        let resp = self
            .http_client
            .get(&url)
            .bearer_auth(token)
            .header("Accept", "application/json")
            .query(&[("query", query.as_str())])
            .send()
            .await
            .map_err(|e| format!("Network error querying customers: {}", e))?;

        let resp = self.check_error_response(resp).await?;
        let body: serde_json::Value = resp
            .json()
            .await
            .map_err(|e| format!("Failed to parse customer query response: {}", e))?;

        let entities = body["QueryResponse"]["entities"]
            .as_array()
            .ok_or_else(|| "Missing QueryResponse.entities".to_string())?;

        let customers: Vec<QuickBooksCustomer> = entities
            .iter()
            .filter_map(|v| serde_json::from_value(v.clone()).ok())
            .collect();

        Ok(customers)
    }

    pub async fn create_customer(
        &self,
        name: &str,
        email: &str,
        phone: &str,
        company: &str,
    ) -> Result<QuickBooksCustomer, String> {
        let token = self.validated_access_token()?;
        let url = format!("{}/customer", self.api_base());

        let payload = serde_json::json!({
            "DisplayName": name,
            "PrimaryEmailAddr": { "Address": email },
            "PrimaryPhone": { "FreeFormNumber": phone },
            "CompanyName": company,
        });

        let resp = self
            .http_client
            .post(&url)
            .bearer_auth(token)
            .header("Accept", "application/json")
            .header("Content-Type", "application/json")
            .json(&payload)
            .send()
            .await
            .map_err(|e| format!("Network error creating customer: {}", e))?;

        let resp = self.check_error_response(resp).await?;
        let body: serde_json::Value = resp
            .json()
            .await
            .map_err(|e| format!("Failed to parse create customer response: {}", e))?;

        let customer_val = body.get("Customer")
            .ok_or_else(|| "Response missing Customer field".to_string())?;

        serde_json::from_value(customer_val.clone())
            .map_err(|e| format!("Failed to deserialize customer: {}", e))
    }

    // ── Invoices ──────────────────────────────────────────────────

    pub async fn get_invoices(&self, max_results: u32) -> Result<Vec<QuickBooksInvoice>, String> {
        let token = self.validated_access_token()?;
        let query = format!(
            "SELECT * FROM Invoice MAXRESULTS {}",
            max_results
        );
        let url = format!("{}/query", self.api_base());
        let resp = self
            .http_client
            .get(&url)
            .bearer_auth(token)
            .header("Accept", "application/json")
            .query(&[("query", query.as_str())])
            .send()
            .await
            .map_err(|e| format!("Network error querying invoices: {}", e))?;

        let resp = self.check_error_response(resp).await?;
        let body: serde_json::Value = resp
            .json()
            .await
            .map_err(|e| format!("Failed to parse invoice query response: {}", e))?;

        let entities = body["QueryResponse"]["entities"]
            .as_array()
            .ok_or_else(|| "Missing QueryResponse.entities".to_string())?;

        let invoices: Vec<QuickBooksInvoice> = entities
            .iter()
            .filter_map(|v| serde_json::from_value(v.clone()).ok())
            .collect();

        Ok(invoices)
    }

    pub async fn create_invoice(
        &self,
        customer_id: &str,
        line_items: &[QuickBooksLineItem],
    ) -> Result<QuickBooksInvoice, String> {
        let token = self.validated_access_token()?;
        let url = format!("{}/invoice", self.api_base());

        let items: Vec<serde_json::Value> = line_items
            .iter()
            .map(|li| {
                serde_json::json!({
                    "Description": li.description,
                    "Amount": li.amount,
                    "DetailType": li.detail_type.as_deref().unwrap_or("SalesItemLineDetail"),
                    "SalesItemLineDetail": li.sales_item,
                })
            })
            .collect();

        let payload = serde_json::json!({
            "CustomerRef": { "value": customer_id },
            "Line": items,
        });

        let resp = self
            .http_client
            .post(&url)
            .bearer_auth(token)
            .header("Accept", "application/json")
            .header("Content-Type", "application/json")
            .json(&payload)
            .send()
            .await
            .map_err(|e| format!("Network error creating invoice: {}", e))?;

        let resp = self.check_error_response(resp).await?;
        let body: serde_json::Value = resp
            .json()
            .await
            .map_err(|e| format!("Failed to parse create invoice response: {}", e))?;

        let inv_val = body.get("Invoice")
            .ok_or_else(|| "Response missing Invoice field".to_string())?;

        serde_json::from_value(inv_val.clone())
            .map_err(|e| format!("Failed to deserialize invoice: {}", e))
    }

    // ── Products ──────────────────────────────────────────────────

    pub async fn get_products(&self, max_results: u32) -> Result<Vec<QuickBooksProduct>, String> {
        let token = self.validated_access_token()?;
        let query = format!(
            "SELECT * FROM Item MAXRESULTS {}",
            max_results
        );
        let url = format!("{}/query", self.api_base());
        let resp = self
            .http_client
            .get(&url)
            .bearer_auth(token)
            .header("Accept", "application/json")
            .query(&[("query", query.as_str())])
            .send()
            .await
            .map_err(|e| format!("Network error querying products: {}", e))?;

        let resp = self.check_error_response(resp).await?;
        let body: serde_json::Value = resp
            .json()
            .await
            .map_err(|e| format!("Failed to parse product query response: {}", e))?;

        let entities = body["QueryResponse"]["entities"]
            .as_array()
            .ok_or_else(|| "Missing QueryResponse.entities".to_string())?;

        let products: Vec<QuickBooksProduct> = entities
            .iter()
            .filter_map(|v| serde_json::from_value(v.clone()).ok())
            .collect();

        Ok(products)
    }

    // ── Accounts ──────────────────────────────────────────────────

    pub async fn get_accounts(&self) -> Result<Vec<QuickBooksAccount>, String> {
        let token = self.validated_access_token()?;
        let query = "SELECT * FROM Account";
        let url = format!("{}/query", self.api_base());
        let resp = self
            .http_client
            .get(&url)
            .bearer_auth(token)
            .header("Accept", "application/json")
            .query(&[("query", query)])
            .send()
            .await
            .map_err(|e| format!("Network error querying accounts: {}", e))?;

        let resp = self.check_error_response(resp).await?;
        let body: serde_json::Value = resp
            .json()
            .await
            .map_err(|e| format!("Failed to parse account query response: {}", e))?;

        let entities = body["QueryResponse"]["entities"]
            .as_array()
            .ok_or_else(|| "Missing QueryResponse.entities".to_string())?;

        let accounts: Vec<QuickBooksAccount> = entities
            .iter()
            .filter_map(|v| serde_json::from_value(v.clone()).ok())
            .collect();

        Ok(accounts)
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
    async fn get_customers_returns_parsed_records() {
        let response = r#"{
            "QueryResponse": {
                "entities": [
                    {
                        "Id": "1",
                        "DisplayName": "Acme Corp",
                        "CompanyName": "Acme"
                    },
                    {
                        "Id": "2",
                        "DisplayName": "Globex Inc",
                        "CompanyName": "Globex"
                    }
                ]
            }
        }"#;
        let (base_url, request_rx) = start_server(response).await;
        let client =
            QuickBooksClient::with_base_url_for_test(base_url, "valid-token".to_string());

        let customers = client.get_customers(10).await.unwrap();
        assert_eq!(customers.len(), 2);
        assert_eq!(customers[0].id.as_deref(), Some("1"));
        assert_eq!(customers[0].display_name.as_deref(), Some("Acme Corp"));

        let request = request_rx.await.unwrap();
        assert!(request.starts_with("GET /query?"));
        assert!(request.contains("SELECT"));
        assert!(
            request.contains("authorization: Bearer valid-token")
                || request.contains("Authorization: Bearer valid-token")
        );
    }

    #[tokio::test]
    async fn create_customer_returns_new_record() {
        let response = r#"{
            "Customer": {
                "Id": "3",
                "DisplayName": "New Customer",
                "PrimaryEmailAddr": { "Address": "new@test.com" },
                "CompanyName": "Test Co"
            }
        }"#;
        let (base_url, request_rx) = start_server(response).await;
        let client =
            QuickBooksClient::with_base_url_for_test(base_url, "test-token".to_string());

        let customer = client
            .create_customer("New Customer", "new@test.com", "555-0000", "Test Co")
            .await
            .unwrap();
        assert_eq!(customer.id.as_deref(), Some("3"));
        assert_eq!(customer.display_name.as_deref(), Some("New Customer"));

        let request = request_rx.await.unwrap();
        assert!(request.starts_with("POST /customer"));
        let body = request_body(&request);
        assert_eq!(body["DisplayName"], "New Customer");
        assert_eq!(body["PrimaryEmailAddr"]["Address"], "new@test.com");
    }

    #[tokio::test]
    async fn get_invoices_returns_parsed_records() {
        let response = r#"{
            "QueryResponse": {
                "entities": [
                    {
                        "Id": "101",
                        "DocNumber": "INV-001",
                        "TotalAmt": "500.00"
                    }
                ]
            }
        }"#;
        let (base_url, _request_rx) = start_server(response).await;
        let client =
            QuickBooksClient::with_base_url_for_test(base_url, "test-token".to_string());

        let invoices = client.get_invoices(5).await.unwrap();
        assert_eq!(invoices.len(), 1);
        assert_eq!(invoices[0].id.as_deref(), Some("101"));
        assert_eq!(invoices[0].doc_number.as_deref(), Some("INV-001"));
    }

    #[tokio::test]
    async fn get_products_returns_parsed_records() {
        let response = r#"{
            "QueryResponse": {
                "entities": [
                    {
                        "Id": "201",
                        "Name": "Widget",
                        "UnitPrice": "19.99"
                    }
                ]
            }
        }"#;
        let (base_url, _request_rx) = start_server(response).await;
        let client =
            QuickBooksClient::with_base_url_for_test(base_url, "test-token".to_string());

        let products = client.get_products(10).await.unwrap();
        assert_eq!(products.len(), 1);
        assert_eq!(products[0].id.as_deref(), Some("201"));
        assert_eq!(products[0].name.as_deref(), Some("Widget"));
    }

    #[tokio::test]
    async fn handles_quickbooks_error_response() {
        let error_body = r#"{"Fault": {"Error": [{"Message": "Invalid token"}], "type": "AUTHENTICATION"}}"#;
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
            QuickBooksClient::with_base_url_for_test(base_url, "expired-token".to_string());
        let error = client.get_customers(10).await.unwrap_err();
        assert!(error.contains("QuickBooks API error"));
    }

    #[tokio::test]
    async fn rejects_blank_access_token() {
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let base_url = format!("http://{}", listener.local_addr().unwrap());

        let client =
            QuickBooksClient::with_base_url_for_test(base_url, "   ".to_string());
        let error = client.get_customers(10).await.unwrap_err();
        assert_eq!(error, "QuickBooks access token is required");
    }
}
