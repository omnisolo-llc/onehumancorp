use std::collections::HashMap;
use std::sync::RwLock;
use std::sync::OnceLock;
use dashmap::{DashMap, DashSet};
use regex::Regex;
use crate::ohc::orchestration::{Agent, MeetingRoom, Message, AgentCapabilities, MeshEvent, TeammateMeshEvent};
use tokio::sync::broadcast;
use tokio::sync::mpsc;
use crate::billing::Tracker;
use crate::tasks::TaskManager;
use crate::scheduler::Scheduler;
use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};
use std::sync::Arc;
use crate::services::billing::auditor::CostAuditor;
use crate::pricing::calculator::CostConfig;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HubEvent {
    pub r#type: String,
    pub payload: String,
    pub occurred_at: DateTime<Utc>,
}

pub struct Hub {
    agents: DashMap<String, Agent>,
    meetings: DashMap<String, MeetingRoom>,
    inbox: DashMap<String, Vec<Message>>,
    subs: DashMap<String, broadcast::Sender<Message>>,
    minimax_api_key: String,
    caps_tx: broadcast::Sender<AgentCapabilities>,
    mesh_events: DashMap<String, broadcast::Sender<MeshEvent>>,
    teammate_events: DashMap<String, broadcast::Sender<TeammateMeshEvent>>,
    tracker: Tracker,
    task_manager: TaskManager,
    scheduler: Scheduler,
    cost_auditor: Arc<CostAuditor>,
    recent_events: RwLock<Vec<HubEvent>>,
    token_usage_history: DashMap<String, Vec<i64>>,
    get_token_usage: Option<Box<dyn Fn() -> HashMap<String, i64> + Send + Sync>>,
    auto_cor_track: DashSet<String>,
    event_log_tx: mpsc::Sender<serde_json::Value>,
    pub(crate) pool: sqlx::PgPool,
}

impl Hub {
    pub fn new(event_log_tx: mpsc::Sender<serde_json::Value>, pool: sqlx::PgPool) -> Self {
        let minimax_api_key = std::env::var("MINIMAX_API_KEY").unwrap_or_default();
        let (caps_tx, _) = broadcast::channel(100);
        Hub {
            agents: DashMap::new(),
            meetings: DashMap::new(),
            inbox: DashMap::new(),
            subs: DashMap::new(),
            minimax_api_key,
            caps_tx,
            pool,
            mesh_events: DashMap::new(),
            teammate_events: DashMap::new(),
            tracker: Tracker::new(),
            task_manager: TaskManager::new(),
            scheduler: Scheduler::new(),
            cost_auditor: Arc::new(CostAuditor::new(CostConfig::default())),
            recent_events: RwLock::new(Vec::new()),
            token_usage_history: DashMap::new(),
            get_token_usage: None,
            auto_cor_track: DashSet::new(),
            event_log_tx,
        }
    }

    pub fn get_cost_auditor(&self) -> Arc<CostAuditor> {
        self.cost_auditor.clone()
    }

    pub fn register_agent(&self, agent: Agent) {
        self.agents.insert(agent.id.clone(), agent);
    }

    pub fn get_agent(&self, id: &str) -> Option<Agent> {
        self.agents.get(id).map(|r| r.value().clone())
    }

    pub fn get_agents_count(&self) -> usize {
        self.agents.len()
    }

    pub fn fire_agent(&self, id: &str) {
        self.agents.remove(id);
        self.inbox.remove(id);
    }

    pub fn get_agents(&self) -> Vec<Agent> {
        let mut agents_vec: Vec<Agent> = self.agents.iter().map(|r| r.value().clone()).collect();
        agents_vec.sort_by(|a, b| a.id.cmp(&b.id));
        agents_vec
    }

    pub fn get_agents_by_org(&self, org_id: &str) -> Vec<Agent> {
        let mut agents_vec: Vec<Agent> = self.agents.iter()
            .filter(|r| r.value().organization_id == org_id || r.key().starts_with(&format!("{}-", org_id)))
            .map(|r| r.value().clone())
            .collect();
        agents_vec.sort_by(|a, b| a.id.cmp(&b.id));
        agents_vec
    }

