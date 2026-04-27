use async_trait::async_trait;
use std::sync::Arc;
use std::sync::RwLock;
use std::collections::HashMap;
use crate::agents::agent::Transport;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq, Eq, Hash)]
pub enum ProviderType {
    Claude,
    Gemini,
    OpenCode,
    OpenClaw,
    IronClaw,
    Builtin,
    Scout,
    MiniMaxi,
}

impl std::fmt::Display for ProviderType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            ProviderType::Claude => "claude",
            ProviderType::Gemini => "gemini",
            ProviderType::OpenCode => "opencode",
            ProviderType::OpenClaw => "openclaw",
            ProviderType::IronClaw => "ironclaw",
            ProviderType::Builtin => "builtin",
            ProviderType::Scout => "scout",
            ProviderType::MiniMaxi => "minimaxi",
        };
        write!(f, "{}", s)
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, Default)]
pub struct Credentials {
    pub api_key: String,
    pub oauth_token: String,
    pub extra: HashMap<String, String>,
}

impl Credentials {
    pub fn is_empty(&self) -> bool {
        self.api_key.is_empty() && self.oauth_token.is_empty()
    }
}

#[async_trait]
pub trait Provider: Send + Sync {
    fn provider_type(&self) -> ProviderType;
    fn description(&self) -> String;
    fn supported_roles(&self) -> Vec<String>;
    fn authenticate(&self, creds: Credentials) -> Result<(), String>;
    fn get_credentials(&self) -> Credentials;
    fn is_authenticated(&self) -> bool;
    async fn run_in_isolation(&self, worktree: &str, transport: Option<Arc<dyn Transport>>) -> Result<(), String>;
}

pub struct BaseProvider {
    cred: RwLock<Credentials>,
}

impl BaseProvider {
    pub fn new() -> Self {
        BaseProvider {
            cred: RwLock::new(Credentials::default()),
        }
    }

    pub fn store(&self, cred: Credentials) {
        let mut c = self.cred.write().unwrap();
        *c = cred;
    }

    pub fn load(&self) -> Credentials {
        let c = self.cred.read().unwrap();
        c.clone()
    }
}

async fn execute_in_isolation(agent_type: &str, worktree: &str, transport: Option<Arc<dyn Transport>>) -> Result<(), String> {
    let isolation_sandbox_id = format!("sandbox-{}-{}", agent_type, chrono::Utc::now().timestamp_nanos_opt().unwrap_or(0));

    let status_msg = serde_json::json!({
        "agent":    agent_type,
        "status":   "RUNNING",
        "worktree": worktree,
        "sandbox":  isolation_sandbox_id,
    });

    if let Some(t) = transport.as_ref() {
        t.send(status_msg.to_string().as_bytes()).await?;
    }

    let output_msg = serde_json::json!({
        "agent":   agent_type,
        "stream":  "stdout",
        "content": format!("Execution started in isolated worktree {}", worktree),
    });

    if let Some(t) = transport.as_ref() {
        let _ = t.send(output_msg.to_string().as_bytes()).await;
    }

    let end_msg = serde_json::json!({
        "agent":  agent_type,
        "status": "COMPLETED",
    });

    if let Some(t) = transport.as_ref() {
        let _ = t.send(end_msg.to_string().as_bytes()).await;
    }

    Ok(())
}

// Implementations

pub struct ClaudeProvider {
    base: BaseProvider,
}

impl ClaudeProvider {
    pub fn new() -> Self {
        ClaudeProvider { base: BaseProvider::new() }
    }
}

#[async_trait]
impl Provider for ClaudeProvider {
    fn provider_type(&self) -> ProviderType { ProviderType::Claude }
    fn description(&self) -> String { "Anthropic Claude Code".to_string() }
    fn supported_roles(&self) -> Vec<String> { vec!["SOFTWARE_ENGINEER".to_string()] }
    fn authenticate(&self, creds: Credentials) -> Result<(), String> {
        if creds.api_key.is_empty() {
            return Err("API key required".to_string());
        }
        self.base.store(creds);
        Ok(())
    }
    fn get_credentials(&self) -> Credentials { self.base.load() }
    fn is_authenticated(&self) -> bool { !self.base.load().is_empty() }
    async fn run_in_isolation(&self, worktree: &str, transport: Option<Arc<dyn Transport>>) -> Result<(), String> {
        execute_in_isolation(&self.provider_type().to_string(), worktree, transport).await
    }
}

// Add more providers here as needed, following the same pattern.
