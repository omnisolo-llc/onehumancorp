use reqwest::Client;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MailchimpList {
    pub id: String,
    pub name: String,
    pub member_count: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MailchimpCampaign {
    pub id: String,
    pub title: String,
    pub status: String,
    pub send_time: Option<String>,
}

pub struct MailchimpClient {
    pub api_key: String,
    http_client: Client,
}

impl MailchimpClient {
    pub fn new(api_key: String) -> Self {
        MailchimpClient {
            api_key,
            http_client: Client::new(),
        }
    }

    fn base_url(&self) -> Result<String, String> {
        let dc = self.api_key.rsplit('-').next().ok_or_else(|| {
            "Invalid Mailchimp API key: cannot extract data center suffix".to_string()
        })?;
        if dc.is_empty() || dc == self.api_key {
            return Err("Invalid Mailchimp API key format".to_string());
        }
        Ok(format!("https://{}.api.mailchimp.com/3.0", dc))
    }

    async fn post(&self, path: &str, body: serde_json::Value) -> Result<serde_json::Value, String> {
        let url = format!("{}{}", self.base_url()?, path);
        let res = self
            .http_client
            .post(&url)
            .basic_auth("anystring", Some(&self.api_key))
            .json(&body)
            .send()
            .await
            .map_err(|e| format!("Mailchimp request failed: {}", e))?;

        if !res.status().is_success() {
            let status = res.status();
            let text = res.text().await.unwrap_or_default();
            return Err(format!("Mailchimp API error ({}): {}", status, text));
        }

        res.json::<serde_json::Value>()
            .await
            .map_err(|e| format!("Mailchimp response parse error: {}", e))
    }

    async fn get(&self, path: &str) -> Result<serde_json::Value, String> {
        let url = format!("{}{}", self.base_url()?, path);
        let res = self
            .http_client
            .get(&url)
            .basic_auth("anystring", Some(&self.api_key))
            .send()
            .await
            .map_err(|e| format!("Mailchimp request failed: {}", e))?;

        if !res.status().is_success() {
            let status = res.status();
            let text = res.text().await.unwrap_or_default();
            return Err(format!("Mailchimp API error ({}): {}", status, text));
        }

        res.json::<serde_json::Value>()
            .await
            .map_err(|e| format!("Mailchimp response parse error: {}", e))
    }

    pub async fn sync_customer(
        &self,
        email: &str,
        _tag: &str,
    ) -> Result<(), String> {
        let payload = serde_json::json!({
            "email_address": email,
            "status": "subscribed",
        });
        self.post("/lists/members", payload).await?;
        Ok(())
    }

    pub async fn send_campaign(
        &self,
        _audience: &str,
        _body: &str,
    ) -> Result<(), String> {
        let payload = serde_json::json!({
            "type": "regular",
            "recipients": { "list_id": _audience },
            "settings": {
                "subject_line": "Campaign",
                "from_name": "Sender",
                "reply_to": "noreply@example.com"
            },
            "content": {
                "html": _body
            }
        });

        let res = self.post("/campaigns", payload).await?;
        let campaign_id = res["id"]
            .as_str()
            .ok_or_else(|| "Missing campaign id in response".to_string())?;

        self.post(&format!("/campaigns/{}/actions/send", campaign_id), serde_json::json!({}))
            .await?;

        Ok(())
    }

    pub async fn get_lists(&self) -> Result<Vec<MailchimpList>, String> {
        let res = self.get("/lists").await?;
        let lists = res["lists"]
            .as_array()
            .ok_or_else(|| "Missing lists array in response".to_string())?
            .iter()
            .filter_map(|item| {
                Some(MailchimpList {
                    id: item["id"].as_str()?.to_string(),
                    name: item["name"].as_str()?.to_string(),
                    member_count: item["member_count"].as_u64()? as u32,
                })
            })
            .collect();
        Ok(lists)
    }

    pub async fn add_subscriber(
        &self,
        list_id: &str,
        email: &str,
        status: &str,
    ) -> Result<(), String> {
        let payload = serde_json::json!({
            "email_address": email,
            "status": status,
        });
        self.post(&format!("/lists/{}/members", list_id), payload)
            .await?;
        Ok(())
    }

    pub async fn get_campaigns(
        &self,
        limit: u32,
    ) -> Result<Vec<MailchimpCampaign>, String> {
        let res = self
            .get(&format!("/campaigns?count={}", limit))
            .await?;
        let campaigns = res["campaigns"]
            .as_array()
            .ok_or_else(|| "Missing campaigns array in response".to_string())?
            .iter()
            .filter_map(|item| {
                Some(MailchimpCampaign {
                    id: item["id"].as_str()?.to_string(),
                    title: item["settings"]["title"]
                        .as_str()
                        .unwrap_or("")
                        .to_string(),
                    status: item["status"].as_str()?.to_string(),
                    send_time: item["send_time"].as_str().map(|s| s.to_string()),
                })
            })
            .collect();
        Ok(campaigns)
    }
}
