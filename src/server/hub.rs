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
use std::sync::Arc;
use redis::Commands;
use crate::services::billing::auditor::CostAuditor;
use crate::pricing::calculator::CostConfig;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HubEvent {
    pub r#type: String,
    pub payload: String,
    pub occurred_at: DateTime<Utc>,
}

pub struct Hub {
    telemetry_tx: tokio::sync::mpsc::UnboundedSender<crate::services::billing::auditor::AuditEvent>,
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
    cost_auditor: Arc<CostAuditor>,
    recent_events: RwLock<Vec<HubEvent>>,
    token_usage_history: RwLock<HashMap<String, Vec<i64>>>,
    get_token_usage: Option<Box<dyn Fn() -> HashMap<String, i64> + Send + Sync>>,
    auto_cor_track: RwLock<std::collections::HashSet<String>>,
    event_log_tx: mpsc::Sender<serde_json::Value>,
    pub(crate) pool: sqlx::PgPool,
    redis_client: Option<redis::Client>,
    agent_cache: RwLock<Option<Arc<Vec<Agent>>>>,
    meetings_cache: RwLock<Option<Arc<Vec<MeetingRoom>>>>,
}

impl Hub {
    pub fn new(event_log_tx: mpsc::Sender<serde_json::Value>, pool: sqlx::PgPool) -> Self {
        let minimax_api_key = std::env::var("MINIMAX_API_KEY").unwrap_or_default();
        let (caps_tx, _) = broadcast::channel(100);
        let redis_client = if std::env::var("STANDALONE_MODE").unwrap_or_else(|_| "true".to_string()) != "true" {
            std::env::var("REDIS_URL").ok().and_then(|url| redis::Client::open(url).ok())
        } else {
            None
        };

        let (telemetry_tx, mut telemetry_rx) = tokio::sync::mpsc::unbounded_channel::<crate::services::billing::auditor::AuditEvent>();
        let pool_clone = pool.clone();
        let cost_auditor = Arc::new({
            let mut a = CostAuditor::new(CostConfig::default());
            a.set_telemetry_tx(telemetry_tx.clone());
            a
        });

        let cost_auditor_clone = cost_auditor.clone();
        tokio::spawn(async move {
            while let Some(event) = telemetry_rx.recv().await {
                let cost = cost_auditor_clone.record_event(event.clone());

                let labels = serde_json::json!({
                    "agent_id": event.agent_id,
                    "input_tokens": event.input_tokens,
                    "output_tokens": event.output_tokens,
                    "cached_input_tokens": event.cached_input_tokens,
                    "local_embedding_tokens": event.local_embedding_tokens,
                    "cost_usd": cost,
                });

                let _ = crate::telemetry::buffer_metric(&pool_clone, "ohc_token_usage_total", "counter", event.output_tokens as f32, labels.clone()).await;

                // Blueprint: track cost in cents
                let cost_cents = (cost * 100.0) as f32;
                let _ = crate::telemetry::buffer_metric(&pool_clone, "ohc_mission_cost_cents", "counter", cost_cents, labels).await;
            }
        });

        Hub {
            telemetry_tx: telemetry_tx.clone(),
            agents: RwLock::new(HashMap::new()),
            agent_cache: RwLock::new(None),
            meetings: RwLock::new(HashMap::new()),
            meetings_cache: RwLock::new(None),
            inbox: RwLock::new(HashMap::new()),
            subs: RwLock::new(HashMap::new()),
            minimax_api_key,
            caps_tx,
            pool,
            mesh_events: RwLock::new(HashMap::new()),
            teammate_events: RwLock::new(HashMap::new()),
            tracker: Tracker::new(),
            task_manager: TaskManager::new(),
            scheduler: Scheduler::new(),
            cost_auditor,
            recent_events: RwLock::new(Vec::new()),
            token_usage_history: RwLock::new(HashMap::new()),
            get_token_usage: None,
            auto_cor_track: RwLock::new(std::collections::HashSet::new()),
            event_log_tx,
            redis_client,
        }
    }

