use std::sync::RwLock;
#[allow(unused_imports)]
use ::server_ohc::orchestration::*;
use chrono::Utc;

pub struct IntegrationCredentials {
    pub bot_token: String,
    pub chat_id: String,
    pub webhook_url: String,
    pub api_token: String,
    pub from_phone: String,
}

pub struct IntegrationsRegistry {
    messages: RwLock<std::collections::HashMap<String, Vec<::server_ohc::orchestration::ChatMessage>>>,
    instances: RwLock<std::collections::HashMap<String, ::server_ohc::orchestration::IntegrationInstance>>,
    pull_requests: RwLock<std::collections::HashMap<String, Vec<::server_ohc::orchestration::PullRequest>>>,
    issues: RwLock<std::collections::HashMap<String, Vec<::server_ohc::orchestration::Issue>>>,
    credentials: RwLock<std::collections::HashMap<String, IntegrationCredentials>>,
    twilio_clients: std::sync::RwLock<std::collections::HashMap<String, std::sync::Arc<crate::integrations::twilio::provider::TwilioProvider>>>,
    nats_clients: std::sync::Arc<std::sync::RwLock<std::collections::HashMap<String, std::sync::Arc<crate::integrations::nats::provider::NatsProvider>>>>,
    meta_clients: std::sync::RwLock<std::collections::HashMap<String, std::sync::Arc<crate::integrations::meta::provider::MetaProvider>>>,
    calendly_clients: std::sync::RwLock<std::collections::HashMap<String, std::sync::Arc<crate::integrations::calendly::provider::CalendlyProvider>>>,
    cal_com_clients: std::sync::RwLock<std::collections::HashMap<String, std::sync::Arc<crate::integrations::cal_com::provider::CalComProvider>>>,
    google_calendar_clients: std::sync::RwLock<std::collections::HashMap<String, std::sync::Arc<crate::integrations::google_calendar::provider::GoogleCalendarProvider>>>,
    mailchimp_clients: std::sync::RwLock<std::collections::HashMap<String, std::sync::Arc<crate::integrations::mailchimp::provider::MailchimpProvider>>>,
    mercadopago_clients: std::sync::RwLock<std::collections::HashMap<String, std::sync::Arc<crate::integrations::mercadopago::provider::MercadoPagoProvider>>>,
    alipay_clients: std::sync::RwLock<std::collections::HashMap<String, std::sync::Arc<crate::integrations::alipay::provider::AlipayProvider>>>,
    pub razorpay_clients: std::sync::RwLock<std::collections::HashMap<String, std::sync::Arc<crate::integrations::razorpay::provider::RazorpayProvider>>>,
    pub manychat_clients: std::sync::RwLock<std::collections::HashMap<String, std::sync::Arc<crate::integrations::manychat::provider::ManychatProvider>>>,
    shippo_clients: std::sync::RwLock<std::collections::HashMap<String, std::sync::Arc<crate::integrations::shippo::provider::ShippoProvider>>>,
    zoom_clients: std::sync::RwLock<std::collections::HashMap<String, std::sync::Arc<crate::integrations::zoom::provider::ZoomProvider>>>,
    jitsi_clients: std::sync::RwLock<std::collections::HashMap<String, std::sync::Arc<crate::integrations::jitsi::provider::JitsiProvider>>>,
    ayrshare_clients: std::sync::RwLock<std::collections::HashMap<String, std::sync::Arc<crate::integrations::ayrshare::provider::AyrshareProvider>>>,
    listmonk_clients: std::sync::RwLock<std::collections::HashMap<String, std::sync::Arc<crate::integrations::listmonk::provider::ListmonkProvider>>>,
    easypost_clients: std::sync::RwLock<std::collections::HashMap<String, std::sync::Arc<crate::integrations::easypost::provider::EasyPostProvider>>>,
    sendgrid_clients: std::sync::RwLock<std::collections::HashMap<String, std::sync::Arc<crate::integrations::sendgrid::provider::SendGridProvider>>>,
}

