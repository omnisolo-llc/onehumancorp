use std::sync::RwLock;
use crate::ohc::orchestration::*;
use chrono::Utc;

pub struct IntegrationsRegistry {
    messages: RwLock<std::collections::HashMap<String, Vec<ChatMessage>>>,
    instances: RwLock<std::collections::HashMap<String, IntegrationInstance>>,
    pull_requests: RwLock<std::collections::HashMap<String, Vec<PullRequest>>>,
    issues: RwLock<std::collections::HashMap<String, Vec<Issue>>>,
}

impl IntegrationsRegistry {
    pub fn new() -> Self {
        IntegrationsRegistry {
            messages: RwLock::new(std::collections::HashMap::new()),
            instances: RwLock::new(std::collections::HashMap::new()),
            pull_requests: RwLock::new(std::collections::HashMap::new()),
            issues: RwLock::new(std::collections::HashMap::new()),
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

    pub fn connect(&self, integration_id: &str, base_url: &str, _creds: ConnectIntegrationRequest) -> Result<IntegrationInstance, String> {
        let mut insts = self.instances.write().unwrap();
        let inst = IntegrationInstance {
            id: integration_id.to_string(),
            name: integration_id.to_string(),
            category: "default".to_string(),
            status: "connected".to_string(),
            base_url: base_url.to_string(),
        };
        insts.insert(integration_id.to_string(), inst.clone());
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
}
