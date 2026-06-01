use async_trait::async_trait;

#[async_trait]
pub trait AgentHarness: Send + Sync {
    async fn execute(&self, cmd: &str) -> Result<String, String>;
}
