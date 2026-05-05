use std::sync::RwLock;
use crate::ohc::orchestration::*;
use chrono::Utc;

pub struct IntegrationCredentials {
    pub bot_token: String,
    pub chat_id: String,
    pub webhook_url: String,
    pub api_token: String,
}

pub struct IntegrationsRegistry {
    messages: RwLock<std::collections::HashMap<String, Vec<ChatMessage>>>,
    instances: RwLock<std::collections::HashMap<String, IntegrationInstance>>,
    pull_requests: RwLock<std::collections::HashMap<String, Vec<PullRequest>>>,
    issues: RwLock<std::collections::HashMap<String, Vec<Issue>>>,
    credentials: RwLock<std::collections::HashMap<String, IntegrationCredentials>>,
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
        }
    }

    // Chat methods
    pub fn test_connection(&self, integration_id: &str, _creds: ChatTestRequest) -> Result<(), String> {
        if integration_id.is_empty() {
            return Err("integrationId is required".to_string());
        }
        Ok(())
    }

    pub fn chat_messages(&self, integration_id: &str) -> Vec<ChatMessage> {
        let msgs = self.messages.read().unwrap();
        msgs.get(integration_id).cloned().unwrap_or_default()
    }

    pub fn send_chat_message(&self, integration_id: &str, channel: &str, from_agent: &str, content: &str, thread_id: &str) -> Result<ChatMessage, String> {
        let msg = ChatMessage {
            id: format!("msg-{}", Utc::now().timestamp()),
            channel: channel.to_string(),
            from_agent: from_agent.to_string(),
            content: content.to_string(),
            thread_id: thread_id.to_string(),
            timestamp_unix: Utc::now().timestamp(),
        };

        let mut msgs = self.messages.write().unwrap();
        msgs.entry(integration_id.to_string()).or_insert_with(Vec::new).push(msg.clone());

        // Attempt real delivery
        let creds_map = self.credentials.read().unwrap();
        if let Some(creds) = creds_map.get(integration_id) {
             let text = format!("[{}] {}", from_agent, content);
             match integration_id {
                 "manychat" => {
                     println!("Mocking Manychat API call for msg: {}", text);
                 }
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
                     println!("Mocking Twilio SMS API call to {}: {}", channel, text);
                 }
                 "mailchimp" => {
                     println!("Mocking Mailchimp API call to list {}: {}", channel, text);
                 }
                 _ => {}
             }
        }

        Ok(msg)
    }

    // Integration methods
    pub fn instances(&self) -> Vec<IntegrationInstance> {
        let insts = self.instances.read().unwrap();
        insts.values().cloned().collect()
    }

    pub fn instances_by_category(&self, category: &str) -> Vec<IntegrationInstance> {
        let insts = self.instances.read().unwrap();
        insts.values().filter(|i| i.category == category).cloned().collect()
    }

    pub fn connect(&self, integration_id: &str, base_url: &str, creds: ConnectIntegrationRequest) -> Result<IntegrationInstance, String> {
        let mut insts = self.instances.write().unwrap();
        let inst = IntegrationInstance {
            id: integration_id.to_string(),
            name: integration_id.to_string(),
            category: "default".to_string(),
            status: "connected".to_string(),
            base_url: base_url.to_string(),
        };
        insts.insert(integration_id.to_string(), inst.clone());

        let mut credentials = self.credentials.write().unwrap();
        credentials.insert(integration_id.to_string(), IntegrationCredentials {
            bot_token: creds.bot_token,
            chat_id: creds.chat_id,
            webhook_url: creds.webhook_url,
            api_token: creds.api_token,
        });

        Ok(inst)
    }

    pub fn disconnect(&self, integration_id: &str) -> Result<IntegrationInstance, String> {
        let mut insts = self.instances.write().unwrap();
        if let Some(inst) = insts.get_mut(integration_id) {
            inst.status = "disconnected".to_string();
            return Ok(inst.clone());
        }
        Err("integration not found".to_string())
    }

    pub fn pull_requests(&self, integration_id: &str) -> Vec<PullRequest> {
        let prs = self.pull_requests.read().unwrap();
        prs.get(integration_id).cloned().unwrap_or_default()
    }

    pub fn create_pull_request(&self, integration_id: &str, _repository: &str, title: &str, body: &str, source_branch: &str, target_branch: &str, created_by: &str) -> Result<PullRequest, String> {
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

        let mut prs = self.pull_requests.write().unwrap();
        prs.entry(integration_id.to_string()).or_insert_with(Vec::new).push(pr.clone());

        Ok(pr)
    }

    pub fn merge_pull_request(&self, pr_id: &str) -> Result<PullRequest, String> {
        let mut prs = self.pull_requests.write().unwrap();
        for v in prs.values_mut() {
            if let Some(pr) = v.iter_mut().find(|p| p.id == pr_id) {
                pr.status = "merged".to_string();
                return Ok(pr.clone());
            }
        }
        Err("pr not found".to_string())
    }

    pub fn close_pull_request(&self, pr_id: &str) -> Result<PullRequest, String> {
        let mut prs = self.pull_requests.write().unwrap();
        for v in prs.values_mut() {
            if let Some(pr) = v.iter_mut().find(|p| p.id == pr_id) {
                pr.status = "closed".to_string();
                return Ok(pr.clone());
            }
        }
        Err("pr not found".to_string())
    }

    pub fn issues(&self, integration_id: &str) -> Vec<Issue> {
        let issues = self.issues.read().unwrap();
        issues.get(integration_id).cloned().unwrap_or_default()
    }

    pub fn create_booking(&self, _integration_id: &str, title: &str, _time: &str) -> Result<(), String> {
        println!("Mocking Calendly/Zoom booking for: {}", title);
        Ok(())
    }

    pub fn create_payment(&self, _integration_id: &str, amount: f64) -> Result<(), String> {
        println!("Mocking Mercado Pago payment for: {}", amount);
        Ok(())
    }

    pub fn create_shipping_label(&self, _integration_id: &str, address: &str) -> Result<(), String> {
        println!("Mocking Shippo shipping label for: {}", address);
        Ok(())
    }

    pub fn create_issue(&self, integration_id: &str, _project: &str, title: &str, description: &str, created_by: &str, priority: &str, labels: Vec<String>) -> Result<Issue, String> {
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

        let mut issues = self.issues.write().unwrap();
        issues.entry(integration_id.to_string()).or_insert_with(Vec::new).push(issue.clone());

        Ok(issue)
    }

    pub fn update_issue_status(&self, issue_id: &str, status: &str) -> Result<Issue, String> {
        let mut issues = self.issues.write().unwrap();
        for v in issues.values_mut() {
            if let Some(issue) = v.iter_mut().find(|i| i.id == issue_id) {
                issue.status = status.to_string();
                return Ok(issue.clone());
            }
        }
        Err("issue not found".to_string())
    }

    pub fn assign_issue(&self, issue_id: &str, assignee: &str) -> Result<Issue, String> {
        let mut issues = self.issues.write().unwrap();
        for v in issues.values_mut() {
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
        println!("Failed to send Telegram message: {}", e);
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
        println!("Failed to send Discord webhook: {}", e);
    }
}