    pub fn open_meeting(&self, id: String, participants: Vec<String>, agenda: String) -> MeetingRoom {
        let meeting = MeetingRoom {
            id: id.clone(),
            agenda,
            participants: participants.clone(),
            transcript: vec![],
        };
        
        self.meetings.insert(id, meeting.clone());
        
        for participant in participants {
            if let Some(mut agent) = self.agents.get_mut(&participant) {
                agent.status = "IN_MEETING".to_string();
            }
        }
        
        meeting
    }

    pub fn publish(self: std::sync::Arc<Self>, msg: Message) -> Result<(), String> {
        let to_agent = msg.to_agent.clone();

        // Check rate limiting
        let tenant_id = msg.to_agent.split("-").next().unwrap_or("default").to_string();
        let agent_id = msg.to_agent.clone();
        let tracker = self.tracker.clone();
        tokio::spawn(async move {
            if let Ok(limit_status) = tracker.check_rate_limit(&tenant_id, &agent_id).await {
                if limit_status.soft_limit_reached {
                    println!("Rate limit warning: {:?}", limit_status.user_message);
                }
            }
        });
        
        // Add to recipient's inbox
        self.inbox.entry(to_agent.clone()).or_insert_with(Vec::new).push(msg.clone());
        
        // Add to meeting transcript if applicable
        if !msg.meeting_id.is_empty() {
            if let Some(mut meeting) = self.meetings.get_mut(&msg.meeting_id) {
                meeting.transcript.push(msg.clone());
                
                // Aggressive AI Context Summarization
                if meeting.transcript.len() > 10 && !self.minimax_api_key.is_empty() {
                    let api_key = self.minimax_api_key.clone();
                    let m_id = msg.meeting_id.clone();
                    let transcript = meeting.transcript.clone();
                    let hub = self.clone();
                    
                    tokio::spawn(async move {
                        let client = crate::minimax::MinimaxClient::new(api_key);
                        let mut prompt = "Extract and summarize ONLY the exact parameters, architectural decisions, and required next steps from this transcript. Discard all conversational filler, pleasantries, and non-actionable text. Output MUST be an ultra-dense, bulleted technical brief optimized for minimal token footprint:\n".to_string();
                        
                        for m in &transcript {
                            prompt.push_str(&format!("{}: {}\n", m.from_agent, m.content));
                        }
                        
                        match client.reason(&prompt).await {
                            Ok(summary) => {
                                if let Some(mut mtg) = hub.meetings.get_mut(&m_id) {
                                    let mut new_transcript = vec![Message {
                                        id: format!("summary-{}", Utc::now().timestamp()),
                                        from_agent: "SYSTEM_SUMMARIZER".to_string(),
                                        to_agent: "all".to_string(),
                                        r#type: "status".to_string(),
                                        content: format!("[CONTEXT SUMMARIZED]: {}", summary),
                                        meeting_id: m_id.clone(),
                                        occurred_at_unix: Utc::now().timestamp(),
                                    }];
                                    
                                    if mtg.transcript.len() > 3 {
                                        new_transcript.extend(mtg.transcript.iter().cloned().skip(mtg.transcript.len() - 3));
                                    } else {
                                        new_transcript.extend(mtg.transcript.iter().cloned());
                                    }
                                    mtg.transcript = new_transcript;
                                }
                            }
                            Err(e) => println!("Summarization failed: {}", e),
                        }
                    });
                }
            }
        }
        
        // Notify subscribers
        if let Some(tx) = self.subs.get(&to_agent) {
            let _ = tx.send(msg);
        }
        
        Ok(())
    }

    pub fn get_meetings(&self) -> Vec<MeetingRoom> {
        self.meetings.iter().map(|r| r.value().clone()).collect()
    }

    pub fn get_inbox(&self, agent_id: &str) -> Vec<Message> {
        self.inbox.remove(agent_id).map(|(_, v)| v).unwrap_or_default()
    }