    fn invalidate_agent_cache(&self) {
        *self.agent_cache.write().unwrap() = None;
        if let Some(client) = &self.redis_client {
            if let Ok(mut conn) = client.get_connection() {
                let _: Result<(), _> = conn.del("hub:agents");
            }
        }
    }

    fn invalidate_meetings_cache(&self) {
        *self.meetings_cache.write().unwrap() = None;
        if let Some(client) = &self.redis_client {
            if let Ok(mut conn) = client.get_connection() {
                let _: Result<(), _> = conn.del("hub:meetings");
            }
        }
    }

    pub fn get_cost_auditor(&self) -> Arc<CostAuditor> {
        self.cost_auditor.clone()
    }

    pub fn get_telemetry_tx(&self) -> tokio::sync::mpsc::UnboundedSender<crate::services::billing::auditor::AuditEvent> {
        self.telemetry_tx.clone()
    }

    pub fn register_agent(&self, agent: Agent) {
        let mut agents = self.agents.write().unwrap();
        agents.insert(agent.id.clone(), agent);
        self.invalidate_agent_cache();
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
        self.invalidate_agent_cache();
    }

    pub fn get_agents(&self) -> Arc<Vec<Agent>> {
        {
            let cache = self.agent_cache.read().unwrap();
            if let Some(agents) = &*cache {
                return Arc::clone(agents);
            }
        }

        if let Some(client) = &self.redis_client {
            if let Ok(mut conn) = client.get_connection() {
                if let Ok(Some(data)) = conn.get::<_, Option<String>>("hub:agents") {
                    if let Ok(agents) = serde_json::from_str::<Vec<Agent>>(&data) {
                        let arc = Arc::new(agents);
                        *self.agent_cache.write().unwrap() = Some(Arc::clone(&arc));
                        return arc;
                    }
                }
            }
        }

        let agents = self.agents.read().unwrap();
        let mut agents_vec: Vec<Agent> = agents.values().cloned().collect();
        agents_vec.sort_by(|a, b| a.id.cmp(&b.id));

        let arc = Arc::new(agents_vec);
        *self.agent_cache.write().unwrap() = Some(Arc::clone(&arc));

        if let Some(client) = &self.redis_client {
            if let Ok(mut conn) = client.get_connection() {
                if let Ok(json) = serde_json::to_string(&*arc) {
                    let _: Result<(), _> = conn.set_ex("hub:agents", json, 3600);
                }
            }
        }

        arc
    }

    pub fn get_agents_by_org(&self, org_id: &str) -> Vec<Agent> {
        let agents = self.agents.read().unwrap();
        let mut agents_vec: Vec<Agent> = agents.values()
            .filter(|a| a.organization_id == org_id || a.id.starts_with(&format!("{}-", org_id)))
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
        
        self.invalidate_agent_cache();
        self.invalidate_meetings_cache();

        meeting
    }

