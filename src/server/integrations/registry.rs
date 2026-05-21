use std::sync::RwLock;
use ::server_ohc::orchestration::*;
use chrono::Utc;

pub struct IntegrationCredentials {
    pub bot_token: String,
    pub chat_id: String,
    pub webhook_url: String,
    pub api_token: String,
    pub from_phone: String,
    pub tenant_id: String,
}

pub struct IntegrationsRegistry {
    messages: RwLock<std::collections::HashMap<String, Vec<ChatMessage>>>,
    instances: RwLock<std::collections::HashMap<String, IntegrationInstance>>,
    pull_requests: RwLock<std::collections::HashMap<String, Vec<PullRequest>>>,
    issues: RwLock<std::collections::HashMap<String, Vec<Issue>>>,
    credentials: RwLock<std::collections::HashMap<String, IntegrationCredentials>>,
    twilio_clients: std::sync::RwLock<std::collections::HashMap<String, std::sync::Arc<crate::integrations::twilio::provider::TwilioProvider>>>,
    nats_clients: std::sync::Arc<std::sync::RwLock<std::collections::HashMap<String, std::sync::Arc<crate::integrations::nats::provider::NatsProvider>>>>,
    meta_clients: std::sync::RwLock<std::collections::HashMap<String, std::sync::Arc<crate::integrations::meta::provider::MetaProvider>>>,
    manychat_clients: std::sync::RwLock<std::collections::HashMap<String, std::sync::Arc<crate::integrations::manychat::provider::ManychatProvider>>>,
    calendly_clients: std::sync::RwLock<std::collections::HashMap<String, std::sync::Arc<crate::integrations::calendly::provider::CalendlyProvider>>>,
    mailchimp_clients: std::sync::RwLock<std::collections::HashMap<String, std::sync::Arc<crate::integrations::mailchimp::provider::MailchimpProvider>>>,
    mercadopago_clients: std::sync::RwLock<std::collections::HashMap<String, std::sync::Arc<crate::integrations::mercadopago::provider::MercadoPagoProvider>>>,
    shippo_clients: std::sync::RwLock<std::collections::HashMap<String, std::sync::Arc<crate::integrations::shippo::provider::ShippoProvider>>>,
    zoom_clients: std::sync::RwLock<std::collections::HashMap<String, std::sync::Arc<crate::integrations::zoom::provider::ZoomProvider>>>,
    cal_com_clients: std::sync::RwLock<std::collections::HashMap<String, std::sync::Arc<crate::integrations::cal_com::provider::CalComProvider>>>,
    google_calendar_clients: std::sync::RwLock<std::collections::HashMap<String, std::sync::Arc<crate::integrations::google_calendar::provider::GoogleCalendarProvider>>>,
    ayrshare_clients: std::sync::RwLock<std::collections::HashMap<String, std::sync::Arc<crate::integrations::ayrshare::provider::AyrshareProvider>>>,
    listmonk_clients: std::sync::RwLock<std::collections::HashMap<String, std::sync::Arc<crate::integrations::listmonk::provider::ListmonkProvider>>>,
    easypost_clients: std::sync::RwLock<std::collections::HashMap<String, std::sync::Arc<crate::integrations::easypost::provider::EasyPostProvider>>>,
    jitsi_clients: std::sync::RwLock<std::collections::HashMap<String, std::sync::Arc<crate::integrations::jitsi::provider::JitsiProvider>>>,
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
            meta_clients: std::sync::RwLock::new(std::collections::HashMap::new()),
            manychat_clients: std::sync::RwLock::new(std::collections::HashMap::new()),
            calendly_clients: std::sync::RwLock::new(std::collections::HashMap::new()),
            mailchimp_clients: std::sync::RwLock::new(std::collections::HashMap::new()),
            mercadopago_clients: std::sync::RwLock::new(std::collections::HashMap::new()),
            shippo_clients: std::sync::RwLock::new(std::collections::HashMap::new()),
            zoom_clients: std::sync::RwLock::new(std::collections::HashMap::new()),
            cal_com_clients: std::sync::RwLock::new(std::collections::HashMap::new()),
            google_calendar_clients: std::sync::RwLock::new(std::collections::HashMap::new()),
            ayrshare_clients: std::sync::RwLock::new(std::collections::HashMap::new()),
            listmonk_clients: std::sync::RwLock::new(std::collections::HashMap::new()),
            easypost_clients: std::sync::RwLock::new(std::collections::HashMap::new()),
            jitsi_clients: std::sync::RwLock::new(std::collections::HashMap::new()),
        }
    }

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

    pub fn send_chat_message(&self, integration_id: &str, to_number: &str, sender_id: &str, content: &str, _thread_id: &str) -> Result<ChatMessage, String> {
        let mut msg = ChatMessage {
            id: format!("msg-{}", Utc::now().timestamp()),
            content: content.to_string(),
            sender_id: sender_id.to_string(),
            to_number: to_number.to_string(),
            status: "sent".to_string(),
            timestamp_unix: Utc::now().timestamp(),
        };

        if integration_id == "twilio" {
            let clients = self.twilio_clients.read().unwrap();
            if let Some(client) = clients.get(integration_id) {
                let to_num = to_number.to_string();
                let msg_content = content.to_string();
                let client_clone = std::sync::Arc::clone(client);

                tokio::spawn(async move {
                    if let Err(e) = client_clone.send_sms(&to_num, &msg_content).await {
                        tracing::error!("Twilio SMS failed: {}", e);
                    }
                });
            } else {
                return Err("twilio client not configured".to_string());
            }
        } else if integration_id == "telegram" {
            let creds = self.credentials.read().unwrap();
            if let Some(c) = creds.get(integration_id) {
                let bot_token = c.bot_token.clone();
                let chat_id = c.chat_id.clone();
                let content_clone = content.to_string();
                tokio::spawn(async move {
                    send_telegram_message(bot_token, chat_id, content_clone).await;
                });
            } else {
                return Err("telegram credentials missing".to_string());
            }
        } else if integration_id == "discord" {
             let creds = self.credentials.read().unwrap();
             if let Some(c) = creds.get(integration_id) {
                 let webhook_url = c.webhook_url.clone();
                 let username = "OHC Bot".to_string();
                 let content_clone = content.to_string();
                 tokio::spawn(async move {
                     send_discord_webhook(webhook_url, username, content_clone).await;
                 });
             } else {
                 return Err("discord credentials missing".to_string());
             }
        }

        let mut msgs = self.messages.write().unwrap();
        msgs.entry(integration_id.to_string()).or_insert_with(Vec::new).push(msg.clone());

        // Simulate an auto-reply for testing
        let reply = ChatMessage {
            id: format!("msg-reply-{}", Utc::now().timestamp()),
            content: format!("Echo: {}", content),
            sender_id: "system".to_string(),
            to_number: "me".to_string(),
            status: "delivered".to_string(),
            timestamp_unix: Utc::now().timestamp() + 1,
        };
        msgs.entry(integration_id.to_string()).or_insert_with(Vec::new).push(reply);

        Ok(msg)
    }

    pub fn instances(&self) -> Vec<IntegrationInstance> {
        let insts = self.instances.read().unwrap();
        insts.values().cloned().collect()
    }

    pub fn connect(&self, integration_id: &str, base_url: &str, creds: ConnectIntegrationRequest) -> Result<IntegrationInstance, String> {
        let mut insts = self.instances.write().unwrap();
        let inst = if let Some(inst) = insts.get_mut(integration_id) {
            inst.status = "connected".to_string();
            inst.clone()
        } else {
            let inst = IntegrationInstance {
                id: integration_id.to_string(),
                name: integration_id.to_string(),
                category: "unknown".to_string(),
                status: "connected".to_string(),
                base_url: base_url.to_string(),
            };
            insts.insert(integration_id.to_string(), inst.clone());
            inst
        };

        let mut c = self.credentials.write().unwrap();
        c.insert(integration_id.to_string(), IntegrationCredentials {
            bot_token: creds.bot_token.clone(),
            chat_id: creds.chat_id.clone(),
            webhook_url: creds.webhook_url.clone(),
            api_token: creds.api_token.clone(),
            from_phone: creds.from_phone.clone(),
            tenant_id: "unknown".to_string(), // Simplified for mock
        });
        if integration_id == "twilio" {
            let mut clients = self.twilio_clients.write().unwrap();
            clients.insert(integration_id.to_string(), std::sync::Arc::new(crate::integrations::twilio::provider::TwilioProvider::new(creds.bot_token.clone(), creds.api_token.clone())));
        }
        if integration_id == "nats" {
            let base_url_clone = base_url.to_string();
            let nats_clients = std::sync::Arc::clone(&self.nats_clients);
            let integration_id_clone = integration_id.to_string();
            tokio::spawn(async move {
                if let Ok(provider) = crate::integrations::nats::provider::NatsProvider::new(&base_url_clone).await {
                    let mut clients = nats_clients.write().unwrap();
                    clients.insert(integration_id_clone, std::sync::Arc::new(provider));
                }
            });
        }
        if integration_id == "meta" {
            let mut clients = self.meta_clients.write().unwrap();
            clients.insert(integration_id.to_string(), std::sync::Arc::new(crate::integrations::meta::provider::MetaProvider::new(
                creds.api_token.clone()
            )));
        }
        if integration_id == "manychat" {
            let mut clients = self.manychat_clients.write().unwrap();
            clients.insert(integration_id.to_string(), std::sync::Arc::new(crate::integrations::manychat::provider::ManychatProvider::new(creds.api_token.clone())));
        }
        if integration_id == "calendly" {
            let mut clients = self.calendly_clients.write().unwrap();
            clients.insert(integration_id.to_string(), std::sync::Arc::new(crate::integrations::calendly::provider::CalendlyProvider::new(creds.api_token.clone())));
        }
        if integration_id == "mailchimp" {
            let mut clients = self.mailchimp_clients.write().unwrap();
            clients.insert(integration_id.to_string(), std::sync::Arc::new(crate::integrations::mailchimp::provider::MailchimpProvider::new(creds.api_token.clone())));
        }
        if integration_id == "mercadopago" {
            let mut clients = self.mercadopago_clients.write().unwrap();
            clients.insert(integration_id.to_string(), std::sync::Arc::new(crate::integrations::mercadopago::provider::MercadoPagoProvider::new(creds.api_token.clone())));
        }
        if integration_id == "shippo" {
            let mut clients = self.shippo_clients.write().unwrap();
            clients.insert(integration_id.to_string(), std::sync::Arc::new(crate::integrations::shippo::provider::ShippoProvider::new(creds.api_token.clone())));
        }
        if integration_id == "google_calendar" {
            let mut clients = self.google_calendar_clients.write().unwrap();
            clients.insert(integration_id.to_string(), std::sync::Arc::new(crate::integrations::google_calendar::provider::GoogleCalendarProvider::new(creds.api_token.clone())));
        }
        if integration_id == "cal_com" {
            let mut clients = self.cal_com_clients.write().unwrap();
            clients.insert(integration_id.to_string(), std::sync::Arc::new(crate::integrations::cal_com::provider::CalComProvider::new(creds.api_token.clone())));
        }
        if integration_id == "zoom" {
            let mut clients = self.zoom_clients.write().unwrap();
            clients.insert(integration_id.to_string(), std::sync::Arc::new(crate::integrations::zoom::provider::ZoomProvider::new(creds.api_token.clone())));
        }
        if integration_id == "ayrshare" {
            let mut clients = self.ayrshare_clients.write().unwrap();
            clients.insert(integration_id.to_string(), std::sync::Arc::new(crate::integrations::ayrshare::provider::AyrshareProvider::new(creds.api_token.clone())));
        }
        if integration_id == "listmonk" {
            let mut clients = self.listmonk_clients.write().unwrap();
            clients.insert(integration_id.to_string(), std::sync::Arc::new(crate::integrations::listmonk::provider::ListmonkProvider::new(creds.api_token.clone())));
        }
        if integration_id == "easypost" {
            let mut clients = self.easypost_clients.write().unwrap();
            clients.insert(integration_id.to_string(), std::sync::Arc::new(crate::integrations::easypost::provider::EasyPostProvider::new(creds.api_token.clone())));
        }
        if integration_id == "jitsi" {
            let mut clients = self.jitsi_clients.write().unwrap();
            clients.insert(integration_id.to_string(), std::sync::Arc::new(crate::integrations::jitsi::provider::JitsiProvider::new(creds.api_token.clone())));
        }

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

    pub fn get_credentials(&self, _integration_id: &str, tenant_id: &str) -> Result<IntegrationCredentials, String> {
        Ok(IntegrationCredentials {
            bot_token: "mock_token".to_string(), chat_id: "".to_string(), webhook_url: "".to_string(), api_token: "mock_token".to_string(), from_phone: "".to_string(), tenant_id: tenant_id.to_string()
        })
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
        let creds = ::server_ohc::orchestration::ConnectIntegrationRequest {
            integration_id: "twilio".to_string(),
            base_url: "https://api.twilio.com".to_string(),
            bot_token: "test_sid".to_string(),
            chat_id: "".to_string(),
            webhook_url: "".to_string(),
            api_token: "test_token".to_string(),
            from_phone: "+1234567890".to_string(),
        };
        registry.connect("twilio", "https://api.twilio.com", creds).unwrap();

        let msg = registry.send_chat_message("twilio", "+0987654321", "agent1", "Hello World", "thread1").unwrap();
        assert_eq!(msg.content, "Hello World");

    }
}
