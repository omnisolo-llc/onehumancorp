use std::sync::Arc;
use std::time::Duration;
use crate::minimax::MinimaxClient;
use ::server_pricing::prompt_caching::PromptCache;
use ::server_pricing::compression::{minify_json_prompt, truncate_by_word_count};

pub struct LocalLLMProvider {
    endpoint: String,
    embed_endpoint: String,
    model: String,
    cache: PromptCache,
    client: reqwest::Client,
}

impl LocalLLMProvider {
    pub fn new(endpoint: String, embed_endpoint: String, model: String) -> Self {
        LocalLLMProvider {
            endpoint,
            embed_endpoint,
            model,
            cache: PromptCache::new(Duration::from_secs(600)), // 10 minute TTL
            client: reqwest::Client::new(),
        }
    }

    pub fn from_env() -> Self {
        let endpoint = std::env::var("OHC_LOCAL_LLM_ENDPOINT")
            .unwrap_or_else(|_| "http://127.0.0.1:11434/api/generate".to_string());
        let embed_endpoint = std::env::var("OHC_LOCAL_LLM_EMBED_ENDPOINT")
            .unwrap_or_else(|_| "http://127.0.0.1:11434/api/embeddings".to_string());
        let model = std::env::var("OHC_LOCAL_MODEL_NAME")
            .unwrap_or_else(|_| "llama3".to_string());
        Self::new(endpoint, embed_endpoint, model)
    }

    pub async fn reason(&self, prompt: &str) -> Result<String, String> {
        // 1. Check Cache
        if let Some(cached) = self.cache.get(prompt) {
            tracing::info!(model = %self.model, "Local prompt cache hit (saved ~{} tokens)", cached.token_count);
            return Ok(cached.text);
        }

        // 2. Optimize Prompt
        let optimized_prompt = if prompt.starts_with('{') {
            minify_json_prompt(prompt)
        } else {
            truncate_by_word_count(prompt, 1500) // Slightly more conservative for local models
        };

        let start = std::time::Instant::now();
        
        let request_body = serde_json::json!({
            "model":  self.model,
            "prompt": optimized_prompt,
            "stream": false,
        });

        let resp = self.client.post(&self.endpoint)
            .json(&request_body)
            .send()
            .await
            .map_err(|e| e.to_string())?;

        let duration = start.elapsed().as_secs_f64();
        tracing::info!(model = %self.model, latency = duration, "Recorded LLM Network Latency");

        if !resp.status().is_success() {
            let status = resp.status();
            let err_body = resp.text().await.unwrap_or_default();
            return Err(format!("local LLM error (status {}): {}", status, err_body));
        }

        let result: serde_json::Value = resp.json().await.map_err(|e| e.to_string())?;
        let response = result["response"].as_str().ok_or_else(|| "missing response field".to_string())?;
        
        // 3. Update Cache
        self.cache.set(prompt, response, prompt.len() / 4);

        Ok(response.to_string())
    }

    pub async fn generate_embedding(&self, text: &str) -> Result<Vec<f32>, String> {
        let start = std::time::Instant::now();
        
        let request_body = serde_json::json!({
            "model":  self.model,
            "prompt": text,
        });

        let resp = self.client.post(&self.embed_endpoint)
            .json(&request_body)
            .send()
            .await
            .map_err(|e| e.to_string())?;

        let duration = start.elapsed().as_secs_f64();
        tracing::info!(model = %self.model, latency = duration, "Recorded LLM Network Latency");

        if !resp.status().is_success() {
            let status = resp.status();
            let err_body = resp.text().await.unwrap_or_default();
            return Err(format!("local LLM embedding error (status {}): {}", status, err_body));
        }

        let result: serde_json::Value = resp.json().await.map_err(|e| e.to_string())?;
        let embedding_val = result["embedding"].as_array().ok_or_else(|| "missing embedding field".to_string())?;
        
        let mut embedding = Vec::new();
        for v in embedding_val {
            if let Some(f) = v.as_f64() {
                embedding.push(f as f32);
            }
        }
        
        Ok(embedding)
    }
}

pub struct ResilientProvider {
    primary: Arc<MinimaxClient>,
    fallback: Arc<LocalLLMProvider>,
}

impl ResilientProvider {
    pub fn new(primary: Arc<MinimaxClient>, fallback: Option<Arc<LocalLLMProvider>>) -> Self {
        let fallback = fallback.unwrap_or_else(|| Arc::new(LocalLLMProvider::from_env()));
        ResilientProvider {
            primary,
            fallback,
        }
    }

    pub async fn reason(&self, prompt: &str) -> Result<String, String> {
        match self.primary.reason(prompt).await {
            Ok(resp) => Ok(resp),
            Err(e) => {
                if is_network_error(&e) {
                    tracing::warn!("Primary LLM failed with network error, falling back to local: {}", e);
                    self.fallback.reason(prompt).await
                } else {
                    Err(e)
                }
            }
        }
    }

    pub async fn generate_embedding(&self, text: &str) -> Result<Vec<f32>, String> {
        match self.primary.generate_embedding(text).await {
            Ok(emb) => Ok(emb),
            Err(e) => {
                if is_network_error(&e) {
                    tracing::warn!("Primary LLM failed with network error, falling back to local: {}", e);
                    self.fallback.generate_embedding(text).await
                } else {
                    Err(e)
                }
            }
        }
    }
}

fn is_network_error(err: &str) -> bool {
    // Simplified check based on string matching, as we don't have typed errors from gRPC or HTTP client here yet in this simplified version.
    err.contains("timeout") || err.contains("connection refused") || err.contains("closed") || err.contains("503")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_local_provider_cache_init() {
        let provider = LocalLLMProvider::from_env();
        // Just verify it doesn't panic and cache is accessible
        assert!(provider.cache.get("test").is_none());
    }
}