    pub fn publish(self: std::sync::Arc<Self>, msg: Message) -> Result<(), String> {
        let mut inbox = self.inbox.write().unwrap();
        let mut meetings = self.meetings.write().unwrap();
        let subs = self.subs.read().unwrap();
        
        let to_agent = msg.to_agent.clone();

        // Check rate limiting
        let tenant_id = msg.to_agent.split("-").next().unwrap_or("default").to_string();
        let agent_id = msg.to_agent.clone();
        let tracker = self.tracker.clone();
        tokio::spawn(async move {
            if let Ok(limit_status) = tracker.check_rate_limit(&tenant_id, &agent_id).await {
                if limit_status.soft_limit_reached {
                    tracing::warn!("Rate limit warning: {:?}", limit_status.user_message);
                }
            }
        });
        
        // Add to recipient's inbox
        let messages = inbox.entry(to_agent.clone()).or_insert_with(Vec::new);
        messages.push(msg.clone());
        
        // Add to meeting transcript if applicable
        if !msg.meeting_id.is_empty() {
            if let Some(meeting) = meetings.get_mut(&msg.meeting_id) {
                meeting.transcript.push(msg.clone());
                self.invalidate_meetings_cache();
                
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
                                    hub.invalidate_meetings_cache();
                                }
                            }
                            Err(e) => tracing::error!("Summarization failed: {}", e),
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

    pub fn get_meetings(&self) -> Arc<Vec<MeetingRoom>> {
        {
            let cache = self.meetings_cache.read().unwrap();
            if let Some(meetings) = &*cache {
                return Arc::clone(meetings);
            }
        }

        if let Some(client) = &self.redis_client {
            if let Ok(mut conn) = client.get_connection() {
                if let Ok(Some(data)) = conn.get::<_, Option<String>>("hub:meetings") {
                    if let Ok(meetings) = serde_json::from_str::<Vec<MeetingRoom>>(&data) {
                        let arc = Arc::new(meetings);
                        *self.meetings_cache.write().unwrap() = Some(Arc::clone(&arc));
                        return arc;
                    }
                }
            }
        }

        let meetings = self.meetings.read().unwrap();
        let meetings_vec: Vec<MeetingRoom> = meetings.values().cloned().collect();

        let arc = Arc::new(meetings_vec);
        *self.meetings_cache.write().unwrap() = Some(Arc::clone(&arc));

        if let Some(client) = &self.redis_client {
            if let Ok(mut conn) = client.get_connection() {
                if let Ok(json) = serde_json::to_string(&*arc) {
                    let _: Result<(), _> = conn.set_ex("hub:meetings", json, 3600);
                }
            }
        }

        arc
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
        
        if !agents.contains_key(from_agent_id) {
            return Err("sender agent is not registered".to_string());
        }

        if agents.len() >= 10 {
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

        agents.insert(sub_agent_id.clone(), sub_agent);
        self.invalidate_agent_cache();
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
        let redacted_raw = crate::telemetry::redact_interface_pii(raw);
        HubEvent {
            r#type: event_type,
            payload: redacted_raw.to_string(),
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
                self.calculate_token_burn_rate().await;
            }
        });
    }

