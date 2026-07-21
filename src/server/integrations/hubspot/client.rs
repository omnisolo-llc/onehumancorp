use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HubSpotContact {
    pub id: String,
    pub properties: serde_json::Value,
    #[serde(default)]
    pub created_at: Option<String>,
    #[serde(default)]
    pub updated_at: Option<String>,
    #[serde(default)]
    pub archived: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HubSpotDeal {
    pub id: String,
    pub properties: serde_json::Value,
    #[serde(default)]
    pub created_at: Option<String>,
    #[serde(default)]
    pub updated_at: Option<String>,
    #[serde(default)]
    pub archived: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HubSpotCompany {
    pub id: String,
    pub properties: serde_json::Value,
    #[serde(default)]
    pub created_at: Option<String>,
    #[serde(default)]
    pub updated_at: Option<String>,
    #[serde(default)]
    pub archived: bool,
}

#[derive(Debug, Deserialize)]
struct HubSpotListResponse<T> {
    results: Vec<T>,
    paging: Option<HubSpotPaging>,
}

#[derive(Debug, Deserialize)]
struct HubSpotPaging {
    next: Option<HubSpotPageLink>,
}

#[derive(Debug, Deserialize)]
struct HubSpotPageLink {
    after: String,
    link: String,
}

#[derive(Debug, Deserialize)]
struct HubSpotSearchResponse<T> {
    total: u64,
    results: Vec<T>,
}

#[derive(Debug, Deserialize)]
struct HubSpotError {
    status: Option<String>,
    message: Option<String>,
}

pub struct HubSpotClient {
    access_token: String,
    http_client: reqwest::Client,
}

impl HubSpotClient {
    pub fn new(access_token: String) -> Self {
        Self {
            access_token,
            http_client: reqwest::Client::new(),
        }
    }

    fn base_url(&self) -> &str {
        "https://api.hubapi.com"
    }

    async fn send_request(
        &self,
        method: reqwest::Method,
        path: &str,
        body: Option<serde_json::Value>,
    ) -> Result<reqwest::Response, String> {
        let url = format!("{}{}", self.base_url(), path);
        let mut request = self
            .http_client
            .request(method, &url)
            .bearer_auth(&self.access_token)
            .header("Content-Type", "application/json");

        if let Some(body) = body {
            request = request.json(&body);
        }

        request.send().await.map_err(|e| format!("reqwest error: {}", e))
    }

    async fn parse_response<T: serde::de::DeserializeOwned>(
        &self,
        resp: reqwest::Response,
    ) -> Result<T, String> {
        let status = resp.status();
        let text = resp.text().await.map_err(|e| format!("failed to read response: {}", e))?;

        if !status.is_success() {
            if let Ok(err) = serde_json::from_str::<HubSpotError>(&text) {
                return Err(format!(
                    "HubSpot API error ({}): {}",
                    status,
                    err.message.unwrap_or_else(|| "unknown error".to_string())
                ));
            }
            return Err(format!("HubSpot API HTTP error {}: {}", status, text));
        }

        serde_json::from_str(&text).map_err(|e| format!("failed to parse response: {}", e))
    }

    pub async fn get_contacts(
        &self,
        limit: u32,
        after: Option<&str>,
    ) -> Result<(Vec<HubSpotContact>, Option<String>), String> {
        let mut path = format!("/crm/v3/objects/contacts?limit={}", limit);
        if let Some(after) = after {
            path.push_str(&format!("&after={}", after));
        }

        let resp = self.send_request(reqwest::Method::GET, &path, None).await?;
        let data: HubSpotListResponse<HubSpotContact> = self.parse_response(resp).await?;
        let next_after = data.paging.and_then(|p| p.next.map(|n| n.after));
        Ok((data.results, next_after))
    }

    pub async fn create_contact(
        &self,
        email: &str,
        first_name: &str,
        last_name: &str,
        company: &str,
        phone: &str,
    ) -> Result<HubSpotContact, String> {
        let body = serde_json::json!({
            "properties": {
                "email": email,
                "firstname": first_name,
                "lastname": last_name,
                "company": company,
                "phone": phone
            }
        });

        let resp = self
            .send_request(reqwest::Method::POST, "/crm/v3/objects/contacts", Some(body))
            .await?;
        self.parse_response(resp).await
    }

    pub async fn update_contact(
        &self,
        contact_id: &str,
        properties: &serde_json::Value,
    ) -> Result<(), String> {
        let body = serde_json::json!({
            "properties": properties
        });

        let path = format!("/crm/v3/objects/contacts/{}", contact_id);
        let resp = self
            .send_request(reqwest::Method::PATCH, &path, Some(body))
            .await?;
        let status = resp.status();
        if !status.is_success() {
            let text = resp.text().await.unwrap_or_default();
            if let Ok(err) = serde_json::from_str::<HubSpotError>(&text) {
                return Err(format!(
                    "HubSpot API error ({}): {}",
                    status,
                    err.message.unwrap_or_else(|| "unknown error".to_string())
                ));
            }
            return Err(format!("HubSpot API HTTP error {}: {}", status, text));
        }
        Ok(())
    }

    pub async fn get_deals(
        &self,
        limit: u32,
        stage: Option<&str>,
    ) -> Result<Vec<HubSpotDeal>, String> {
        let mut path = format!("/crm/v3/objects/deals?limit={}", limit);
        if let Some(stage) = stage {
            path.push_str(&format!("&properties=dealstage&filterGroups=[{{\"filters\":[{{\"propertyName\":\"dealstage\",\"operator\":\"EQ\",\"value\":\"{}\"}}]}}]", stage));
        }

        let resp = self.send_request(reqwest::Method::GET, &path, None).await?;
        let data: HubSpotListResponse<HubSpotDeal> = self.parse_response(resp).await?;
        Ok(data.results)
    }

    pub async fn create_deal(
        &self,
        name: &str,
        stage: &str,
        amount: Option<f64>,
        close_date: Option<&str>,
    ) -> Result<HubSpotDeal, String> {
        let mut properties = serde_json::json!({
            "dealname": name,
            "dealstage": stage,
        });

        if let Some(amount) = amount {
            properties["amount"] = serde_json::json!(amount);
        }
        if let Some(close_date) = close_date {
            properties["closedate"] = serde_json::json!(close_date);
        }

        let body = serde_json::json!({
            "properties": properties
        });

        let resp = self
            .send_request(reqwest::Method::POST, "/crm/v3/objects/deals", Some(body))
            .await?;
        self.parse_response(resp).await
    }

    pub async fn update_deal(
        &self,
        deal_id: &str,
        properties: &serde_json::Value,
    ) -> Result<(), String> {
        let body = serde_json::json!({
            "properties": properties
        });

        let path = format!("/crm/v3/objects/deals/{}", deal_id);
        let resp = self
            .send_request(reqwest::Method::PATCH, &path, Some(body))
            .await?;
        let status = resp.status();
        if !status.is_success() {
            let text = resp.text().await.unwrap_or_default();
            if let Ok(err) = serde_json::from_str::<HubSpotError>(&text) {
                return Err(format!(
                    "HubSpot API error ({}): {}",
                    status,
                    err.message.unwrap_or_else(|| "unknown error".to_string())
                ));
            }
            return Err(format!("HubSpot API HTTP error {}: {}", status, text));
        }
        Ok(())
    }

    pub async fn get_companies(&self, limit: u32) -> Result<Vec<HubSpotCompany>, String> {
        let path = format!("/crm/v3/objects/companies?limit={}", limit);
        let resp = self.send_request(reqwest::Method::GET, &path, None).await?;
        let data: HubSpotListResponse<HubSpotCompany> = self.parse_response(resp).await?;
        Ok(data.results)
    }

    pub async fn create_company(
        &self,
        name: &str,
        domain: &str,
    ) -> Result<HubSpotCompany, String> {
        let body = serde_json::json!({
            "properties": {
                "name": name,
                "domain": domain
            }
        });

        let resp = self
            .send_request(reqwest::Method::POST, "/crm/v3/objects/companies", Some(body))
            .await?;
        self.parse_response(resp).await
    }

    pub async fn search_contacts(
        &self,
        query: &str,
        limit: u32,
    ) -> Result<Vec<HubSpotContact>, String> {
        let body = serde_json::json!({
            "filterGroups": [{
                "filters": [{
                    "propertyName": "email",
                    "operator": "CONTAINS_TOKEN",
                    "value": query
                }]
            }],
            "limit": limit
        });

        let resp = self
            .send_request(
                reqwest::Method::POST,
                "/crm/v3/objects/contacts/search",
                Some(body),
            )
            .await?;
        let data: HubSpotSearchResponse<HubSpotContact> = self.parse_response(resp).await?;
        Ok(data.results)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hubspot_client_new() {
        let client = HubSpotClient::new("test-access-token".to_string());
        assert_eq!(client.access_token, "test-access-token");
    }

    #[test]
    fn test_hubspot_contact_deserialize() {
        let json = serde_json::json!({
            "id": "123",
            "properties": {
                "email": "test@example.com",
                "firstname": "John",
                "lastname": "Doe"
            },
            "created_at": "2024-01-01T00:00:00Z",
            "updated_at": "2024-01-02T00:00:00Z",
            "archived": false
        });
        let contact: HubSpotContact = serde_json::from_value(json).unwrap();
        assert_eq!(contact.id, "123");
        assert_eq!(
            contact.properties["email"].as_str().unwrap(),
            "test@example.com"
        );
    }

    #[test]
    fn test_hubspot_deal_deserialize() {
        let json = serde_json::json!({
            "id": "456",
            "properties": {
                "dealname": "Big Deal",
                "dealstage": "closedwon",
                "amount": "50000"
            },
            "created_at": "2024-01-01T00:00:00Z",
            "updated_at": "2024-01-02T00:00:00Z",
            "archived": false
        });
        let deal: HubSpotDeal = serde_json::from_value(json).unwrap();
        assert_eq!(deal.id, "456");
        assert_eq!(deal.properties["dealname"].as_str().unwrap(), "Big Deal");
    }

    #[test]
    fn test_hubspot_company_deserialize() {
        let json = serde_json::json!({
            "id": "789",
            "properties": {
                "name": "Acme Corp",
                "domain": "acme.com"
            },
            "created_at": "2024-01-01T00:00:00Z",
            "updated_at": "2024-01-02T00:00:00Z",
            "archived": false
        });
        let company: HubSpotCompany = serde_json::from_value(json).unwrap();
        assert_eq!(company.id, "789");
        assert_eq!(company.properties["name"].as_str().unwrap(), "Acme Corp");
    }

    #[tokio::test]
    async fn test_get_contacts_list_response() {
        let resp = serde_json::json!({
            "results": [
                {
                    "id": "123",
                    "properties": { "email": "test@example.com" },
                    "created_at": "2024-01-01T00:00:00Z",
                    "updated_at": "2024-01-01T00:00:00Z",
                    "archived": false
                }
            ],
            "paging": {
                "next": {
                    "after": "cursor123",
                    "link": "https://api.hubapi.com/crm/v3/objects/contacts?after=cursor123"
                }
            }
        })
        .to_string();

        let data: HubSpotListResponse<HubSpotContact> = serde_json::from_str(&resp).unwrap();
        assert_eq!(data.results.len(), 1);
        assert_eq!(data.results[0].id, "123");
        assert_eq!(
            data.paging.unwrap().next.unwrap().after,
            "cursor123"
        );
    }

    #[tokio::test]
    async fn test_create_contact_response() {
        let resp = serde_json::json!({
            "id": "456",
            "properties": {
                "email": "new@example.com",
                "firstname": "Jane",
                "lastname": "Smith"
            },
            "created_at": "2024-01-01T00:00:00Z",
            "updated_at": "2024-01-01T00:00:00Z",
            "archived": false
        })
        .to_string();

        let contact: HubSpotContact = serde_json::from_str(&resp).unwrap();
        assert_eq!(contact.id, "456");
        assert_eq!(
            contact.properties["email"].as_str().unwrap(),
            "new@example.com"
        );
    }

    #[test]
    fn test_hubspot_error_deserialize() {
        let json = serde_json::json!({
            "status": "error",
            "message": "Invalid input"
        });
        let err: HubSpotError = serde_json::from_value(json).unwrap();
        assert_eq!(err.status.unwrap(), "error");
        assert_eq!(err.message.unwrap(), "Invalid input");
    }

    #[test]
    fn test_hubspot_search_response() {
        let json = serde_json::json!({
            "total": 2,
            "results": [
                {
                    "id": "1",
                    "properties": { "email": "a@test.com" },
                    "archived": false
                },
                {
                    "id": "2",
                    "properties": { "email": "b@test.com" },
                    "archived": false
                }
            ]
        });
        let data: HubSpotSearchResponse<HubSpotContact> = serde_json::from_value(json).unwrap();
        assert_eq!(data.total, 2);
        assert_eq!(data.results.len(), 2);
    }
}
