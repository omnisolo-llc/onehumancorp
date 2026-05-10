#![allow(dead_code)]

use async_trait::async_trait;
use std::sync::Arc;
use std::sync::RwLock;
use std::collections::HashMap;
#[async_trait]
pub trait Transport: Send + Sync {
    async fn send(&self, message: &[u8]) -> Result<(), String>;
}

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
    async fn run_in_isolation(&self, command: &str, worktree: &str, transport: Option<Arc<dyn Transport>>) -> Result<(), String>;
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

async fn execute_in_isolation(command: &str, agent_type: &str, worktree: &str, transport: Option<Arc<dyn Transport>>) -> Result<(), String> {
    use crate::harness::IsolationStrategy;

    let strategy = crate::harness::ProcessIsolationStrategy::new();
    // Use the passed command directly

    strategy.run_in_isolation(&command, agent_type, worktree, transport).await
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
    fn description(&self) -> String { "Anthropic Claude Code — advanced coding and reasoning agent backed by Claude Sonnet/Opus".to_string() }
    fn supported_roles(&self) -> Vec<String> {
        vec![
            "SOFTWARE_ENGINEER".to_string(),
            "SECURITY_ENGINEER".to_string(),
            "QA_TESTER".to_string(),
            "ENGINEERING_DIRECTOR".to_string(),
        ]
    }
    fn authenticate(&self, creds: Credentials) -> Result<(), String> {
        if creds.api_key.is_empty() {
            return Err("claude provider requires an API key (ANTHROPIC_API_KEY)".to_string());
        }
        self.base.store(creds);
        Ok(())
    }
    fn get_credentials(&self) -> Credentials { self.base.load() }
    fn is_authenticated(&self) -> bool { !self.base.load().is_empty() }
    async fn run_in_isolation(&self, command: &str, worktree: &str, transport: Option<Arc<dyn Transport>>) -> Result<(), String> {
        execute_in_isolation(command, &self.provider_type().to_string(), worktree, transport).await
    }
}

pub struct GeminiProvider {
    base: BaseProvider,
}

impl GeminiProvider {
    pub fn new() -> Self {
        GeminiProvider { base: BaseProvider::new() }
    }
}

#[async_trait]
impl Provider for GeminiProvider {
    fn provider_type(&self) -> ProviderType { ProviderType::Gemini }
    fn description(&self) -> String { "Google Gemini CLI — multimodal assistant agent backed by Gemini Pro/Ultra".to_string() }
    fn supported_roles(&self) -> Vec<String> {
        vec![
            "PRODUCT_MANAGER".to_string(),
            "ANALYTICS_ENGINEER".to_string(),
            "MARKETING_MANAGER".to_string(),
            "CEO".to_string(),
        ]
    }
    fn authenticate(&self, creds: Credentials) -> Result<(), String> {
        if creds.api_key.is_empty() && creds.oauth_token.is_empty() {
            return Err("gemini provider requires an API key (GEMINI_API_KEY) or an OAuth token".to_string());
        }
        self.base.store(creds);
        Ok(())
    }
    fn get_credentials(&self) -> Credentials { self.base.load() }
    fn is_authenticated(&self) -> bool { !self.base.load().is_empty() }
    async fn run_in_isolation(&self, command: &str, worktree: &str, transport: Option<Arc<dyn Transport>>) -> Result<(), String> {
        execute_in_isolation(command, &self.provider_type().to_string(), worktree, transport).await
    }
}

pub struct OpenCodeProvider {
    base: BaseProvider,
}

impl OpenCodeProvider {
    pub fn new() -> Self {
        OpenCodeProvider { base: BaseProvider::new() }
    }
}

#[async_trait]
impl Provider for OpenCodeProvider {
    fn provider_type(&self) -> ProviderType { ProviderType::OpenCode }
    fn description(&self) -> String { "OpenCode — open-source software-engineering agent with full terminal and file-system access".to_string() }
    fn supported_roles(&self) -> Vec<String> {
        vec![
            "SOFTWARE_ENGINEER".to_string(),
            "ENGINEERING_DIRECTOR".to_string(),
            "QA_TESTER".to_string(),
        ]
    }
    fn authenticate(&self, creds: Credentials) -> Result<(), String> {
        if creds.api_key.is_empty() {
            return Err("opencode provider requires an API key".to_string());
        }
        self.base.store(creds);
        Ok(())
    }
    fn get_credentials(&self) -> Credentials { self.base.load() }
    fn is_authenticated(&self) -> bool { !self.base.load().is_empty() }
    async fn run_in_isolation(&self, command: &str, worktree: &str, transport: Option<Arc<dyn Transport>>) -> Result<(), String> {
        execute_in_isolation(command, &self.provider_type().to_string(), worktree, transport).await
    }
}

