use async_trait::async_trait;

#[async_trait]
pub trait SandboxAskCallback: Send + Sync {
    async fn ask_for_permission(&self, command: &str, reason: &str) -> bool;
}
