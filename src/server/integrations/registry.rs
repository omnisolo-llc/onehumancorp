use std::sync::RwLock;
use crate::ohc::orchestration::*;
use chrono::Utc;

pub struct IntegrationCredentials {
    pub bot_token: String,
    pub chat_id: String,
    pub webhook_url: String,
    pub api_token: String,
    pub from_phone: String,
}

pub struct IntegrationsRegistry {
    messages: RwLock<std::collections::HashMap<String, Vec<ChatMessage>>>,
    instances: RwLock<std::collections::HashMap<String, IntegrationInstance>>,
    pull_requests: RwLock<std::collections::HashMap<String, Vec<PullRequest>>>,
    issues: RwLock<std::collections::HashMap<String, Vec<Issue>>>,
    credentials: RwLock<std::collections::HashMap<String, IntegrationCredentials>>,
    twilio_clients: std::sync::RwLock<std::collections::HashMap<String, std::sync::Arc<crate::integrations::twilio::provider::TwilioProvider>>>,
    nats_clients: std::sync::Arc<std::sync::RwLock<std::collections::HashMap<String, std::sync::Arc<crate::integrations::nats::provider::NatsProvider>>>>,
}

impl IntegrationsRegistry {
    pub fn new() -> Self {
        let mut instances = std::collections::HashMap::new();
        for provider in crate::integrations::catalog::get_catalog() {
            let id = provider.metadata.id.clone();
            instances.insert(id.clone(), IntegrationInstance {
                id: id.clone(),
                name: provider.metadata.name.clone(),
                category: provider.metadata.category.clone(),
                status: "disconnected".to_string(),
                base_url: provider.metadata.base_url.clone(),
            });
        }

        IntegrationsRegistry {
            messages: RwLock::new(std::collections::HashMap::new()),
            instances: RwLock::new(instances),
            pull_requests: RwLock::new(std::collections::HashMap::new()),
            issues: RwLock::new(std::collections::HashMap::new()),
            credentials: RwLock::new(std::collections::HashMap::new()),
            twilio_clients: std::sync::RwLock::new(std::collections::HashMap::new()),
            nats_clients: std::sync::Arc::new(std::sync::RwLock::new(std::collections::HashMap::new())),
        }
    }

    // Chat methods
    pub fn test_connection(&self, integration_id: &str, _creds: ChatTestRequest) -> Result<(), String> {
        if integration_id.is_empty() {
            return Err("integrationId is required".to_string());
        }
        Ok(())
    }

    pub fn chat_messages(&self, tenant_id: &str, integration_id: &str) -> Vec<ChatMessage> {
        let composite_key = format!("{}::{}", tenant_id, integration_id);
        let msgs = self.messages.read().unwrap();
        msgs.get(&composite_key).cloned().unwrap_or_default()
    }

    pub fn send_chat_message(&self, tenant_id: &str, integration_id: &str, channel: &str, from_agent: &str, content: &str, thread_id: &str) -> Result<ChatMessage, String> {
        let composite_key = format!("{}::{}", tenant_id, integration_id);
        let msg = ChatMessage {
            id: format!("msg-{}", Utc::now().timestamp()),
            channel: channel.to_string(),
            from_agent: from_agent.to_string(),
            content: content.to_string(),
            thread_id: thread_id.to_string(),
            timestamp_unix: Utc::now().timestamp(),
        };

        let mut msgs = self.messages.write().unwrap();
        msgs.entry(composite_key.clone()).or_insert_with(Vec::new).push(msg.clone());

        // Attempt real delivery
        let creds_map = self.credentials.read().unwrap();
        if let Some(creds) = creds_map.get(&composite_key) {
             let text = format!("[{}] {}", from_agent, content);
             match integration_id {
                 "telegram" => {
                     if !creds.bot_token.is_empty() {
                         let chat_id = if !creds.chat_id.is_empty() { creds.chat_id.clone() } else { channel.to_string() };
                         tokio::spawn(send_telegram_message(creds.bot_token.clone(), chat_id, text));
                     }
                 }
                 "discord" => {
                     if !creds.webhook_url.is_empty() {
                          tokio::spawn(send_discord_webhook(creds.webhook_url.clone(), from_agent.to_string(), content.to_string()));
                     }
                 }
                 "twilio" => {
                     if !creds.from_phone.is_empty() {
                         let to = if !creds.chat_id.is_empty() { creds.chat_id.clone() } else { channel.to_string() };
                         let from = creds.from_phone.clone();
                         let text = content.to_string();

                         let clients = self.twilio_clients.read().unwrap();
                         if let Some(client) = clients.get(&composite_key) {
                             let client = client.clone();
                             tokio::spawn(async move {
                                 if let Err(e) = client.send_sms(&to, &from, &text).await {
                                     tracing::error!("Failed to send Twilio SMS: {}", e);
                                 }
                             });
                         }
                     }
                 }
                 _ => {}
             }
        }

        Ok(msg)
    }

