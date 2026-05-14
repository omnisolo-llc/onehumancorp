use std::sync::RwLock;
use ::server_ohc::orchestration::*;
use chrono::Utc;
use std::collections::HashMap;
use std::sync::Arc;

pub struct IntegrationCredentials {
    pub bot_token: String,
    pub chat_id: String,
    pub webhook_url: String,
    pub api_token: String,
    pub from_phone: String,
}

pub struct IntegrationsRegistry {
    messages: RwLock<HashMap<String, HashMap<String, Vec<ChatMessage>>>>,
    instances: RwLock<HashMap<String, HashMap<String, IntegrationInstance>>>,
    pull_requests: RwLock<HashMap<String, HashMap<String, Vec<PullRequest>>>>,
    issues: RwLock<HashMap<String, HashMap<String, Vec<Issue>>>>,
    credentials: RwLock<HashMap<String, HashMap<String, IntegrationCredentials>>>,

    twilio_clients: RwLock<HashMap<String, Arc<crate::integrations::twilio::provider::TwilioProvider>>>,
    meta_clients: RwLock<HashMap<String, Arc<crate::integrations::meta::provider::MetaProvider>>>,
    calendar_clients: RwLock<HashMap<String, Arc<crate::integrations::google_calendar::provider::GoogleCalendarProvider>>>,
    sendgrid_clients: RwLock<HashMap<String, Arc<crate::integrations::sendgrid::provider::SendGridProvider>>>,
    shippo_clients: RwLock<HashMap<String, Arc<crate::integrations::shippo::provider::ShippoProvider>>>,
    zoom_clients: RwLock<HashMap<String, Arc<crate::integrations::zoom::provider::ZoomProvider>>>,
    nats_clients: Arc<RwLock<HashMap<String, Arc<crate::integrations::nats::provider::NatsProvider>>>>,
}

