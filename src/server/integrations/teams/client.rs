use reqwest::Client;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MicrosoftTeam {
    pub id: String,
    pub display_name: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MicrosoftChannel {
    pub id: String,
    pub display_name: String,
}

pub struct TeamsClient {
    pub access_token: String,
    http_client: Client,
}

impl TeamsClient {
    pub fn new(access_token: String) -> Self {
        TeamsClient {
            access_token,
            http_client: Client::new(),
        }
    }

    async fn post(
        &self,
        url: &str,
        body: serde_json::Value,
    ) -> Result<serde_json::Value, String> {
        let res = self
            .http_client
            .post(url)
            .bearer_auth(&self.access_token)
            .json(&body)
            .send()
            .await
            .map_err(|e| format!("Network error: {}", e))?;

        if !res.status().is_success() {
            let status = res.status();
            let text = res.text().await.unwrap_or_default();
            return Err(format!("Teams API error ({}): {}", status, text));
        }

        res.json::<serde_json::Value>()
            .await
            .map_err(|e| format!("Teams response parse error: {}", e))
    }

    async fn get(&self, url: &str) -> Result<serde_json::Value, String> {
        let res = self
            .http_client
            .get(url)
            .bearer_auth(&self.access_token)
            .send()
            .await
            .map_err(|e| format!("Network error: {}", e))?;

        if !res.status().is_success() {
            let status = res.status();
            let text = res.text().await.unwrap_or_default();
            return Err(format!("Teams API error ({}): {}", status, text));
        }

        res.json::<serde_json::Value>()
            .await
            .map_err(|e| format!("Teams response parse error: {}", e))
    }

    pub async fn create_meeting(
        &self,
        subject: &str,
        start_time: &str,
        end_time: &str,
        attendees: &[String],
    ) -> Result<String, String> {
        let url = "https://graph.microsoft.com/v1.0/me/onlineMeetings".to_string();
        let attendee_list: Vec<serde_json::Value> = attendees
            .iter()
            .map(|a| {
                serde_json::json!({
                    "emailAddress": { "address": a }
                })
            })
            .collect();

        let payload = serde_json::json!({
            "startDateTime": start_time,
            "endDateTime": end_time,
            "subject": subject,
            "attendees": attendee_list
        });

        let res = self.post(&url, payload).await?;
        let join_url = res["joinUrl"]
            .as_str()
            .unwrap_or("")
            .to_string();
        Ok(join_url)
    }

    pub async fn send_channel_message(
        &self,
        team_id: &str,
        channel_id: &str,
        message: &str,
    ) -> Result<(), String> {
        let url = format!(
            "https://graph.microsoft.com/v1.0/teams/{}/channels/{}/messages",
            team_id, channel_id
        );
        let payload = serde_json::json!({
            "body": {
                "contentType": "html",
                "content": message
            }
        });
        self.post(&url, payload).await?;
        Ok(())
    }

    pub async fn get_teams(&self) -> Result<Vec<MicrosoftTeam>, String> {
        let url = "https://graph.microsoft.com/v1.0/me/joinedTeams".to_string();
        let res = self.get(&url).await?;
        let teams = res["value"]
            .as_array()
            .ok_or_else(|| "Missing value array in response".to_string())?
            .iter()
            .filter_map(|item| {
                Some(MicrosoftTeam {
                    id: item["id"].as_str()?.to_string(),
                    display_name: item["displayName"].as_str()?.to_string(),
                })
            })
            .collect();
        Ok(teams)
    }

    pub async fn get_channels(&self, team_id: &str) -> Result<Vec<MicrosoftChannel>, String> {
        let url = format!(
            "https://graph.microsoft.com/v1.0/teams/{}/channels",
            team_id
        );
        let res = self.get(&url).await?;
        let channels = res["value"]
            .as_array()
            .ok_or_else(|| "Missing value array in response".to_string())?
            .iter()
            .filter_map(|item| {
                Some(MicrosoftChannel {
                    id: item["id"].as_str()?.to_string(),
                    display_name: item["displayName"].as_str()?.to_string(),
                })
            })
            .collect();
        Ok(channels)
    }
}
