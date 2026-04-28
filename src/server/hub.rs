use crate::db::TenantPool;
use std::collections::HashMap;
use std::sync::RwLock;
use std::sync::OnceLock;
use regex::Regex;
use crate::ohc::orchestration::{Agent, MeetingRoom, Message, AgentCapabilities, MeshEvent, TeammateMeshEvent};
use tokio::sync::broadcast;
use tokio::sync::mpsc;
use crate::billing::Tracker;
use crate::tasks::TaskManager;
use crate::scheduler::Scheduler;
use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HubEvent {
    pub r#type: String,
    pub payload: String,
    pub occurred_at: DateTime<Utc>,
}

pub struct Hub {
    agents: RwLock<HashMap<String, Agent>>,
    meetings: RwLock<HashMap<String, MeetingRoom>>,
    inbox: RwLock<HashMap<String, Vec<Message>>>,
    subs: RwLock<HashMap<String, broadcast::Sender<Message>>>,
    minimax_api_key: String,
    caps_tx: broadcast::Sender<AgentCapabilities>,
    mesh_events: RwLock<HashMap<String, broadcast::Sender<MeshEvent>>>,
    teammate_events: RwLock<HashMap<String, broadcast::Sender<TeammateMeshEvent>>>,
    tracker: Tracker,
    task_manager: TaskManager,
    scheduler: Scheduler,
    recent_events: RwLock<Vec<HubEvent>>,
    token_usage_history: RwLock<HashMap<String, Vec<i64>>>,
    get_token_usage: Option<Box<dyn Fn() -> HashMap<String, i64> + Send + Sync>>,
    auto_cor_track: RwLock<std::collections::HashSet<String>>,
    event_log_tx: mpsc::Sender<serde_json::Value>,
    pub(crate) pool: sqlx::PgPool,
    pub(crate) wizard_state: RwLock<HashMap<String, serde_json::Value>>,
}

impl Hub {
    pub fn new(event_log_tx: mpsc::Sender<serde_json::Value>, pool: sqlx::PgPool) -> Self {
        let minimax_api_key = std::env::var("MINIMAX_API_KEY").unwrap_or_default();
        let (caps_tx, _) = broadcast::channel(100);
        Hub {
            agents: RwLock::new(HashMap::new()),
            meetings: RwLock::new(HashMap::new()),
            inbox: RwLock::new(HashMap::new()),
            subs: RwLock::new(HashMap::new()),
            minimax_api_key,
            caps_tx,
            pool,
            mesh_events: RwLock::new(HashMap::new()),
            teammate_events: RwLock::new(HashMap::new()),
            tracker: Tracker::new(),
            task_manager: TaskManager::new(event_log_tx.clone()),
            scheduler: Scheduler::new(),
            recent_events: RwLock::new(Vec::new()),
            token_usage_history: RwLock::new(HashMap::new()),
            get_token_usage: None,
            auto_cor_track: RwLock::new(std::collections::HashSet::new()),
            event_log_tx,
            wizard_state: RwLock::new(HashMap::new()),
        }
    }

    pub fn register_agent(&self, agent: Agent) {
        let mut agents = self.agents.write().unwrap();
        agents.insert(agent.id.clone(), agent);
    }

    pub fn get_agent(&self, id: &str) -> Option<Agent> {
        let agents = self.agents.read().unwrap();
        agents.get(id).cloned()
    }

    pub fn get_agents_count(&self) -> usize {
        let agents = self.agents.read().unwrap();
        agents.len()
    }

    pub fn fire_agent(&self, id: &str) {
        let mut agents = self.agents.write().unwrap();
        let mut inbox = self.inbox.write().unwrap();
        
        agents.remove(id);
        inbox.remove(id);
    }

    pub fn get_agents(&self) -> Vec<Agent> {
        let agents = self.agents.read().unwrap();
        let mut agents_vec: Vec<Agent> = agents.values().cloned().collect();
        agents_vec.sort_by(|a, b| a.id.cmp(&b.id));
        agents_vec
    }