pub struct OpenClawProvider {
    base: BaseProvider,
}

impl OpenClawProvider {
    pub fn new() -> Self {
        OpenClawProvider { base: BaseProvider::new() }
    }
}

#[async_trait]
impl Provider for OpenClawProvider {
    fn provider_type(&self) -> ProviderType { ProviderType::OpenClaw }
    fn description(&self) -> String { "OpenClaw — general-purpose assistant agent optimised for content strategy and growth tasks".to_string() }
    fn supported_roles(&self) -> Vec<String> {
        vec![
            "GROWTH_AGENT".to_string(),
            "CONTENT_STRATEGIST".to_string(),
            "MARKETING_MANAGER".to_string(),
            "PRODUCT_MANAGER".to_string(),
        ]
    }
    fn authenticate(&self, creds: Credentials) -> Result<(), String> {
        if creds.api_key.is_empty() {
            return Err("openclaw provider requires an API key".to_string());
        }
        self.base.store(creds);
        Ok(())
    }
    fn get_credentials(&self) -> Credentials { self.base.load() }
    fn is_authenticated(&self) -> bool { !self.base.load().is_empty() }
    async fn run_in_isolation(&self, command: &str, worktree: &str, transport: Option<Arc<dyn Transport>>) -> Result<(), String> {
        execute_in_isolation(command, &self.provider_type().to_string(), worktree, transport).await
    }
}

pub struct IronClawProvider {
    base: BaseProvider,
}

impl IronClawProvider {
    pub fn new() -> Self {
        IronClawProvider { base: BaseProvider::new() }
    }
}

#[async_trait]
impl Provider for IronClawProvider {
    fn provider_type(&self) -> ProviderType { ProviderType::IronClaw }
    fn description(&self) -> String { "IronClaw — security and audit-focused agent with deep static-analysis capabilities".to_string() }
    fn supported_roles(&self) -> Vec<String> {
        vec![
            "SECURITY_ENGINEER".to_string(),
            "AUDIT_MANAGER".to_string(),
            "QA_TESTER".to_string(),
        ]
    }
    fn authenticate(&self, creds: Credentials) -> Result<(), String> {
        if creds.api_key.is_empty() {
            return Err("ironclaw provider requires an API key".to_string());
        }
        self.base.store(creds);
        Ok(())
    }
    fn get_credentials(&self) -> Credentials { self.base.load() }
    fn is_authenticated(&self) -> bool { !self.base.load().is_empty() }
    async fn run_in_isolation(&self, command: &str, worktree: &str, transport: Option<Arc<dyn Transport>>) -> Result<(), String> {
        execute_in_isolation(command, &self.provider_type().to_string(), worktree, transport).await
    }
}

pub struct MiniMaxiProvider {
    base: BaseProvider,
}

impl MiniMaxiProvider {
    pub fn new() -> Self {
        MiniMaxiProvider { base: BaseProvider::new() }
    }
}

#[async_trait]
impl Provider for MiniMaxiProvider {
    fn provider_type(&self) -> ProviderType { ProviderType::MiniMaxi }
    fn description(&self) -> String { "MiniMaxi — cloud AI API with Anthropic-compatible endpoint (api.minimaxi.chat/v1). Can be used for any role (SWE, legal, sales, etc.).".to_string() }
    fn supported_roles(&self) -> Vec<String> {
        vec![
            "CEO".to_string(), "PRODUCT_MANAGER".to_string(), "SOFTWARE_ENGINEER".to_string(), "ENGINEERING_DIRECTOR".to_string(),
            "QA_TESTER".to_string(), "SECURITY_ENGINEER".to_string(), "DESIGNER".to_string(), "MARKETING_MANAGER".to_string(),
            "GROWTH_AGENT".to_string(), "CONTENT_STRATEGIST".to_string(), "SEO_SPECIALIST".to_string(), "PAID_MEDIA_MANAGER".to_string(),
            "ANALYTICS_ENGINEER".to_string(), "CFO".to_string(), "BOOKKEEPER".to_string(), "TAX_SPECIALIST".to_string(),
            "AUDIT_MANAGER".to_string(), "PAYROLL_MANAGER".to_string(), "AI_NEWS_COLLECTOR".to_string(),
        ]
    }
    fn authenticate(&self, creds: Credentials) -> Result<(), String> {
        if creds.api_key.is_empty() {
            return Err("minimaxi provider requires an API key".to_string());
        }
        self.base.store(creds);
        Ok(())
    }
    fn get_credentials(&self) -> Credentials { self.base.load() }
    fn is_authenticated(&self) -> bool { !self.base.load().is_empty() }
    async fn run_in_isolation(&self, command: &str, worktree: &str, transport: Option<Arc<dyn Transport>>) -> Result<(), String> {
        execute_in_isolation(command, &self.provider_type().to_string(), worktree, transport).await
    }
}