    pub fn subscribe(&self, agent_id: String) -> broadcast::Receiver<Message> {
        let tx = self.subs.entry(agent_id).or_insert_with(|| {
            let (tx, _) = broadcast::channel(100); // Buffer of 100
            tx
        });
        tx.subscribe()
    }

    pub fn delegate_task(self: std::sync::Arc<Self>, from_agent_id: String, to_agent_id: String, mut task: Message) -> Result<(), String> {
        check_documentation_gate(&task.content)?;
        
        if !self.agents.contains_key(&from_agent_id) {
            return Err("sender agent is not registered".to_string());
        }
        if !self.agents.contains_key(&to_agent_id) {
            return Err("recipient agent is not registered".to_string());
        }
        
        task.from_agent = from_agent_id;
        task.to_agent = to_agent_id;
        
        self.publish(task)
    }

    pub fn delegate_sub_task(
        self: std::sync::Arc<Self>,
        from_agent_id: &str,
        target_role: &str,
        instruction: &str,
        parent_thread_id: &str,
    ) -> Result<String, String> {
        check_documentation_gate(instruction)?;

        if self.agents.len() >= 10 {
            return Err("VRAM quota limit exceeded".to_string());
        }

        let sub_agent_id = format!("sub-agent-{}-{}", target_role, chrono::Utc::now().timestamp_nanos_opt().unwrap_or(0));
        let sub_agent = Agent {
            id: sub_agent_id.clone(),
            name: format!("Specialized {} Agent", target_role),
            role: target_role.to_string(),
            organization_id: "dynamic-delegation".to_string(),
            status: "IDLE".to_string(),
            provider_type: "builtin".to_string(),
        };

        self.agents.insert(sub_agent_id.clone(), sub_agent);

        let msg = Message {
            id: format!("msg-{}", uuid::Uuid::new_v4()),
            from_agent: from_agent_id.to_string(),
            to_agent: sub_agent_id.clone(),
            r#type: "TaskDelegation".to_string(),
            content: format!("Execute Task: {}\nContext: {}", instruction, parent_thread_id),
            occurred_at_unix: chrono::Utc::now().timestamp(),
            meeting_id: String::new(),
        };

        self.publish(msg)?;

        Ok(sub_agent_id)
    }

    pub fn minimax_api_key(&self) -> &str {
        &self.minimax_api_key
    }

    pub fn advertise_capabilities(&self, caps: AgentCapabilities) -> Result<(), String> {
        let _ = self.caps_tx.send(caps);
        Ok(())
    }

    pub fn subscribe_capabilities(&self) -> broadcast::Receiver<AgentCapabilities> {
        self.caps_tx.subscribe()
    }

    pub fn publish_mesh_event(&self, event: MeshEvent) -> Result<(), String> {
        let tx = self.mesh_events.entry(event.topic.clone()).or_insert_with(|| {
            let (tx, _) = broadcast::channel(100);
            tx
        });
        let _ = tx.send(event);
        Ok(())
    }

    pub fn subscribe_mesh_events(&self, topic: String) -> broadcast::Receiver<MeshEvent> {
        let tx = self.mesh_events.entry(topic).or_insert_with(|| {
            let (tx, _) = broadcast::channel(100);
            tx
        });
        tx.subscribe()
    }

    pub fn publish_teammate_event(&self, channel: String, event: TeammateMeshEvent) -> Result<(), String> {
        let tx = self.teammate_events.entry(channel).or_insert_with(|| {
            let (tx, _) = broadcast::channel(100);
            tx
        });
        let _ = tx.send(event);
        Ok(())
    }

    pub fn subscribe_teammate_mesh(&self, channel: String) -> broadcast::Receiver<TeammateMeshEvent> {
        let tx = self.teammate_events.entry(channel).or_insert_with(|| {
            let (tx, _) = broadcast::channel(100);
            tx
        });
        tx.subscribe()
    }

    pub fn tracker(&self) -> &Tracker {
        &self.tracker
    }

    pub fn task_manager(&self) -> &TaskManager {
        &self.task_manager
    }

    pub fn scheduler(&self) -> &Scheduler {
        &self.scheduler
    }

