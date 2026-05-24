use async_trait::async_trait;

#[derive(Debug, Clone, Default)]
pub struct SandboxConfig {
    pub read_only_paths: Vec<String>,
    pub blocked_domains: Vec<String>,
    pub disabled_commands: Vec<String>,
    pub deny_list_dirs: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct ViolationEvent {
    pub reason: String,
    pub command: String,
}

#[async_trait]
pub trait OHCSandboxManager: Send + Sync {
    async fn execute(&self, cmd: &str) -> Result<(bool, String, String), ViolationEvent>;
}
