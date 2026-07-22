use reqwest::Client;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShopifyProduct {
    pub id: Option<String>,
    pub title: Option<String>,
    pub description: Option<String>,
    pub vendor: Option<String>,
    pub product_type: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShopifyOrder {
    pub id: Option<String>,
    pub order_number: Option<u64>,
    pub name: Option<String>,
    pub total_price: Option<String>,
    pub financial_status: Option<String>,
    pub fulfillment_status: Option<String>,
    pub created_at: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShopifyInventoryItem {
    pub inventory_item_id: Option<String>,
    pub location_id: Option<String>,
    pub available: Option<i32>,
    #[serde(flatten)]
    pub extra: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShopifyCustomer {
    pub id: Option<String>,
    pub email: Option<String>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Value,
}

#[derive(Debug, Deserialize)]
struct GraphQLError {
    message: String,
}

#[derive(Debug, Deserialize)]
struct GraphQLResponse {
    data: Option<serde_json::Value>,
    errors: Option<Vec<GraphQLError>>,
}

pub struct ShopifyClient {
    store_url: String,
    access_token: String,
    http_client: Client,
}

impl ShopifyClient {
    pub fn new(store_url: String, access_token: String) -> Self {
        Self {
            store_url: store_url.trim_end_matches('/').to_string(),
            access_token,
            http_client: Client::new(),
        }
    }

    #[cfg(test)]
    fn with_base_url_for_test(base_url: String, access_token: String) -> Self {
        Self {
            store_url: base_url,
            access_token,
            http_client: Client::new(),
        }
    }

    fn validated_access_token(&self) -> Result<&str, String> {
        let token = self.access_token.trim();
        if token.is_empty() {
            Err("Shopify access token is required".to_string())
        } else {
            Ok(token)
        }
    }

    fn graphql_url(&self) -> String {
        format!("{}/admin/api/2024-01/graphql.json", self.store_url)
    }

    async fn graphql_request(
        &self,
        query: &str,
        variables: Option<serde_json::Value>,
    ) -> Result<serde_json::Value, String> {
        let token = self.validated_access_token()?;
        let url = self.graphql_url();

        let mut payload = serde_json::json!({ "query": query });
        if let Some(vars) = variables {
            payload["variables"] = vars;
        }

        let resp = self
            .http_client
            .post(&url)
            .header("X-Shopify-Access-Token", token)
            .header("Content-Type", "application/json")
            .json(&payload)
            .send()
            .await
            .map_err(|e| format!("Network error: {}", e))?;

        if !resp.status().is_success() {
            let status = resp.status();
            let text = resp.text().await.unwrap_or_default();
            return Err(format!("Shopify API error {}: {}", status, text));
        }

        let body: GraphQLResponse = resp
            .json()
            .await
            .map_err(|e| format!("Failed to parse GraphQL response: {}", e))?;

        if let Some(errors) = body.errors {
            let msgs: Vec<String> = errors.iter().map(|e| e.message.clone()).collect();
            return Err(format!("GraphQL errors: {}", msgs.join(", ")));
        }

        body.data
            .ok_or_else(|| "No data in GraphQL response".to_string())
    }

    // ── Products ──────────────────────────────────────────────────

    pub async fn get_products(&self, first: u32) -> Result<Vec<ShopifyProduct>, String> {
        let query = format!(
            r#"query {{ products(first: {}) {{ edges {{ node {{ id title description vendor productType }} }} }} }}"#,
            first
        );

        let data = self.graphql_request(&query, None).await?;
        let edges = data["products"]["edges"]
            .as_array()
            .ok_or_else(|| "Missing products.edges".to_string())?;

        let products: Vec<ShopifyProduct> = edges
            .iter()
            .filter_map(|edge| {
                let node = &edge["node"];
                Some(ShopifyProduct {
                    id: node["id"].as_str().map(String::from),
                    title: node["title"].as_str().map(String::from),
                    description: node["description"].as_str().map(String::from),
                    vendor: node["vendor"].as_str().map(String::from),
                    product_type: node["productType"].as_str().map(String::from),
                    extra: node.clone(),
                })
            })
            .collect();

        Ok(products)
    }

    pub async fn create_product(
        &self,
        title: &str,
        description: &str,
        price: &str,
    ) -> Result<ShopifyProduct, String> {
        let query = r#"mutation productCreate($input: ProductInput!) {
            productCreate(input: $input) {
                product { id title description vendor }
                userErrors { message }
            }
        }"#;

        let variables = serde_json::json!({
            "input": {
                "title": title,
                "descriptionHtml": description,
                "variants": [{ "price": price }]
            }
        });

        let data = self.graphql_request(query, Some(variables)).await?;

        let errors = &data["productCreate"]["userErrors"];
        if errors.is_array() && errors.as_array().unwrap().len() > 0 {
            let msg = errors[0]["message"]
                .as_str()
                .unwrap_or("unknown error");
            return Err(format!("Product creation error: {}", msg));
        }

        let node = &data["productCreate"]["product"];
        Ok(ShopifyProduct {
            id: node["id"].as_str().map(String::from),
            title: node["title"].as_str().map(String::from),
            description: node["description"].as_str().map(String::from),
            vendor: node["vendor"].as_str().map(String::from),
            product_type: None,
            extra: node.clone(),
        })
    }

    // ── Orders ────────────────────────────────────────────────────

    pub async fn get_orders(
        &self,
        first: u32,
        status: Option<&str>,
    ) -> Result<Vec<ShopifyOrder>, String> {
        let status_filter = match status {
            Some(s) => format!(", query: \"status:{}\"", s),
            None => String::new(),
        };
        let query = format!(
            r#"query {{ orders(first: {}{}) {{ edges {{ node {{ id orderNumber name totalPrice financialStatus fulfillmentStatus createdAt }} }} }} }}"#,
            first, status_filter
        );

        let data = self.graphql_request(&query, None).await?;
        let edges = data["orders"]["edges"]
            .as_array()
            .ok_or_else(|| "Missing orders.edges".to_string())?;

        let orders: Vec<ShopifyOrder> = edges
            .iter()
            .filter_map(|edge| {
                let node = &edge["node"];
                Some(ShopifyOrder {
                    id: node["id"].as_str().map(String::from),
                    order_number: node["orderNumber"].as_u64(),
                    name: node["name"].as_str().map(String::from),
                    total_price: node["totalPrice"].as_str().map(String::from),
                    financial_status: node["financialStatus"].as_str().map(String::from),
                    fulfillment_status: node["fulfillmentStatus"].as_str().map(String::from),
                    created_at: node["createdAt"].as_str().map(String::from),
                    extra: node.clone(),
                })
            })
            .collect();

        Ok(orders)
    }

    // ── Inventory ─────────────────────────────────────────────────

    pub async fn get_inventory_levels(
        &self,
        location_id: &str,
    ) -> Result<Vec<ShopifyInventoryItem>, String> {
        let query = format!(
            r#"query {{ inventoryLevels(locationId: "{}") {{ edges {{ node {{ inventoryItem {{ id }} available location {{ id }} }} }} }} }}"#,
            location_id
        );

        let data = self.graphql_request(&query, None).await?;
        let edges = data["inventoryLevels"]["edges"]
            .as_array()
            .ok_or_else(|| "Missing inventoryLevels.edges".to_string())?;

        let items: Vec<ShopifyInventoryItem> = edges
            .iter()
            .filter_map(|edge| {
                let node = &edge["node"];
                Some(ShopifyInventoryItem {
                    inventory_item_id: node["inventoryItem"]["id"].as_str().map(String::from),
                    location_id: node["location"]["id"].as_str().map(String::from),
                    available: node["available"].as_i64().map(|v| v as i32),
                    extra: node.clone(),
                })
            })
            .collect();

        Ok(items)
    }

    pub async fn update_inventory(
        &self,
        inventory_item_id: &str,
        location_id: &str,
        available: i32,
    ) -> Result<(), String> {
        let query = r#"mutation inventoryAdjustment($input: InventoryAdjustInput!) {
            inventoryAdjustment(input: $input) {
                inventoryAdjustment { id }
                userErrors { message }
            }
        }"#;

        let variables = serde_json::json!({
            "input": {
                "inventoryItemId": inventory_item_id,
                "locationId": location_id,
                "delta": available
            }
        });

        let data = self.graphql_request(query, Some(variables)).await?;

        let errors = &data["inventoryAdjustment"]["userErrors"];
        if errors.is_array() && errors.as_array().unwrap().len() > 0 {
            let msg = errors[0]["message"]
                .as_str()
                .unwrap_or("unknown error");
            return Err(format!("Inventory adjustment error: {}", msg));
        }

        Ok(())
    }

    // ── Customers ─────────────────────────────────────────────────

    pub async fn get_customers(&self, first: u32) -> Result<Vec<ShopifyCustomer>, String> {
        let query = format!(
            r#"query {{ customers(first: {}) {{ edges {{ node {{ id email firstName lastName }} }} }} }}"#,
            first
        );

        let data = self.graphql_request(&query, None).await?;
        let edges = data["customers"]["edges"]
            .as_array()
            .ok_or_else(|| "Missing customers.edges".to_string())?;

        let customers: Vec<ShopifyCustomer> = edges
            .iter()
            .filter_map(|edge| {
                let node = &edge["node"];
                Some(ShopifyCustomer {
                    id: node["id"].as_str().map(String::from),
                    email: node["email"].as_str().map(String::from),
                    first_name: node["firstName"].as_str().map(String::from),
                    last_name: node["lastName"].as_str().map(String::from),
                    extra: node.clone(),
                })
            })
            .collect();

        Ok(customers)
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
    async fn get_products_returns_parsed_records() {
        let response = r#"{
            "data": {
                "products": {
                    "edges": [
                        { "node": { "id": "gid://shopify/Product/1", "title": "Widget", "vendor": "Acme" } },
                        { "node": { "id": "gid://shopify/Product/2", "title": "Gadget", "vendor": "Globex" } }
                    ]
                }
            }
        }"#;
        let (base_url, request_rx) = start_server(response).await;
        let client =
            ShopifyClient::with_base_url_for_test(base_url, "valid-token".to_string());

        let products = client.get_products(10).await.unwrap();
        assert_eq!(products.len(), 2);
        assert_eq!(products[0].id.as_deref(), Some("gid://shopify/Product/1"));
        assert_eq!(products[0].title.as_deref(), Some("Widget"));

        let request = request_rx.await.unwrap();
        assert!(request.starts_with("POST"));
        assert!(
            request.contains("x-shopify-access-token: valid-token")
                || request.contains("X-Shopify-Access-Token: valid-token")
        );
        let body = request_body(&request);
        assert!(body["query"].as_str().unwrap().contains("products"));
    }

    #[tokio::test]
    async fn create_product_returns_new_record() {
        let response = r#"{
            "data": {
                "productCreate": {
                    "product": {
                        "id": "gid://shopify/Product/3",
                        "title": "New Product",
                        "description": "A new product"
                    },
                    "userErrors": []
                }
            }
        }"#;
        let (base_url, request_rx) = start_server(response).await;
        let client =
            ShopifyClient::with_base_url_for_test(base_url, "test-token".to_string());

        let product = client
            .create_product("New Product", "A new product", "29.99")
            .await
            .unwrap();
        assert_eq!(product.id.as_deref(), Some("gid://shopify/Product/3"));
        assert_eq!(product.title.as_deref(), Some("New Product"));

        let request = request_rx.await.unwrap();
        assert!(request.starts_with("POST"));
        let body = request_body(&request);
        assert!(body["query"].as_str().unwrap().contains("productCreate"));
    }

    #[tokio::test]
    async fn get_orders_returns_parsed_records() {
        let response = r##"{
            "data": {
                "orders": {
                    "edges": [
                        { "node": { "id": "gid://shopify/Order/1", "orderNumber": 1001, "name": "#1001", "totalPrice": "50.00", "financialStatus": "paid" } }
                    ]
                }
            }
        }"##;
        let (base_url, _request_rx) = start_server(response).await;
        let client =
            ShopifyClient::with_base_url_for_test(base_url, "test-token".to_string());

        let orders = client.get_orders(10, None).await.unwrap();
        assert_eq!(orders.len(), 1);
        assert_eq!(orders[0].id.as_deref(), Some("gid://shopify/Order/1"));
        assert_eq!(orders[0].order_number, Some(1001));
    }

    #[tokio::test]
    async fn get_customers_returns_parsed_records() {
        let response = r#"{
            "data": {
                "customers": {
                    "edges": [
                        { "node": { "id": "gid://shopify/Customer/1", "email": "john@test.com", "firstName": "John", "lastName": "Doe" } }
                    ]
                }
            }
        }"#;
        let (base_url, _request_rx) = start_server(response).await;
        let client =
            ShopifyClient::with_base_url_for_test(base_url, "test-token".to_string());

        let customers = client.get_customers(10).await.unwrap();
        assert_eq!(customers.len(), 1);
        assert_eq!(customers[0].email.as_deref(), Some("john@test.com"));
    }

    #[tokio::test]
    async fn handles_shopify_error_response() {
        let error_body = r#"{"errors": "Unauthorized"}"#;
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
            ShopifyClient::with_base_url_for_test(base_url, "expired-token".to_string());
        let error = client.get_products(10).await.unwrap_err();
        assert!(error.contains("Shopify API error"));
    }

    #[tokio::test]
    async fn handles_graphql_errors() {
        let response = r#"{
            "errors": [{ "message": "Cannot query field 'invalid' on type 'QueryRoot'" }]
        }"#;
        let (base_url, _request_rx) = start_server(response).await;
        let client =
            ShopifyClient::with_base_url_for_test(base_url, "test-token".to_string());

        let error = client.get_products(10).await.unwrap_err();
        assert!(error.contains("GraphQL errors"));
    }

    #[tokio::test]
    async fn rejects_blank_access_token() {
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let base_url = format!("http://{}", listener.local_addr().unwrap());

        let client =
            ShopifyClient::with_base_url_for_test(base_url, "   ".to_string());
        let error = client.get_products(10).await.unwrap_err();
        assert_eq!(error, "Shopify access token is required");
    }
}