    pub fn log_event(&self, event: serde_json::Value) {
        let _ = self.event_log_tx.try_send(event);
    }

    pub fn append_recent_event(&self, event: HubEvent) {
        let mut recent = self.recent_events.write().unwrap();
        recent.push(event);
        if recent.len() > 200 {
            recent.remove(0);
        }
    }

    pub fn recent_events(&self, limit: usize) -> Vec<HubEvent> {
        let recent = self.recent_events.read().unwrap();
        let count = recent.len().min(limit);
        recent.iter().rev().take(count).cloned().collect()
    }

    pub fn sanitize_hub_event(&self, raw: serde_json::Value) -> HubEvent {
        let event_type = raw["type"].as_str().unwrap_or("unknown").to_string();
        HubEvent {
            r#type: event_type,
            payload: raw.to_string(),
            occurred_at: Utc::now(),
        }
    }

    pub fn start_token_burn_rate_worker(self: std::sync::Arc<Self>) {
        if self.get_token_usage.is_none() {
            return;
        }
        
        tokio::spawn(async move {
            let mut interval = tokio::time::interval(tokio::time::Duration::from_secs(60));
            loop {
                interval.tick().await;
                self.calculate_token_burn_rate();
            }
        });
    }

    fn calculate_token_burn_rate(&self) {
        if let Some(ref get_usage) = self.get_token_usage {
            let usages = get_usage();
            
            let mut active_orgs = HashMap::new();
            
            for (org_id, total_tokens) in usages {
                active_orgs.insert(org_id.clone(), true);
                if total_tokens > 0 {
                    let mut hist = self.token_usage_history.entry(org_id.clone()).or_insert_with(Vec::new);
                    hist.push(total_tokens);
                    
                    if hist.len() > 5 {
                        hist.remove(0);
                    }
                    
                    if hist.len() > 1 {
                        let _rate = (hist[hist.len() - 1] - hist[0]) as f64 / (hist.len() - 1) as f64;
                        // Removed noisy log for signal hygiene
                    }
                } else {
                    self.token_usage_history.remove(&org_id);
                }
            }
            
            self.token_usage_history.retain(|org_id, _| active_orgs.contains_key(org_id));
        }
    }

    pub fn tool_parameter_auto_correction(&self, event_id: &str, agent_id: &str, payload: &[u8]) -> Result<(), String> {
        if !self.auto_cor_track.insert(event_id.to_string()) {
            return Err("event already being processed".to_string());
        }
        
        struct Guard<'a>(&'a Hub, &'a str);
        impl<'a> Drop for Guard<'a> {
            fn drop(&mut self) {
                self.0.auto_cor_track.remove(self.1);
            }
        }
        let _guard = Guard(self, event_id);
        
        let mut temp: HashMap<String, serde_json::Value> = serde_json::from_slice(payload).map_err(|e| e.to_string())?;
        
        let mut corrected = false;
        for (_k, v) in temp.iter_mut() {
            if let Some(s) = v.as_str() {
                if let Ok(n) = s.parse::<i64>() {
                    if n.to_string() == s {
                        *v = serde_json::Value::Number(n.into());
                        corrected = true;
                    }
                }
            }
        }
        
        self.log_event(serde_json::json!({
            "event_id": event_id,
            "agent_id": agent_id,
            "type": "ToolParameterAutoCorrection",
            "payload": temp,
            "corrected": corrected,
        }));
        
        Ok(())
    }