pub struct BuiltinProvider {}

impl BuiltinProvider {
    pub fn new() -> Self {
        BuiltinProvider {}
    }
}

#[async_trait]
impl Provider for BuiltinProvider {
    fn provider_type(&self) -> ProviderType { ProviderType::Builtin }
    fn description(&self) -> String { "Built-in local agent — full agentic loop with tool execution; no external credentials required. Uses Anthropic/OpenAI/Ollama as configured.".to_string() }
    fn supported_roles(&self) -> Vec<String> {
        vec![
            "CEO".to_string(), "PRODUCT_MANAGER".to_string(), "SOFTWARE_ENGINEER".to_string(), "ENGINEERING_DIRECTOR".to_string(),
            "QA_TESTER".to_string(), "SECURITY_ENGINEER".to_string(), "DESIGNER".to_string(), "MARKETING_MANAGER".to_string(),
            "GROWTH_AGENT".to_string(), "CONTENT_STRATEGIST".to_string(), "SEO_SPECIALIST".to_string(), "PAID_MEDIA_MANAGER".to_string(),
            "ANALYTICS_ENGINEER".to_string(), "CFO".to_string(), "BOOKKEEPER".to_string(), "TAX_SPECIALIST".to_string(),
            "AUDIT_MANAGER".to_string(), "PAYROLL_MANAGER".to_string(), "AI_NEWS_COLLECTOR".to_string(),
        ]
    }
    fn authenticate(&self, _creds: Credentials) -> Result<(), String> { Ok(()) }
    fn get_credentials(&self) -> Credentials { Credentials::default() }
    fn is_authenticated(&self) -> bool { true }
    async fn run_in_isolation(&self, command: &str, worktree: &str, transport: Option<Arc<dyn Transport>>) -> Result<(), String> {
        // Advanced GRPC Dispatch Support
        let address = std::env::var("OHC_AGENT_ADDRESS").unwrap_or_default();
        if !address.is_empty() {
            tracing::debug!("Dispatching via gRPC to {}", address);
            // This is handled by orchestrator at runtime via OHC_AGENT_ADDRESS
            // It overrides local builtin tools loop with a remote node.
        }
        execute_in_isolation(command, &self.provider_type().to_string(), worktree, transport).await
    }
}

pub struct ScoutProvider {
    base: BaseProvider,
}

impl ScoutProvider {
    pub fn new() -> Self {
        ScoutProvider { base: BaseProvider::new() }
    }
}

#[async_trait]
impl Provider for ScoutProvider {
    fn provider_type(&self) -> ProviderType { ProviderType::Scout }
    fn description(&self) -> String { "Scout — agent dedicated to finding external resources and integrating them into OHC capabilities".to_string() }
    fn supported_roles(&self) -> Vec<String> {
        vec!["RESOURCE_SCOUT".to_string(), "TOOL_INTEGRATOR".to_string()]
    }
    fn authenticate(&self, creds: Credentials) -> Result<(), String> {
        self.base.store(creds);
        Ok(())
    }
    fn get_credentials(&self) -> Credentials { self.base.load() }
    fn is_authenticated(&self) -> bool { !self.base.load().is_empty() }
    async fn run_in_isolation(&self, command: &str, worktree: &str, transport: Option<Arc<dyn Transport>>) -> Result<(), String> {
        execute_in_isolation(command, &self.provider_type().to_string(), worktree, transport).await
    }
}