    pub fn get_agents_by_org(&self, tenant_id: &str) -> Vec<Agent> {
        let agents = self.agents.read().unwrap();
        let mut agents_vec: Vec<Agent> = agents.values()
            .filter(|a| a.tenant_id == tenant_id || a.id.starts_with(&format!("{}-", tenant_id)))
            .cloned()
            .collect();
        agents_vec.sort_by(|a, b| a.id.cmp(&b.id));
        agents_vec
    }

    pub fn open_meeting(&self, id: String, participants: Vec<String>, agenda: String) -> MeetingRoom {
        let mut meetings = self.meetings.write().unwrap();
        let mut agents = self.agents.write().unwrap();
        
        let meeting = MeetingRoom {
            id: id.clone(),
            agenda,
            participants: participants.clone(),
            transcript: vec![],
        };
        
        meetings.insert(id, meeting.clone());
        
        for participant in participants {
            if let Some(agent) = agents.get_mut(&participant) {
                agent.status = "IN_MEETING".to_string();
            }
        }
        
        meeting
    }

    pub fn publish(self: std::sync::Arc<Self>, msg: Message) -> Result<(), String> {
        let mut inbox = self.inbox.write().unwrap();
        let mut meetings = self.meetings.write().unwrap();
        let subs = self.subs.read().unwrap();
        
        let to_agent = msg.to_agent.clone();
        
        // Add to recipient's inbox
        let messages = inbox.entry(to_agent.clone()).or_insert_with(Vec::new);
        messages.push(msg.clone());
        
        // Add to meeting transcript if applicable
        if !msg.meeting_id.is_empty() {
            if let Some(meeting) = meetings.get_mut(&msg.meeting_id) {
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
                                let mut meetings = hub.meetings.write().unwrap();
                                if let Some(mtg) = meetings.get_mut(&m_id) {
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
        if let Some(tx) = subs.get(&to_agent) {
            let _ = tx.send(msg);
        }
        
        Ok(())
    }

    pub fn get_meetings(&self) -> Vec<MeetingRoom> {
        let meetings = self.meetings.read().unwrap();
        meetings.values().cloned().collect()
    }

    pub fn get_inbox(&self, agent_id: &str) -> Vec<Message> {
        let mut inbox = self.inbox.write().unwrap();
        inbox.remove(agent_id).unwrap_or_default()
    }

    pub fn subscribe(&self, agent_id: String) -> broadcast::Receiver<Message> {
        let mut subs = self.subs.write().unwrap();
        let tx = subs.entry(agent_id).or_insert_with(|| {
            let (tx, _) = broadcast::channel(100); // Buffer of 100
            tx
        });
        tx.subscribe()
    }

    pub fn delegate_task(self: std::sync::Arc<Self>, from_agent_id: String, to_agent_id: String, mut task: Message) -> Result<(), String> {
        check_documentation_gate(&task.content)?;
        
        if !self.agents.read().unwrap().contains_key(&from_agent_id) {
            return Err("sender agent is not registered".to_string());
        }
        if !self.agents.read().unwrap().contains_key(&to_agent_id) {
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

        let mut agents = self.agents.write().unwrap();
        
        if agents.len() >= 10 {
            return Err("VRAM quota limit exceeded".to_string());
        }

        let sub_agent_id = format!("sub-agent-{}-{}", target_role, chrono::Utc::now().timestamp_nanos_opt().unwrap_or(0));
        let sub_agent = Agent {
            id: sub_agent_id.clone(),
            name: format!("Specialized {} Agent", target_role),
            role: target_role.to_string(),
            tenant_id: "dynamic-delegation".to_string(),
            status: "IDLE".to_string(),
            provider_type: "builtin".to_string(),
        };

        agents.insert(sub_agent_id.clone(), sub_agent);
        drop(agents);

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
        let mut mesh_events = self.mesh_events.write().unwrap();
        let tx = mesh_events.entry(event.topic.clone()).or_insert_with(|| {
            let (tx, _) = broadcast::channel(100);
            tx
        });
        let _ = tx.send(event);
        Ok(())
    }

    pub fn subscribe_mesh_events(&self, topic: String) -> broadcast::Receiver<MeshEvent> {
        let mut mesh_events = self.mesh_events.write().unwrap();
        let tx = mesh_events.entry(topic).or_insert_with(|| {
            let (tx, _) = broadcast::channel(100);
            tx
        });
        tx.subscribe()
    }

    pub fn publish_teammate_event(&self, channel: String, event: TeammateMeshEvent) -> Result<(), String> {
        let mut teammate_events = self.teammate_events.write().unwrap();
        let tx = teammate_events.entry(channel).or_insert_with(|| {
            let (tx, _) = broadcast::channel(100);
            tx
        });
        let _ = tx.send(event);
        Ok(())
    }

    pub fn subscribe_teammate_mesh(&self, channel: String) -> broadcast::Receiver<TeammateMeshEvent> {
        let mut teammate_events = self.teammate_events.write().unwrap();
        let tx = teammate_events.entry(channel).or_insert_with(|| {
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
            
            let mut history = self.token_usage_history.write().unwrap();
            let mut active_orgs = HashMap::new();
            
            for (tenant_id, total_tokens) in usages {
                active_orgs.insert(tenant_id.clone(), true);
                if total_tokens > 0 {
                    let hist = history.entry(tenant_id.clone()).or_insert_with(Vec::new);
                    hist.push(total_tokens);
                    
                    if hist.len() > 5 {
                        hist.remove(0);
                    }
                    
                    if hist.len() > 1 {
                        let rate = (hist[hist.len() - 1] - hist[0]) as f64 / (hist.len() - 1) as f64;
                        println!("Telemetry: Token burn rate for {}: {}", tenant_id, rate);
                    }
                } else {
                    history.remove(&tenant_id);
                }
            }
            
            history.retain(|tenant_id, _| active_orgs.contains_key(tenant_id));
        }
    }

    pub fn tool_parameter_auto_correction(&self, event_id: &str, agent_id: &str, payload: &[u8]) -> Result<(), String> {
        let mut auto_cor_track = self.auto_cor_track.write().unwrap();
        if auto_cor_track.contains(event_id) {
            return Err("event already being processed".to_string());
        }
        auto_cor_track.insert(event_id.to_string());
        drop(auto_cor_track);
        
        struct Guard<'a>(&'a Hub, &'a str);
        impl<'a> Drop for Guard<'a> {
            fn drop(&mut self) {
                let mut track = self.0.auto_cor_track.write().unwrap();
                track.remove(self.1);
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
        let mut agents = self.agents.write().unwrap();
        
        let parent = agents.get(parent_id).ok_or_else(|| format!("parent agent not found: {}", parent_id))?.clone();
        
        let child_id = format!("{}-fork-{}", parent_id, uuid::Uuid::new_v4());
        let child = Agent {
            id: child_id.clone(),
            name: format!("{} (Fork)", parent.name),
            role: parent.role.clone(),
            tenant_id: parent.tenant_id.clone(),
            status: "IDLE".to_string(),
            provider_type: parent.provider_type.clone(),
        };
        
        agents.insert(child_id.clone(), child);
        drop(agents); // Release lock before calling publish!
        
        // Copy history
        let history = {
            let inbox = self.inbox.read().unwrap();
            inbox.get(parent_id).cloned().unwrap_or_default()
        };
        
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
        let db_ping = match sqlx::query("SELECT 1").execute(&mut *self.pool.acquire_tenant("system").await.map_err(|e| tonic::Status::internal(e.to_string()))?).await {
            Ok(_) => start.elapsed().as_millis() as u64,
            Err(_) => 0,
        };

        let mode = if std::env::var("OHC_STANDALONE").unwrap_or_default() == "true" {
            "standalone"
        } else {
            "cloud"
        };

        let status = if db_ping > 0 { "healthy" } else { "degraded" };

        Ok(serde_json::json!({
            "mode": mode,
            "status": status,
            "db_ping_ms": db_ping,
            "mesh_active": true, 
            "cloud_connected": true,
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