    pub fn fork_agent(self: std::sync::Arc<Self>, parent_id: &str, directive: &str) -> Result<String, String> {
        let parent = self.agents.get(parent_id).ok_or_else(|| format!("parent agent not found: {}", parent_id))?.value().clone();
        
        let child_id = format!("{}-fork-{}", parent_id, uuid::Uuid::new_v4());
        let child = Agent {
            id: child_id.clone(),
            name: format!("{} (Fork)", parent.name),
            role: parent.role.clone(),
            organization_id: parent.organization_id.clone(),
            status: "IDLE".to_string(),
            provider_type: parent.provider_type.clone(),
        };
        
        self.agents.insert(child_id.clone(), child);
        
        // Copy history
        let history = self.inbox.get(parent_id).map(|r| r.value().clone()).unwrap_or_default();
        
        for msg in history {
            let mut child_msg = msg.clone();
            child_msg.id = format!("msg-{}", uuid::Uuid::new_v4());
            child_msg.to_agent = child_id.clone();
            self.clone().publish(child_msg)?;
        }
        
        // Send directive
        let directive_msg = Message {
            id: format!("msg-{}", uuid::Uuid::new_v4()),
            from_agent: "SYSTEM".to_string(),
            to_agent: child_id.clone(),
            r#type: "TaskAssignment".to_string(),
            content: format!("<task-notification>\nDirective: {}\n</task-notification>", directive),
            occurred_at_unix: Utc::now().timestamp(),
            meeting_id: String::new(),
        };
        
        self.clone().publish(directive_msg)?;
        
        Ok(child_id)
    }

    pub async fn check_health(&self) -> Result<serde_json::Value, String> {
        let start = std::time::Instant::now();
        let db_ping = match sqlx::query("SELECT 1").execute(&self.pool).await {
            Ok(_) => start.elapsed().as_millis() as u64,
            Err(_) => 0,
        };

        let mode = if std::env::var("OHC_STANDALONE").unwrap_or_default() == "true" {
            "standalone"
        } else {
            "cloud"
        };

        let status = if db_ping > 0 { "healthy" } else { "degraded" };
        let mesh_active = db_ping > 0;
        let cloud_connected = mode != "standalone";

        let mission_sync_backlog: i64 = match sqlx::query_scalar("SELECT count(*) FROM agent_missions WHERE status IN ('PENDING', 'BURSTING')")
            .fetch_one(&self.pool)
            .await
        {
            Ok(count) => count,
            Err(_) => 0,
        };

        Ok(serde_json::json!({
            "mode": mode,
            "status": status,
            "db_ping_ms": db_ping,
            "mesh_active": mesh_active,
            "cloud_connected": cloud_connected,
            "mission_sync_backlog": mission_sync_backlog,
        }))
    }
}



fn get_feature_regex() -> &'static Regex {
    static REGEX: OnceLock<Regex> = OnceLock::new();
    REGEX.get_or_init(|| Regex::new(r"\[Feature:\s*([^\]]+)\]").unwrap())
}

pub fn check_documentation_gate(content: &str) -> Result<(), String> {
    let regex = get_feature_regex();
    
    if let Some(caps) = regex.captures(content) {
        let feature_name: &str = caps.get(1).unwrap().as_str();
        let base_dir = format!("docs/features/{}", feature_name);
        
        let required_files = ["design-doc.md", "cuj.md", "test-plan.md"];
        for file in &required_files {
            let path = std::path::Path::new(&base_dir).join(file);
            if !path.exists() {
                return Err(format!("missing required documentation: {}", path.display()));
            }
        }
    }
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::sync::mpsc;

    #[tokio::test]
    async fn test_check_health() {
        // Skip test if no database is available
        if std::env::var("DATABASE_URL").is_err() {
            return;
        }

        let db_url = std::env::var("DATABASE_URL").unwrap();
        // Since test db is likely unmigrated/empty, we connect lazily
        let pool = sqlx::postgres::PgPoolOptions::new()
            .acquire_timeout(std::time::Duration::from_millis(50))
            .connect_lazy(&db_url)
            .unwrap();
        let (tx, _) = mpsc::channel(100);
        let hub = Hub::new(tx, pool);

        let health = hub.check_health().await.unwrap();

        // When lazily connected, if DB doesn't exist, status might be degraded,
        // or we might get an error depending on how check_health handles failure.
        // In our check_health, failure to query SELECT 1 results in db_ping = 0.
        // We just ensure the response contains the fields we expect.
        assert!(health.get("status").is_some());
        assert!(health.get("db_ping_ms").is_some());
        assert!(health.get("mission_sync_backlog").is_some());
    }
}
