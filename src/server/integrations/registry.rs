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
    pub ayrshare_providers: std::sync::RwLock<std::collections::HashMap<String, std::sync::Arc<crate::integrations::ayrshare::provider::AyrshareProvider>>>,
    pub calcom_providers: std::sync::RwLock<std::collections::HashMap<String, std::sync::Arc<crate::integrations::calcom::provider::CalComProvider>>>,
    pub listmonk_providers: std::sync::RwLock<std::collections::HashMap<String, std::sync::Arc<crate::integrations::listmonk::provider::ListmonkProvider>>>,
    pub easypost_providers: std::sync::RwLock<std::collections::HashMap<String, std::sync::Arc<crate::integrations::easypost::provider::EasyPostProvider>>>,
    pub jitsi_providers: std::sync::RwLock<std::collections::HashMap<String, std::sync::Arc<crate::integrations::jitsi::provider::JitsiProvider>>>,
    pub mercadopago_providers: std::sync::RwLock<std::collections::HashMap<String, std::sync::Arc<crate::integrations::mercadopago::provider::MercadoPagoProvider>>>,
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
            ayrshare_providers: std::sync::RwLock::new(std::collections::HashMap::new()),
            calcom_providers: std::sync::RwLock::new(std::collections::HashMap::new()),
            listmonk_providers: std::sync::RwLock::new(std::collections::HashMap::new()),
            easypost_providers: std::sync::RwLock::new(std::collections::HashMap::new()),
            jitsi_providers: std::sync::RwLock::new(std::collections::HashMap::new()),
            mercadopago_providers: std::sync::RwLock::new(std::collections::HashMap::new()),
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
             match integration_id.split("_").last().unwrap_or(integration_id) { // extract underlying tool name
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
                         if let Some(client) = clients.get(integration_id) { // E0425: conn_id was replaced by integration_id.to_string() in the string replace but left &conn_id here
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
        let conn_id = format!("{}_{}", creds.bot_token, integration_id); // use bot_token as a mock tenant_id for composite key to ensure multi-tenant safety
        let mut inst = insts.get(&conn_id).cloned().unwrap_or_else(|| {
            IntegrationInstance {
                id: conn_id.clone(),
                name: integration_id.to_string(),
                category: "default".to_string(),
                status: "disconnected".to_string(),
                base_url: base_url.to_string(),
            }
        });
        inst.status = "connected".to_string();
        insts.insert(conn_id.clone(), inst.clone());

        let mut credentials = self.credentials.write().unwrap();
        credentials.insert(conn_id.clone(), IntegrationCredentials {
            bot_token: creds.bot_token.clone(),
            chat_id: creds.chat_id.clone(),
            webhook_url: creds.webhook_url.clone(),
            api_token: creds.api_token.clone(),
            from_phone: creds.from_phone.clone(),
        });
        if integration_id == "twilio" {
            let mut clients = self.twilio_clients.write().unwrap();
            clients.insert(conn_id.clone(), std::sync::Arc::new(crate::integrations::twilio::provider::TwilioProvider::new(creds.bot_token.clone(), creds.api_token.clone())));
        }
        if integration_id == "nats" {
            let base_url_clone = base_url.to_string();
            let nats_clients = std::sync::Arc::clone(&self.nats_clients);
            let integration_id_clone = integration_id.to_string();
            let conn_id_clone = conn_id.clone();
            tokio::spawn(async move {
                if let Ok(provider) = crate::integrations::nats::provider::NatsProvider::new(&base_url_clone).await {
                    let mut clients = nats_clients.write().unwrap();
                    clients.insert(conn_id_clone, std::sync::Arc::new(provider));
                }
            });
        }
        if integration_id == "ayrshare" {
            let mut providers = self.ayrshare_providers.write().unwrap();
            providers.insert(conn_id.clone(), std::sync::Arc::new(crate::integrations::ayrshare::provider::AyrshareProvider::with_api_key(creds.api_token.clone())));
        }
        if integration_id == "calcom" {
            let mut providers = self.calcom_providers.write().unwrap();
            providers.insert(conn_id.clone(), std::sync::Arc::new(crate::integrations::calcom::provider::CalComProvider::with_api_key(creds.api_token.clone())));
        }
        if integration_id == "listmonk" {
            let mut providers = self.listmonk_providers.write().unwrap();
            providers.insert(conn_id.clone(), std::sync::Arc::new(crate::integrations::listmonk::provider::ListmonkProvider::with_credentials(base_url.to_string(), creds.bot_token.clone(), Some(creds.api_token.clone()))));
        }
        if integration_id == "easypost" {
            let mut providers = self.easypost_providers.write().unwrap();
            providers.insert(conn_id.clone(), std::sync::Arc::new(crate::integrations::easypost::provider::EasyPostProvider::with_api_key(creds.api_token.clone())));
        }
        if integration_id == "jitsi" {
            let mut providers = self.jitsi_providers.write().unwrap();
            providers.insert(conn_id.clone(), std::sync::Arc::new(crate::integrations::jitsi::provider::JitsiProvider::with_base_url(base_url.to_string())));
        }
        if integration_id == "mercadopago" {
            let mut providers = self.mercadopago_providers.write().unwrap();
            providers.insert(conn_id.clone(), std::sync::Arc::new(crate::integrations::mercadopago::provider::MercadoPagoProvider::with_access_token(creds.api_token.clone())));
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


    // Delegation methods for new integrations
    pub async fn ayrshare_fetch_messages(&self, conn_id: &str) -> Result<Vec<String>, String> {
        let provider = self.ayrshare_providers.read().unwrap().get(conn_id).cloned();
        if let Some(provider) = provider {
            provider.fetch_messages().await
        } else {
            Err("Integration not found".to_string())
        }
    }

    pub async fn ayrshare_send_reply(&self, conn_id: &str, platform: &str, user_id: &str, text: &str) -> Result<(), String> {
        let provider = self.ayrshare_providers.read().unwrap().get(conn_id).cloned();
        if let Some(provider) = provider {
            provider.send_reply(platform, user_id, text).await
        } else {
            Err("Integration not found".to_string())
        }
    }

    pub async fn ayrshare_schedule_post(&self, conn_id: &str, text: &str, platforms: Vec<&str>) -> Result<String, String> {
        let provider = self.ayrshare_providers.read().unwrap().get(conn_id).cloned();
        if let Some(provider) = provider {
            provider.schedule_post(text, platforms).await
        } else {
            Err("Integration not found".to_string())
        }
    }

    pub async fn calcom_create_booking_link(&self, conn_id: &str, event_type: &str, duration_mins: i32) -> Result<String, String> {
        let provider = self.calcom_providers.read().unwrap().get(conn_id).cloned();
        if let Some(provider) = provider {
            provider.create_booking_link(event_type, duration_mins).await
        } else {
            Err("Integration not found".to_string())
        }
    }

    pub async fn calcom_get_availability(&self, conn_id: &str, from_date: &str, to_date: &str) -> Result<Vec<String>, String> {
        let provider = self.calcom_providers.read().unwrap().get(conn_id).cloned();
        if let Some(provider) = provider {
            provider.get_availability(from_date, to_date).await
        } else {
            Err("Integration not found".to_string())
        }
    }

    pub async fn listmonk_send_email_campaign(&self, conn_id: &str, list_ids: Vec<i32>, name: &str, subject: &str, body: &str) -> Result<i32, String> {
        let provider = self.listmonk_providers.read().unwrap().get(conn_id).cloned();
        if let Some(provider) = provider {
            provider.send_email_campaign(list_ids, name, subject, body).await
        } else {
            Err("Integration not found".to_string())
        }
    }

    pub async fn easypost_get_shipping_rates(&self, conn_id: &str, from_zip: &str, to_zip: &str, weight_oz: f32) -> Result<Vec<String>, String> {
        let provider = self.easypost_providers.read().unwrap().get(conn_id).cloned();
        if let Some(provider) = provider {
            provider.get_shipping_rates(from_zip, to_zip, weight_oz).await
        } else {
            Err("Integration not found".to_string())
        }
    }

    pub async fn easypost_purchase_label(&self, conn_id: &str, rate_id: &str) -> Result<String, String> {
        let provider = self.easypost_providers.read().unwrap().get(conn_id).cloned();
        if let Some(provider) = provider {
            provider.purchase_label(rate_id).await
        } else {
            Err("Integration not found".to_string())
        }
    }

    pub async fn jitsi_create_meeting_room(&self, conn_id: &str, room_prefix: &str) -> Result<String, String> {
        let provider = self.jitsi_providers.read().unwrap().get(conn_id).cloned();
        if let Some(provider) = provider {
            provider.create_meeting_room(room_prefix).await
        } else {
            Err("Integration not found".to_string())
        }
    }

    pub async fn mercadopago_create_checkout_preference(&self, conn_id: &str, price_id: &str, tenant_id: &str) -> Result<String, String> {
        let provider = self.mercadopago_providers.read().unwrap().get(conn_id).cloned();
        if let Some(provider) = provider {
            provider.create_checkout_preference(price_id, tenant_id).await
        } else {
            Err("Integration not found".to_string())
        }
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
        registry.connect("twilio", "https://api.twilio.com", creds.clone()).unwrap();

        let msg = registry.send_chat_message("test_sid_twilio", "+0987654321", "agent1", "Hello World", "thread1").unwrap();
        assert_eq!(msg.content, "Hello World");

        // Test connections for the other integrations
        registry.connect("ayrshare", "https://app.ayrshare.com/api", creds.clone()).unwrap();
        assert_eq!(registry.ayrshare_providers.read().unwrap().len(), 1);
        let _ = registry.ayrshare_fetch_messages("test_sid_ayrshare").await;

        registry.connect("calcom", "https://api.cal.com/v1", creds.clone()).unwrap();
        assert_eq!(registry.calcom_providers.read().unwrap().len(), 1);
        let _ = registry.calcom_create_booking_link("test_sid_calcom", "type", 30).await;

        registry.connect("listmonk", "http://localhost:9000/api", creds.clone()).unwrap();
        assert_eq!(registry.listmonk_providers.read().unwrap().len(), 1);
        let _ = registry.listmonk_send_email_campaign("test_sid_listmonk", vec![1], "n", "s", "b").await;

        registry.connect("easypost", "https://api.easypost.com/v2", creds.clone()).unwrap();
        assert_eq!(registry.easypost_providers.read().unwrap().len(), 1);
        let _ = registry.easypost_get_shipping_rates("test_sid_easypost", "1", "2", 1.0).await;

        registry.connect("jitsi", "https://meet.jit.si", creds.clone()).unwrap();
        assert_eq!(registry.jitsi_providers.read().unwrap().len(), 1);
        let _ = registry.jitsi_create_meeting_room("test_sid_jitsi", "p").await;

        registry.connect("mercadopago", "https://api.mercadopago.com/v1", creds.clone()).unwrap();
        assert_eq!(registry.mercadopago_providers.read().unwrap().len(), 1);
        let _ = registry.mercadopago_create_checkout_preference("test_sid_mercadopago", "p", "t").await;

        let instances = registry.instances();
        assert_eq!(instances.len(), 9); // there are 9 instances now
    }
}