    async fn calculate_token_burn_rate(&self) {
        if let Some(ref get_usage) = self.get_token_usage {
            let usages = get_usage();
            let mut forecasts_to_record = Vec::new();
            
            {
                let mut history = self.token_usage_history.write().unwrap();
                let mut active_orgs = HashMap::new();

                for (org_id, total_tokens) in usages {
                    active_orgs.insert(org_id.clone(), true);
                    if total_tokens > 0 {
                        let hist = history.entry(org_id.clone()).or_insert_with(Vec::new);
                        hist.push(total_tokens);

                        if hist.len() > 5 {
                            hist.remove(0);
                        }

                        if hist.len() > 1 {
                            let rate = (hist[hist.len() - 1] - hist[0]) as f64 / (hist.len() - 1) as f64;
                            let forecast = hist.last().unwrap() + (rate * 43200.0) as i64;
                            forecasts_to_record.push((org_id.clone(), forecast as f32));
                        }
                    } else {
                        history.remove(&org_id);
                    }
                }

                history.retain(|org_id, _| active_orgs.contains_key(org_id));
            }
            
            for (org_id, forecast) in forecasts_to_record {
                let _ = crate::telemetry::record_token_usage_forecast(&self.pool, &org_id, forecast).await;
            }
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
            organization_id: parent.organization_id.clone(),
            status: "IDLE".to_string(),
            provider_type: parent.provider_type.clone(),
        };
        
        agents.insert(child_id.clone(), child);
        self.invalidate_agent_cache();
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

        let ping_future = async {
            match sqlx::query("SELECT 1").execute(&self.pool).await {
                Ok(_) => start.elapsed().as_millis() as u64,
                Err(_) => 0,
            }
        };
        let backlog_future = sqlx::query_scalar::<_, i64>("SELECT count(*) FROM agent_missions WHERE status IN ('PENDING', 'BURSTING')").fetch_one(&self.pool);
        let sync_queue_future = sqlx::query_scalar::<_, i64>("SELECT count(*) FROM agent_missions WHERE _sync_status = 'pending'").fetch_one(&self.pool);

        let (db_ping, backlog_res, sync_queue_res) = tokio::join!(ping_future, backlog_future, sync_queue_future);

        let mission_sync_backlog = backlog_res.unwrap_or(0);
        let local_to_cloud_sync_queue = sync_queue_res.unwrap_or(0);

        let mode = if std::env::var("OHC_STANDALONE").unwrap_or_default() == "true" {
            "standalone"
        } else {
            "cloud"
        };

        let status = if db_ping > 0 { "healthy" } else { "degraded" };
        let mesh_active = db_ping > 0;
        let cloud_connected = mode != "standalone";

        let hybrid_mode_ready = if mode == "standalone" {
            std::env::var("DATABASE_URL").is_ok() && db_ping > 0
        } else {
            db_ping > 0
        };

        Ok(serde_json::json!({
            "mode": mode,
            "status": status,
            "db_ping_ms": db_ping,
            "mesh_active": mesh_active,
            "cloud_connected": cloud_connected,
            "mission_sync_backlog": mission_sync_backlog,
            "hybrid_mode_ready": hybrid_mode_ready,
            "local_to_cloud_sync_queue": local_to_cloud_sync_queue,
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
    async fn test_sanitize_hub_event_redaction() {
        if std::env::var("DATABASE_URL").is_err() {
            return;
        }
        let db_url = std::env::var("DATABASE_URL").unwrap();
        let pool = sqlx::postgres::PgPoolOptions::new()
            .after_release(|conn, _meta| { Box::pin(async move { use sqlx::Executor; conn.execute("DISCARD ALL").await?; Ok(true) }) })
            .acquire_timeout(std::time::Duration::from_millis(50))
            .connect_lazy(&db_url)
            .unwrap();
        let (tx, _) = mpsc::channel(100);
        let hub = std::sync::Arc::new(Hub::new(tx, pool));

        let raw = serde_json::json!({
            "type": "TestEvent",
            "password": "secret-password",
            "email": "test@example.com",
            "nested": {
                "auth_token": "token123"
            }
        });

        let sanitized = hub.sanitize_hub_event(raw);
        let payload: serde_json::Value = serde_json::from_str(&sanitized.payload).unwrap();

        assert_eq!(payload["password"], "[REDACTED]");
        assert_eq!(payload["email"], "[REDACTED]");
        assert_eq!(payload["nested"]["auth_token"], "[REDACTED]");
    }

    #[tokio::test]
    async fn test_cache_invalidation() {
        if std::env::var("DATABASE_URL").is_err() {
            return;
        }
        let db_url = std::env::var("DATABASE_URL").unwrap();
        let pool = sqlx::postgres::PgPoolOptions::new()
            .after_release(|conn, _meta| { Box::pin(async move { use sqlx::Executor; conn.execute("DISCARD ALL").await?; Ok(true) }) })
            .acquire_timeout(std::time::Duration::from_millis(50))
            .connect_lazy(&db_url)
            .unwrap();
        let (tx, _) = mpsc::channel(100);
        let hub = std::sync::Arc::new(Hub::new(tx, pool));

        // 1. Initial get caches empty state
        let agents = hub.get_agents();
        assert_eq!(agents.len(), 0);

        // Cache should be populated
        assert!(hub.agent_cache.read().unwrap().is_some());

        // 2. Registering an agent invalidates the cache
        hub.register_agent(Agent {
            id: "agent1".to_string(),
            name: "Agent 1".to_string(),
            role: "test".to_string(),
            organization_id: "org1".to_string(),
            status: "IDLE".to_string(),
            provider_type: "test".to_string(),
        });
        assert!(hub.agent_cache.read().unwrap().is_none());

        // 3. Get agents caches again
        let agents = hub.get_agents();
        assert_eq!(agents.len(), 1);
        assert!(hub.agent_cache.read().unwrap().is_some());

        // 4. Fire agent invalidates
        hub.fire_agent("agent1");
        assert!(hub.agent_cache.read().unwrap().is_none());

        // 5. Open meeting invalidates both caches
        let meetings = hub.get_meetings();
        assert_eq!(meetings.len(), 0);
        assert!(hub.meetings_cache.read().unwrap().is_some());

        hub.open_meeting("meeting1".to_string(), vec![], "agenda".to_string());
        assert!(hub.meetings_cache.read().unwrap().is_none());
        assert!(hub.agent_cache.read().unwrap().is_none());

        // 6. Publish invalidates meeting cache
        let meetings = hub.get_meetings();
        assert_eq!(meetings.len(), 1);
        assert!(hub.meetings_cache.read().unwrap().is_some());

        let _ = hub.clone().publish(Message {
            id: "msg1".to_string(),
            from_agent: "sys".to_string(),
            to_agent: "all".to_string(),
            r#type: "test".to_string(),
            content: "test".to_string(),
            occurred_at_unix: 0,
            meeting_id: "meeting1".to_string(),
        });
        assert!(hub.meetings_cache.read().unwrap().is_none());
    }
    #[tokio::test]
    async fn test_delegate_sub_task_invalid_sender() {
        if std::env::var("DATABASE_URL").is_err() {
            return;
        }

        let db_url = std::env::var("DATABASE_URL").unwrap();
        let pool = sqlx::postgres::PgPoolOptions::new()
            .after_release(|conn, _meta| { Box::pin(async move { use sqlx::Executor; conn.execute("DISCARD ALL").await?; Ok(true) }) })
            .acquire_timeout(std::time::Duration::from_millis(50))
            .connect_lazy(&db_url)
            .unwrap();
        let (tx, _) = mpsc::channel(100);
        let hub = std::sync::Arc::new(Hub::new(tx, pool));

        let res = hub.delegate_sub_task(
            "non_existent_agent",
            "developer",
            "fix the bug",
            "thread123",
        );
        assert!(res.is_err());
        assert_eq!(res.unwrap_err(), "sender agent is not registered");
    }

    #[tokio::test]
    async fn test_check_health() {
        // Skip test if no database is available
        if std::env::var("DATABASE_URL").is_err() {
            return;
        }

        let db_url = std::env::var("DATABASE_URL").unwrap();
        // Since test db is likely unmigrated/empty, we connect lazily
        let pool = sqlx::postgres::PgPoolOptions::new()
            .after_release(|conn, _meta| { Box::pin(async move { use sqlx::Executor; conn.execute("DISCARD ALL").await?; Ok(true) }) })
            .acquire_timeout(std::time::Duration::from_millis(50))
            .connect_lazy(&db_url)
            .unwrap();
        let (tx, _) = mpsc::channel(100);
        let hub = std::sync::Arc::new(Hub::new(tx, pool));

        let health = hub.check_health().await.unwrap();

        // When lazily connected, if DB doesn't exist, status might be degraded,
        // or we might get an error depending on how check_health handles failure.
        // In our check_health, failure to query SELECT 1 results in db_ping = 0.
        // We just ensure the response contains the fields we expect.
        assert!(health.get("status").is_some());
        assert!(health.get("db_ping_ms").is_some());
        assert!(health.get("mission_sync_backlog").is_some());
        assert!(health.get("hybrid_mode_ready").is_some());
        assert!(health.get("local_to_cloud_sync_queue").is_some());
    }
}
