use async_trait::async_trait;

#[async_trait]
pub trait LLMClient: Send + Sync {
    async fn generate_embedding(&self, text: &str) -> Result<Vec<f32>, String>;
}

pub struct MockLLMClient {
    pub embedding: Vec<f32>,
}

#[async_trait]
impl LLMClient for MockLLMClient {
    async fn generate_embedding(&self, _text: &str) -> Result<Vec<f32>, String> {
        Ok(self.embedding.clone())
    }
}

pub struct RealLLMClient {
    inner: crate::minimax::LocalLLMClient,
}

impl RealLLMClient {
    pub fn new() -> Self {
        RealLLMClient {
            inner: crate::minimax::LocalLLMClient::new(),
        }
    }
}

#[async_trait]
impl LLMClient for RealLLMClient {
    async fn generate_embedding(&self, text: &str) -> Result<Vec<f32>, String> {
        self.inner.generate_embedding(text).await
    }
}