impl IntegrationsRegistry {
    pub fn new() -> Self {
        let mut instances = std::collections::HashMap::new();
        for provider in crate::integrations::catalog::get_catalog() {
            let id = provider.metadata.id.clone();
            instances.insert(id.clone(), ::server_ohc::orchestration::IntegrationInstance {
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
            calendly_clients: std::sync::RwLock::new(std::collections::HashMap::new()),
            cal_com_clients: std::sync::RwLock::new(std::collections::HashMap::new()),
            google_calendar_clients: std::sync::RwLock::new(std::collections::HashMap::new()),
            mailchimp_clients: std::sync::RwLock::new(std::collections::HashMap::new()),
            mercadopago_clients: std::sync::RwLock::new(std::collections::HashMap::new()),
            razorpay_clients: std::sync::RwLock::new(std::collections::HashMap::new()),
            manychat_clients: std::sync::RwLock::new(std::collections::HashMap::new()),
            alipay_clients: std::sync::RwLock::new(std::collections::HashMap::new()),
            shippo_clients: std::sync::RwLock::new(std::collections::HashMap::new()),
            zoom_clients: std::sync::RwLock::new(std::collections::HashMap::new()),
            jitsi_clients: std::sync::RwLock::new(std::collections::HashMap::new()),
            ayrshare_clients: std::sync::RwLock::new(std::collections::HashMap::new()),
            listmonk_clients: std::sync::RwLock::new(std::collections::HashMap::new()),
            easypost_clients: std::sync::RwLock::new(std::collections::HashMap::new()),
            sendgrid_clients: std::sync::RwLock::new(std::collections::HashMap::new()),
        }
    }

    // Chat methods
    pub fn test_connection(&self, integration_id: &str, _creds: ::server_ohc::orchestration::ChatTestRequest) -> Result<(), String> {
        if integration_id.is_empty() {
            return Err("integrationId is required".to_string());
        }
        Ok(())
    }

    pub fn chat_messages(&self, integration_id: &str) -> Vec<::server_ohc::orchestration::ChatMessage> {
        let msgs = self.messages.read().unwrap();
        msgs.get(integration_id).cloned().unwrap_or_default()
    }

    pub fn send_chat_message(&self, integration_id: &str, channel: &str, from_agent: &str, content: &str, thread_id: &str) -> Result<::server_ohc::orchestration::ChatMessage, String> {
        let msg = ::server_ohc::orchestration::ChatMessage {
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
                         if let Some(client) = clients.get(integration_id) {
                             let client = client.clone();
                             tokio::spawn(async move {
                                 if let Err(e) = client.send_sms(&to, &from, &text).await {
                                     tracing::error!("Failed to send Twilio SMS: {}", e);
                                 }
                             });
                         }
                     }
                 }
                 "meta" => {
                     if !creds.api_token.is_empty() {
                         let to = if !creds.chat_id.is_empty() { creds.chat_id.clone() } else { channel.to_string() };
                         let text = content.to_string();

                         let clients = self.meta_clients.read().unwrap();
                         if let Some(client) = clients.get(integration_id) {
                             let client = client.clone();
                             tokio::spawn(async move {
                                 // For this naive integration, we assume channel might specify the platform like "whatsapp", "instagram"
                                 // Otherwise we default to whatsapp
                                 let platform = if to.contains("whatsapp") { "whatsapp" } else if to.contains("instagram") { "instagram" } else { "facebook" };
                                 if let Err(e) = client.send_message(platform, &to, &text).await {
                                     tracing::error!("Failed to send Meta message: {}", e);
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
    pub fn instances(&self) -> Vec<::server_ohc::orchestration::IntegrationInstance> {
        let insts = self.instances.read().unwrap();
        insts.values().cloned().collect()
    }

    pub fn instances_by_category(&self, category: &str) -> Vec<::server_ohc::orchestration::IntegrationInstance> {
        let insts = self.instances.read().unwrap();
        insts.values().filter(|i| i.category == category).cloned().collect()
    }

    pub fn connect(&self, integration_id: &str, base_url: &str, creds: ::server_ohc::orchestration::ConnectIntegrationRequest) -> Result<::server_ohc::orchestration::IntegrationInstance, String> {
        let mut insts = self.instances.write().unwrap();
        let inst = ::server_ohc::orchestration::IntegrationInstance {
            id: integration_id.to_string(),
            name: integration_id.to_string(),
            category: "default".to_string(),
            status: "connected".to_string(),
            base_url: base_url.to_string(),
        };
        insts.insert(integration_id.to_string(), inst.clone());

        let mut credentials = self.credentials.write().unwrap();
        credentials.insert(integration_id.to_string(), IntegrationCredentials {
            bot_token: creds.bot_token.clone(),
            chat_id: creds.chat_id.clone(),
            webhook_url: creds.webhook_url.clone(),
            api_token: creds.api_token.clone(),
            from_phone: creds.from_phone.clone(),
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
        if integration_id == "calendly" {
            let mut clients = self.calendly_clients.write().unwrap();
            clients.insert(integration_id.to_string(), std::sync::Arc::new(crate::integrations::calendly::provider::CalendlyProvider::new(creds.api_token.clone())));
        }
        if integration_id == "cal_com" {
            let mut clients = self.cal_com_clients.write().unwrap();
            clients.insert(integration_id.to_string(), std::sync::Arc::new(crate::integrations::cal_com::provider::CalComProvider::new(creds.api_token.clone())));
        }
        if integration_id == "google_calendar" {
            let mut clients = self.google_calendar_clients.write().unwrap();
            clients.insert(integration_id.to_string(), std::sync::Arc::new(crate::integrations::google_calendar::provider::GoogleCalendarProvider::new(creds.api_token.clone())));
        }
        if integration_id == "mailchimp" {
            let mut clients = self.mailchimp_clients.write().unwrap();
            clients.insert(integration_id.to_string(), std::sync::Arc::new(crate::integrations::mailchimp::provider::MailchimpProvider::new(creds.api_token.clone())));
        }
        if integration_id == "alipay" {
            let mut clients = self.alipay_clients.write().unwrap();
            clients.insert(integration_id.to_string(), std::sync::Arc::new(crate::integrations::alipay::provider::AlipayProvider::new(creds.api_token.clone())));
        }
        if integration_id == "mercadopago" {
            let mut clients = self.mercadopago_clients.write().unwrap();
            clients.insert(integration_id.to_string(), std::sync::Arc::new(crate::integrations::mercadopago::provider::MercadoPagoProvider::new(creds.api_token.clone())));
        }

        if integration_id == "razorpay" {
            let mut clients = self.razorpay_clients.write().unwrap();
            clients.insert(integration_id.to_string(), std::sync::Arc::new(crate::integrations::razorpay::provider::RazorpayProvider::new(creds.api_token.clone(), creds.api_token.clone())));
        }
        if integration_id == "shippo" {
            let mut clients = self.shippo_clients.write().unwrap();
            clients.insert(integration_id.to_string(), std::sync::Arc::new(crate::integrations::shippo::provider::ShippoProvider::new(creds.api_token.clone())));
        }
        if integration_id == "zoom" {
            let mut clients = self.zoom_clients.write().unwrap();
            clients.insert(integration_id.to_string(), std::sync::Arc::new(crate::integrations::zoom::provider::ZoomProvider::new(creds.api_token.clone())));
        }
        if integration_id == "jitsi" {
            let mut clients = self.jitsi_clients.write().unwrap();
            clients.insert(integration_id.to_string(), std::sync::Arc::new(crate::integrations::jitsi::provider::JitsiProvider::new(creds.api_token.clone())));
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

        if integration_id == "manychat" {
            let mut clients = self.manychat_clients.write().unwrap();
            clients.insert(integration_id.to_string(), std::sync::Arc::new(crate::integrations::manychat::provider::ManychatProvider::new(creds.api_token.clone())));
        }

        if integration_id == "sendgrid" {
            let mut clients = self.sendgrid_clients.write().unwrap();
            clients.insert(integration_id.to_string(), std::sync::Arc::new(crate::integrations::sendgrid::provider::SendGridProvider::new(creds.api_token.clone())));
        }

        Ok(inst)
    }

    pub fn disconnect(&self, integration_id: &str) -> Result<::server_ohc::orchestration::IntegrationInstance, String> {
        let mut insts = self.instances.write().unwrap();
        if let Some(inst) = insts.get_mut(integration_id) {
            inst.status = "disconnected".to_string();
            return Ok(inst.clone());
        }
        Err("integration not found".to_string())
    }

    pub fn pull_requests(&self, integration_id: &str) -> Vec<::server_ohc::orchestration::PullRequest> {
        let prs = self.pull_requests.read().unwrap();
        prs.get(integration_id).cloned().unwrap_or_default()
    }

    pub fn create_pull_request(&self, integration_id: &str, _repository: &str, title: &str, body: &str, source_branch: &str, target_branch: &str, created_by: &str) -> Result<::server_ohc::orchestration::PullRequest, String> {
        let pr = ::server_ohc::orchestration::PullRequest {
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

    pub fn merge_pull_request(&self, pr_id: &str) -> Result<::server_ohc::orchestration::PullRequest, String> {
        let mut prs = self.pull_requests.write().unwrap();
        for v in prs.values_mut() {
            if let Some(pr) = v.iter_mut().find(|p| p.id == pr_id) {
                pr.status = "merged".to_string();
                return Ok(pr.clone());
            }
        }
        Err("pr not found".to_string())
    }

    pub fn close_pull_request(&self, pr_id: &str) -> Result<::server_ohc::orchestration::PullRequest, String> {
        let mut prs = self.pull_requests.write().unwrap();
        for v in prs.values_mut() {
            if let Some(pr) = v.iter_mut().find(|p| p.id == pr_id) {
                pr.status = "closed".to_string();
                return Ok(pr.clone());
            }
        }
        Err("pr not found".to_string())
    }

    pub fn issues(&self, integration_id: &str) -> Vec<::server_ohc::orchestration::Issue> {
        let issues = self.issues.read().unwrap();
        issues.get(integration_id).cloned().unwrap_or_default()
    }

    pub fn create_issue(&self, integration_id: &str, _project: &str, title: &str, description: &str, created_by: &str, priority: &str, labels: Vec<String>) -> Result<::server_ohc::orchestration::Issue, String> {
        let issue = ::server_ohc::orchestration::Issue {
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

    pub fn update_issue_status(&self, issue_id: &str, status: &str) -> Result<::server_ohc::orchestration::Issue, String> {
        let mut issues = self.issues.write().unwrap();
        for v in issues.values_mut() {
            if let Some(issue) = v.iter_mut().find(|i| i.id == issue_id) {
                issue.status = status.to_string();
                return Ok(issue.clone());
            }
        }
        Err("issue not found".to_string())
    }

    pub fn assign_issue(&self, issue_id: &str, assignee: &str) -> Result<::server_ohc::orchestration::Issue, String> {
        let mut issues = self.issues.write().unwrap();
        for v in issues.values_mut() {
            if let Some(issue) = v.iter_mut().find(|i| i.id == issue_id) {
                issue.assigned_agent = assignee.to_string();
                return Ok(issue.clone());
            }
        }
        Err("issue not found".to_string())
    }

    pub async fn get_free_busy(&self, integration_id: &str, time_min: &str, time_max: &str) -> Result<String, String> {
        let client = {
            if integration_id == "google_calendar" {
                let clients = self.google_calendar_clients.read().unwrap();
                clients.get(integration_id).cloned()
            } else {
                None
            }
        };
        if let Some(c) = client {
            return c.get_free_busy(time_min, time_max).await;
        }
        Err("integration not found or not supported".to_string())
    }

    pub async fn fetch_event_types(&self, integration_id: &str) -> Result<Vec<String>, String> {
        let client = {
            if integration_id == "calendly" {
                let clients = self.calendly_clients.read().unwrap();
                clients.get(integration_id).cloned()
            } else {
                None
            }
        };
        if let Some(c) = client {
            return c.fetch_event_types().await;
        }
        Err("integration not found or not supported".to_string())
    }

    pub async fn sync_customer(&self, integration_id: &str, email: &str, tag: &str) -> Result<(), String> {
        let client = {
            if integration_id == "mailchimp" {
                let clients = self.mailchimp_clients.read().unwrap();
                clients.get(integration_id).cloned()
            } else {
                None
            }
        };
        if let Some(c) = client {
            return c.sync_customer(email, tag).await;
        }
        Err("integration not found or not supported".to_string())
    }

    pub async fn send_campaign(&self, integration_id: &str, audience: &str, body: &str) -> Result<(), String> {
        let client = {
            if integration_id == "mailchimp" {
                let clients = self.mailchimp_clients.read().unwrap();
                clients.get(integration_id).cloned()
            } else {
                None
            }
        };
        if let Some(c) = client {
            return c.send_campaign(audience, body).await;
        }
        Err("integration not found or not supported".to_string())
    }

    pub async fn send_message(&self, integration_id: &str, platform: &str, to: &str, body: &str) -> Result<(), String> {
        let client = {
            if integration_id == "meta" {
                let clients = self.meta_clients.read().unwrap();
                clients.get(integration_id).cloned()
            } else {
                None
            }
        };
        if let Some(c) = client {
            return c.send_message(platform, to, body).await;
        }
        Err("integration not found or not supported".to_string())
    }

    pub async fn send_sms(&self, integration_id: &str, to: &str, from: &str, body: &str) -> Result<(), String> {
        let client = {
            if integration_id == "twilio" {
                let clients = self.twilio_clients.read().unwrap();
                clients.get(integration_id).cloned()
            } else {
                None
            }
        };
        if let Some(c) = client {
            return c.send_sms(to, from, body).await;
        }
        Err("integration not found or not supported".to_string())
    }

    pub async fn mercadopago_create_payment(&self, integration_id: &str, amount: f64, description: &str, payer_email: &str) -> Result<String, String> {
        let client = {
            if integration_id == "mercadopago" {
                let clients = self.mercadopago_clients.read().unwrap();
                clients.get(integration_id).cloned()
            } else {
                None
            }
        };
        if let Some(c) = client {
            return c.create_payment(amount, description, payer_email).await;
        }
        Err("integration not found or not supported".to_string())
    }

    pub async fn handle_webhook(&self, integration_id: &str, payload: &str) -> Result<(), String> {
        let client = {
            if integration_id == "mercadopago" {
                let clients = self.mercadopago_clients.read().unwrap();
                clients.get(integration_id).cloned()
            } else {
                None
            }
        };
        if let Some(c) = client {
            return c.handle_webhook(payload).await;
        }
        Err("integration not found or not supported".to_string())
    }

    pub async fn alipay_create_checkout_preference(&self, integration_id: &str, price_id: &str, tenant_id: &str) -> Result<String, String> {
        let client = {
            if integration_id == "alipay" {
                let clients = self.alipay_clients.read().unwrap();
                clients.get(integration_id).cloned()
            } else {
                None
            }
        };
        if let Some(c) = client {
            return c.create_checkout_preference(price_id, tenant_id).await;
        }
        Err("integration not found or not supported".to_string())
    }

    pub async fn mercadopago_create_checkout_preference(&self, integration_id: &str, price_id: &str, tenant_id: &str) -> Result<String, String> {
        let client = {
            if integration_id == "mercadopago" {
                let clients = self.mercadopago_clients.read().unwrap();
                clients.get(integration_id).cloned()
            } else {
                None
            }
        };
        if let Some(c) = client {
            return c.create_checkout_preference(price_id, tenant_id).await;
        }
        Err("integration not found or not supported".to_string())
    }

    pub async fn razorpay_create_checkout_preference(&self, integration_id: &str, price_id: &str, tenant_id: &str) -> Result<String, String> {
        let client = {
            if integration_id == "razorpay" {
                let clients = self.razorpay_clients.read().unwrap();
                clients.get(integration_id).cloned()
            } else {
                None
            }
        };
        if let Some(c) = client {
            return c.create_checkout_preference(price_id, tenant_id).await;
        }
        Err("integration not found or not supported".to_string())
    }
    pub async fn fetch_rates(&self, integration_id: &str, weight: f64, dimensions: &str) -> Result<Vec<String>, String> {
        let client = {
            if integration_id == "shippo" {
                let clients = self.shippo_clients.read().unwrap();
                clients.get(integration_id).cloned()
            } else {
                None
            }
        };
        if let Some(c) = client {
            return c.fetch_rates(weight, dimensions).await;
        }
        Err("integration not found or not supported".to_string())
    }

    pub async fn purchase_label(&self, integration_id: &str, rate_id: &str) -> Result<String, String> {
        let client = {
            if integration_id == "shippo" {
                let clients = self.shippo_clients.read().unwrap();
                clients.get(integration_id).cloned()
            } else {
                None
            }
        };
        if let Some(c) = client {
            return c.purchase_label(rate_id).await;
        }
        Err("integration not found or not supported".to_string())
    }

    pub async fn ayrshare_post_message(&self, integration_id: &str, message: &str, platforms: Vec<&str>) -> Result<(), String> {
        let client = {
            if integration_id == "ayrshare" {
                let clients = self.ayrshare_clients.read().unwrap();
                clients.get(integration_id).cloned()
            } else {
                None
            }
        };
        if let Some(c) = client {
            return c.post_message(message, platforms).await;
        }
        Err("integration not found or not supported".to_string())
    }

    pub async fn listmonk_send_campaign(&self, integration_id: &str, list_id: &str, template_id: &str, subject: &str, body: &str) -> Result<(), String> {
        let client = {
            if integration_id == "listmonk" {
                let clients = self.listmonk_clients.read().unwrap();
                clients.get(integration_id).cloned()
            } else {
                None
            }
        };
        if let Some(c) = client {
            return c.send_campaign(list_id, template_id, subject, body).await;
        }
        Err("integration not found or not supported".to_string())
    }

    pub async fn mercadopago_handle_webhook(&self, integration_id: &str, payload: &str) -> Result<(), String> {
        let client = {
            if integration_id == "mercadopago" {
                let clients = self.mercadopago_clients.read().unwrap();
                clients.get(integration_id).cloned()
            } else {
                None
            }
        };
        if let Some(c) = client {
            return c.handle_webhook(payload).await;
        }
        Err("integration not found or not supported".to_string())
    }

    pub async fn easypost_create_shipment(&self, integration_id: &str, to_address: &str, from_address: &str, parcel_details: &str) -> Result<String, String> {
        let client = {
            if integration_id == "easypost" {
                let clients = self.easypost_clients.read().unwrap();
                clients.get(integration_id).cloned()
            } else {
                None
            }
        };
        if let Some(c) = client {
            return c.create_shipment(to_address, from_address, parcel_details).await;
        }
        Err("integration not found or not supported".to_string())
    }

    pub async fn create_meeting(&self, integration_id: &str, topic: &str) -> Result<String, String> {
        let client_zoom = {
            if integration_id == "zoom" {
                let clients = self.zoom_clients.read().unwrap();
                clients.get(integration_id).cloned()
            } else {
                None
            }
        };
        if let Some(c) = client_zoom {
            return c.create_meeting(topic).await;
        }

        let client_jitsi = {
            if integration_id == "jitsi" {
                let clients = self.jitsi_clients.read().unwrap();
                clients.get(integration_id).cloned()
            } else {
                None
            }
        };
        if let Some(c) = client_jitsi {
            return c.create_meeting(topic).await;
        }

        Err("integration not found or not supported".to_string())
    }

    pub async fn create_event(&self, integration_id: &str, summary: &str, start_time: &str, end_time: &str) -> Result<String, String> {
        let client = {
            if integration_id == "google_calendar" {
                let clients = self.google_calendar_clients.read().unwrap();
                clients.get(integration_id).cloned()
            } else {
                None
            }
        };
        if let Some(c) = client {
            return c.create_event(summary, start_time, end_time).await;
        }
        Err("integration not found or not supported".to_string())
    }

    pub async fn get_booking_link(&self, integration_id: &str, event_type: &str) -> Result<String, String> {
        let client = {
            if integration_id == "cal_com" {
                let clients = self.cal_com_clients.read().unwrap();
                clients.get(integration_id).cloned()
            } else {
                None
            }
        };
        if let Some(c) = client {
            return c.get_booking_link(event_type).await;
        }
        Err("integration not found or not supported".to_string())
    }

    pub async fn send_email(&self, integration_id: &str, to: &str, subject: &str, body: &str) -> Result<(), String> {
        let client = {
            if integration_id == "sendgrid" {
                let clients = self.sendgrid_clients.read().unwrap();
                clients.get(integration_id).cloned()
            } else {
                None
            }
        };
        if let Some(c) = client {
            return c.send_email(to, subject, body).await;
        }
        Err("integration not found or not supported".to_string())
    }

    pub async fn create_shipment(&self, integration_id: &str, to_address: &str, from_address: &str, parcel_details: &str) -> Result<String, String> {
        let client = {
            if integration_id == "easypost" {
                let clients = self.easypost_clients.read().unwrap();
                clients.get(integration_id).cloned()
            } else {
                None
            }
        };
        if let Some(c) = client {
            return c.create_shipment(to_address, from_address, parcel_details).await;
        }
        Err("integration not found or not supported".to_string())
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