    // Integration methods
    pub fn instances(&self, tenant_id: &str) -> Vec<IntegrationInstance> {
        let insts = self.instances.read().unwrap();
        insts.iter().filter(|(k, _)| k.starts_with(&format!("{}::", tenant_id))).map(|(_, v)| v.clone()).collect()
    }

    pub fn instances_by_category(&self, tenant_id: &str, category: &str) -> Vec<IntegrationInstance> {
        let insts = self.instances.read().unwrap();
        insts.iter().filter(|(k, v)| k.starts_with(&format!("{}::", tenant_id)) && v.category == category).map(|(_, v)| v.clone()).collect()
    }

    pub fn connect(&self, tenant_id: &str, integration_id: &str, base_url: &str, creds: ConnectIntegrationRequest) -> Result<IntegrationInstance, String> {
        let composite_key = format!("{}::{}", tenant_id, integration_id);

        let mut insts = self.instances.write().unwrap();
        let inst = IntegrationInstance {
            id: integration_id.to_string(),
            name: integration_id.to_string(),
            category: "default".to_string(),
            status: "connected".to_string(),
            base_url: base_url.to_string(),
        };
        insts.insert(composite_key.clone(), inst.clone());

        let mut credentials = self.credentials.write().unwrap();
        credentials.insert(composite_key.clone(), IntegrationCredentials {
            bot_token: creds.bot_token.clone(),
            chat_id: creds.chat_id.clone(),
            webhook_url: creds.webhook_url.clone(),
            api_token: creds.api_token.clone(),
            from_phone: creds.from_phone.clone(),
        });
        if integration_id == "twilio" {
            let mut clients = self.twilio_clients.write().unwrap();
            clients.insert(composite_key.clone(), std::sync::Arc::new(crate::integrations::twilio::provider::TwilioProvider::new(creds.bot_token.clone(), creds.api_token.clone())));
        }
        if integration_id == "nats" {
            let base_url_clone = base_url.to_string();
            let nats_clients = std::sync::Arc::clone(&self.nats_clients);
            let composite_key_clone = composite_key.clone();
            tokio::spawn(async move {
                if let Ok(provider) = crate::integrations::nats::provider::NatsProvider::new(&base_url_clone).await {
                    let mut clients = nats_clients.write().unwrap();
                    clients.insert(composite_key_clone, std::sync::Arc::new(provider));
                }
            });
        }

        Ok(inst)
    }

    pub fn disconnect(&self, tenant_id: &str, integration_id: &str) -> Result<IntegrationInstance, String> {
        let composite_key = format!("{}::{}", tenant_id, integration_id);
        let mut insts = self.instances.write().unwrap();
        if let Some(inst) = insts.get_mut(&composite_key) {
            inst.status = "disconnected".to_string();
            return Ok(inst.clone());
        }
        Err("integration not found".to_string())
    }

    pub fn pull_requests(&self, tenant_id: &str, integration_id: &str) -> Vec<PullRequest> {
        let composite_key = format!("{}::{}", tenant_id, integration_id);
        let prs = self.pull_requests.read().unwrap();
        prs.get(&composite_key).cloned().unwrap_or_default()
    }

    pub fn create_pull_request(&self, tenant_id: &str, integration_id: &str, _repository: &str, title: &str, body: &str, source_branch: &str, target_branch: &str, created_by: &str) -> Result<PullRequest, String> {
        let pr = PullRequest {
            id: format!("pr-{}", Utc::now().timestamp()),
            title: title.to_string(),
            body: body.to_string(),
            source_branch: source_branch.to_string(),
            target_branch: target_branch.to_string(),
            status: "open".to_string(),
            created_by: created_by.to_string(),
            created_at_unix: Utc::now().timestamp(),
        };

        let composite_key = format!("{}::{}", tenant_id, integration_id);
        let mut prs = self.pull_requests.write().unwrap();
        prs.entry(composite_key).or_insert_with(Vec::new).push(pr.clone());

        Ok(pr)
    }