impl IntegrationsRegistry {
    pub fn new() -> Self {
        IntegrationsRegistry {
            messages: RwLock::new(HashMap::new()),
            instances: RwLock::new(HashMap::new()),
            pull_requests: RwLock::new(HashMap::new()),
            issues: RwLock::new(HashMap::new()),
            credentials: RwLock::new(HashMap::new()),
            twilio_clients: RwLock::new(HashMap::new()),
            meta_clients: RwLock::new(HashMap::new()),
            calendar_clients: RwLock::new(HashMap::new()),
            sendgrid_clients: RwLock::new(HashMap::new()),
            shippo_clients: RwLock::new(HashMap::new()),
            zoom_clients: RwLock::new(HashMap::new()),
            nats_clients: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub fn test_connection(&self, _tenant_id: &str, integration_id: &str, _creds: ChatTestRequest) -> Result<(), String> {
        if integration_id.is_empty() {
            return Err("integrationId is required".to_string());
        }
        Ok(())
    }

    pub fn chat_messages(&self, tenant_id: &str, integration_id: &str) -> Vec<ChatMessage> {
        let msgs = self.messages.read().unwrap();
        msgs.get(integration_id)
            .and_then(|t_map| t_map.get(tenant_id))
            .cloned()
            .unwrap_or_default()
    }

    pub fn send_chat_message(&self, tenant_id: &str, integration_id: &str, channel: &str, from_agent: &str, content: &str, thread_id: &str) -> Result<ChatMessage, String> {
        let msg = ChatMessage {
            id: format!("msg-{}", Utc::now().timestamp()),
            channel: channel.to_string(),
            from_agent: from_agent.to_string(),
            content: content.to_string(),
            thread_id: thread_id.to_string(),
            timestamp_unix: Utc::now().timestamp(),
        };

        let mut msgs = self.messages.write().unwrap();
        msgs.entry(integration_id.to_string())
            .or_insert_with(HashMap::new)
            .entry(tenant_id.to_string())
            .or_insert_with(Vec::new)
            .push(msg.clone());

        match integration_id {
            "twilio" => {
                if let Some(client) = self.twilio_clients.read().unwrap().get(tenant_id) {
                    let client = client.clone();
                    let to = channel.to_string();
                    let from = String::new();
                    let text = content.to_string();
                    tokio::spawn(async move {
                        let _ = client.send_sms(&to, &from, &text).await;
                    });
                }
            }
            "meta" => {
                if let Some(client) = self.meta_clients.read().unwrap().get(tenant_id) {
                    let client = client.clone();
                    let recipient_id = channel.to_string();
                    let text = content.to_string();
                    tokio::spawn(async move {
                        let _ = client.send_message(&recipient_id, &text).await;
                    });
                }
            }
            _ => {}
        }

        Ok(msg)
    }

    pub fn instances(&self, tenant_id: &str) -> Vec<IntegrationInstance> {
        let insts = self.instances.read().unwrap();
        let mut result = Vec::new();

        for provider in crate::integrations::catalog::get_catalog() {
            let id = &provider.metadata.id;
            if let Some(t_map) = insts.get(id) {
                if let Some(inst) = t_map.get(tenant_id) {
                    result.push(inst.clone());
                    continue;
                }
            }

            // Fallback to catalog template (disconnected)
            result.push(IntegrationInstance {
                id: provider.metadata.id.clone(),
                name: provider.metadata.name.clone(),
                category: provider.metadata.category.clone(),
                status: "disconnected".to_string(),
                base_url: provider.metadata.base_url.clone(),
            });
        }

        result
    }

    pub fn instances_by_category(&self, tenant_id: &str, category: &str) -> Vec<IntegrationInstance> {
        self.instances(tenant_id).into_iter().filter(|i| i.category == category).collect()
    }

    pub fn connect(&self, tenant_id: &str, integration_id: &str, base_url: &str, creds: ConnectIntegrationRequest) -> Result<IntegrationInstance, String> {
        let mut insts = self.instances.write().unwrap();
        let category = match integration_id {
            "meta" => "social_media",
            "google_calendar" => "calendar",
            "sendgrid" => "email",
            "shippo" => "shipping",
            "zoom" => "video",
            "twilio" => "sms",
            _ => "default",
        };

        let inst = IntegrationInstance {
            id: integration_id.to_string(),
            name: integration_id.to_string(),
            category: category.to_string(),
            status: "connected".to_string(),
            base_url: base_url.to_string(),
        };
        insts.entry(integration_id.to_string()).or_insert_with(HashMap::new).insert(tenant_id.to_string(), inst.clone());

        let mut credentials = self.credentials.write().unwrap();
        credentials.entry(integration_id.to_string()).or_insert_with(HashMap::new).insert(tenant_id.to_string(), IntegrationCredentials {
            bot_token: creds.bot_token.clone(),
            chat_id: creds.chat_id.clone(),
            webhook_url: creds.webhook_url.clone(),
            api_token: creds.api_token.clone(),
            from_phone: creds.from_phone.clone(),
        });

        match integration_id {
            "twilio" => {
                self.twilio_clients.write().unwrap().insert(tenant_id.to_string(), Arc::new(crate::integrations::twilio::provider::TwilioProvider::new(creds.bot_token.clone(), creds.api_token.clone())));
            }
            "meta" => {
                self.meta_clients.write().unwrap().insert(tenant_id.to_string(), Arc::new(crate::integrations::meta::provider::MetaProvider::new(creds.api_token.clone())));
            }
            "google_calendar" => {
                self.calendar_clients.write().unwrap().insert(tenant_id.to_string(), Arc::new(crate::integrations::google_calendar::provider::GoogleCalendarProvider::new(creds.api_token.clone())));
            }
            "sendgrid" => {
                self.sendgrid_clients.write().unwrap().insert(tenant_id.to_string(), Arc::new(crate::integrations::sendgrid::provider::SendGridProvider::new(creds.api_token.clone())));
            }
            "shippo" => {
                self.shippo_clients.write().unwrap().insert(tenant_id.to_string(), Arc::new(crate::integrations::shippo::provider::ShippoProvider::new(creds.api_token.clone())));
            }
            "zoom" => {
                self.zoom_clients.write().unwrap().insert(tenant_id.to_string(), Arc::new(crate::integrations::zoom::provider::ZoomProvider::new(creds.api_token.clone())));
            }
            _ => {}
        }

        Ok(inst)
    }

    pub fn disconnect(&self, tenant_id: &str, integration_id: &str) -> Result<IntegrationInstance, String> {
        let mut insts = self.instances.write().unwrap();
        if let Some(t_map) = insts.get_mut(integration_id) {
            if let Some(inst) = t_map.get_mut(tenant_id) {
                inst.status = "disconnected".to_string();
                return Ok(inst.clone());
            }
        }
        Err("integration not found".to_string())
    }

    pub fn pull_requests(&self, tenant_id: &str, integration_id: &str) -> Vec<PullRequest> {
        let prs = self.pull_requests.read().unwrap();
        prs.get(integration_id).and_then(|t_map| t_map.get(tenant_id)).cloned().unwrap_or_default()
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
        let mut prs = self.pull_requests.write().unwrap();
        prs.entry(integration_id.to_string()).or_insert_with(HashMap::new).entry(tenant_id.to_string()).or_insert_with(Vec::new).push(pr.clone());
        Ok(pr)
    }

    pub fn merge_pull_request(&self, tenant_id: &str, pr_id: &str) -> Result<PullRequest, String> {
        let mut prs = self.pull_requests.write().unwrap();
        for t_map in prs.values_mut() {
            if let Some(v) = t_map.get_mut(tenant_id) {
                if let Some(pr) = v.iter_mut().find(|p| p.id == pr_id) {
                    pr.status = "merged".to_string();
                    return Ok(pr.clone());
                }
            }
        }
        Err("pr not found".to_string())
    }

    pub fn close_pull_request(&self, tenant_id: &str, pr_id: &str) -> Result<PullRequest, String> {
        let mut prs = self.pull_requests.write().unwrap();
        for t_map in prs.values_mut() {
            if let Some(v) = t_map.get_mut(tenant_id) {
                if let Some(pr) = v.iter_mut().find(|p| p.id == pr_id) {
                    pr.status = "closed".to_string();
                    return Ok(pr.clone());
                }
            }
        }
        Err("pr not found".to_string())
    }

    pub fn issues(&self, tenant_id: &str, integration_id: &str) -> Vec<Issue> {
        let issues = self.issues.read().unwrap();
        issues.get(integration_id).and_then(|t_map| t_map.get(tenant_id)).cloned().unwrap_or_default()
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
        let mut issues = self.issues.write().unwrap();
        issues.entry(integration_id.to_string()).or_insert_with(HashMap::new).entry(tenant_id.to_string()).or_insert_with(Vec::new).push(issue.clone());
        Ok(issue)
    }

    pub fn update_issue_status(&self, tenant_id: &str, issue_id: &str, status: &str) -> Result<Issue, String> {
        let mut issues = self.issues.write().unwrap();
        for t_map in issues.values_mut() {
            if let Some(v) = t_map.get_mut(tenant_id) {
                if let Some(issue) = v.iter_mut().find(|i| i.id == issue_id) {
                    issue.status = status.to_string();
                    return Ok(issue.clone());
                }
            }
        }
        Err("issue not found".to_string())
    }

    pub fn assign_issue(&self, tenant_id: &str, issue_id: &str, assignee: &str) -> Result<Issue, String> {
        let mut issues = self.issues.write().unwrap();
        for t_map in issues.values_mut() {
            if let Some(v) = t_map.get_mut(tenant_id) {
                if let Some(issue) = v.iter_mut().find(|i| i.id == issue_id) {
                    issue.assigned_agent = assignee.to_string();
                    return Ok(issue.clone());
                }
            }
        }
        Err("issue not found".to_string())
    }
}
