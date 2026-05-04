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

pub struct DefaultLLMClient {
    client: crate::minimax::LocalLLMClient,
}

impl DefaultLLMClient {
    pub fn new() -> Self {
        Self { client: crate::minimax::LocalLLMClient::new() }
    }
}

#[async_trait]
impl LLMClient for DefaultLLMClient {
    async fn generate_embedding(&self, text: &str) -> Result<Vec<f32>, String> {
        self.client.generate_embedding(text).await
    }
}