    pub fn merge_pull_request(&self, tenant_id: &str, pr_id: &str) -> Result<PullRequest, String> {
        let mut prs = self.pull_requests.write().unwrap();
        for (k, v) in prs.iter_mut() {
            if !k.starts_with(&format!("{}::", tenant_id)) { continue; }
            if let Some(pr) = v.iter_mut().find(|p| p.id == pr_id) {
                pr.status = "merged".to_string();
                return Ok(pr.clone());
            }
        }
        Err("pr not found".to_string())
    }

    pub fn close_pull_request(&self, tenant_id: &str, pr_id: &str) -> Result<PullRequest, String> {
        let mut prs = self.pull_requests.write().unwrap();
        for (k, v) in prs.iter_mut() {
            if !k.starts_with(&format!("{}::", tenant_id)) { continue; }
            if let Some(pr) = v.iter_mut().find(|p| p.id == pr_id) {
                pr.status = "closed".to_string();
                return Ok(pr.clone());
            }
        }
        Err("pr not found".to_string())
    }

    pub fn issues(&self, tenant_id: &str, integration_id: &str) -> Vec<Issue> {
        let composite_key = format!("{}::{}", tenant_id, integration_id);
        let issues = self.issues.read().unwrap();
        issues.get(&composite_key).cloned().unwrap_or_default()
    }

    pub fn create_issue(&self, tenant_id: &str, integration_id: &str, _project: &str, title: &str, description: &str, created_by: &str, priority: &str, labels: Vec<String>) -> Result<Issue, String> {
        let issue = Issue {
            id: format!("issue-{}", Utc::now().timestamp()),
            title: title.to_string(),
            description: description.to_string(),
            status: "open".to_string(),
            priority: priority.to_string(),
            labels,
            assigned_agent: String::new(),
            created_by: created_by.to_string(),
            created_at_unix: Utc::now().timestamp(),
        };

        let composite_key = format!("{}::{}", tenant_id, integration_id);
        let mut issues = self.issues.write().unwrap();
        issues.entry(composite_key).or_insert_with(Vec::new).push(issue.clone());

        Ok(issue)
    }

    pub fn update_issue_status(&self, tenant_id: &str, issue_id: &str, status: &str) -> Result<Issue, String> {
        let mut issues = self.issues.write().unwrap();
        for (k, v) in issues.iter_mut() {
            if !k.starts_with(&format!("{}::", tenant_id)) { continue; }
            if let Some(issue) = v.iter_mut().find(|i| i.id == issue_id) {
                issue.status = status.to_string();
                return Ok(issue.clone());
            }
        }
        Err("issue not found".to_string())
    }

    pub fn assign_issue(&self, tenant_id: &str, issue_id: &str, assignee: &str) -> Result<Issue, String> {
        let mut issues = self.issues.write().unwrap();
        for (k, v) in issues.iter_mut() {
            if !k.starts_with(&format!("{}::", tenant_id)) { continue; }
            if let Some(issue) = v.iter_mut().find(|i| i.id == issue_id) {
                issue.assigned_agent = assignee.to_string();
                return Ok(issue.clone());
            }
        }
        Err("issue not found".to_string())
    }
}

async fn send_telegram_message(bot_token: String, chat_id: String, text: String) {
    let url = format!("https://api.telegram.org/bot{}/sendMessage", bot_token);
    let client = reqwest::Client::new();
    let res = client.post(&url)
        .json(&serde_json::json!({
            "chat_id": chat_id,
            "text": text,
        }))
        .send()
        .await;
    
    if let Err(e) = res {
        tracing::error!("Failed to send Telegram message: {}", e);
    }
}

async fn send_discord_webhook(webhook_url: String, username: String, content: String) {
    let client = reqwest::Client::new();
    let res = client.post(&webhook_url)
        .json(&serde_json::json!({
            "username": username,
            "content": content,
        }))
        .send()
        .await;

    if let Err(e) = res {
        tracing::error!("Failed to send Discord webhook: {}", e);
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[tokio::test]
    async fn test_twilio_integration() {
        let registry = IntegrationsRegistry::new();
        let creds = crate::ohc::orchestration::ConnectIntegrationRequest {
            integration_id: "twilio".to_string(),
            base_url: "https://api.twilio.com".to_string(),
            bot_token: "test_sid".to_string(),
            chat_id: "".to_string(),
            webhook_url: "".to_string(),
            api_token: "test_token".to_string(),
            from_phone: "+1234567890".to_string(),
        };
        registry.connect("tenant1", "twilio", "https://api.twilio.com", creds).unwrap();

        let msg = registry.send_chat_message("tenant1", "twilio", "+0987654321", "agent1", "Hello World", "thread1").unwrap();
        assert_eq!(msg.content, "Hello World");

    }
}
