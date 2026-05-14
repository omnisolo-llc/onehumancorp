use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct TooltipEntry {
    pub key: String,
    pub plain_language_text: String,
    pub help_article_url: Option<String>,
}

pub struct TooltipRegistry {
    entries: Arc<RwLock<HashMap<String, TooltipEntry>>>,
}

impl TooltipRegistry {
    pub fn new() -> Self {
        let mut initial_entries = HashMap::new();
        initial_entries.insert(
            "marketing.campaign.start_button".to_string(),
            TooltipEntry {
                key: "marketing.campaign.start_button".to_string(),
                plain_language_text: "Click here to start a new email campaign to your customers.".to_string(),
                help_article_url: Some("/help/marketing".to_string()),
            }
        );

        Self {
            entries: Arc::new(RwLock::new(initial_entries)),
        }
    }

    pub async fn get_tooltip(&self, key: &str) -> Option<TooltipEntry> {
        let entries = self.entries.read().await;
        entries.get(key).cloned()
    }
}
