use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum BotType {
    Webhook = 0,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentBot {
    pub id: i64,
    pub name: String,
    pub description: Option<String>,
    pub bot_type: BotType,
    pub outgoing_url: Option<String>,
    pub account_id: Option<i64>,
    pub avatar_url: Option<String>,
    pub secret: Option<String>,
    pub bot_config: Option<serde_json::Value>,
}

impl AgentBot {
    pub fn new(id: i64, name: String) -> Self {
        Self {
            id,
            name,
            description: None,
            bot_type: BotType::Webhook,
            outgoing_url: None,
            account_id: None,
            avatar_url: None,
            secret: None,
            bot_config: None,
        }
    }

    pub fn available_name(&self) -> String {
        self.name.clone()
    }

    pub fn push_event_data(&self, inbox_avatar_url: Option<&str>) -> serde_json::Value {
        serde_json::json!({
            "id": self.id,
            "name": self.name,
            "avatar_url": self.avatar_url.as_deref().or(inbox_avatar_url),
            "type": "agent_bot"
        })
    }

    pub fn webhook_data(&self) -> serde_json::Value {
        serde_json::json!({
            "id": self.id,
            "name": self.name,
            "type": "agent_bot"
        })
    }

    pub fn system_bot(&self) -> bool {
        self.account_id.is_none()
    }
}
