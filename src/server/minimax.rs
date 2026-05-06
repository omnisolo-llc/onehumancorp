use serde::{Deserialize, Serialize};
use std::sync::Mutex;
use std::sync::OnceLock;
use std::time::{Duration, Instant};

struct CircuitBreaker {
    failures: Mutex<usize>,
    last_failure: Mutex<Option<Instant>>,
    max_failures: usize,
    reset_timeout: Duration,
}

impl CircuitBreaker {
    fn new(max_failures: usize, reset_timeout: Duration) -> Self {
        CircuitBreaker {
            failures: Mutex::new(0),
            last_failure: Mutex::new(None),
            max_failures,
            reset_timeout,
        }
    }

    fn allow(&self) -> bool {
        let failures = self.failures.lock().unwrap();
        if *failures >= self.max_failures {
            let last_failure = self.last_failure.lock().unwrap();
            if let Some(last) = *last_failure {
                if last.elapsed() > self.reset_timeout {
                    return true;
                }
            }
            return false;
        }
        true
    }

    fn record_success(&self) {
        let mut failures = self.failures.lock().unwrap();
        *failures = 0;
    }

    fn record_failure(&self) {
        let mut failures = self.failures.lock().unwrap();
        *failures += 1;
        let mut last_failure = self.last_failure.lock().unwrap();
        *last_failure = Some(Instant::now());
    }
}

static GLOBAL_CIRCUIT_BREAKER: OnceLock<CircuitBreaker> = OnceLock::new();
static PROMPT_CACHE: OnceLock<crate::pricing::cache::LocalEmbeddingCache> = OnceLock::new();

fn get_circuit_breaker() -> &'static CircuitBreaker {
    GLOBAL_CIRCUIT_BREAKER.get_or_init(|| CircuitBreaker::new(3, Duration::from_secs(120)))
}

fn get_prompt_cache() -> &'static crate::pricing::cache::LocalEmbeddingCache {
    PROMPT_CACHE.get_or_init(|| crate::pricing::cache::LocalEmbeddingCache::new(Duration::from_secs(3600 * 24))) // 24 hours
}

pub struct MinimaxClient {
    api_key: String,
    url: String,
}

#[derive(Debug, Serialize)]
struct MinimaxRequest {
    model: String,
    messages: Vec<MinimaxMessage>,
}

#[derive(Debug, Serialize)]
struct MinimaxMessage {
    role: String,
    content: String,
}

#[derive(Debug, Deserialize)]
struct MinimaxResponse {
    choices: Vec<Choice>,
}

#[derive(Debug, Deserialize)]
struct Choice {
    message: MessageContent,
}

#[derive(Debug, Deserialize)]
struct MessageContent {
    content: String,
}

impl MinimaxClient {
    pub fn new(api_key: String) -> Self {
        MinimaxClient {
            api_key,
            url: "https://api.minimax.chat/v1/chat/completions".to_string(),
        }
    }

    pub async fn reason(&self, prompt: &str) -> Result<String, String> {
        if let Some(cached) = get_prompt_cache().get(prompt) {
            return Ok(cached);
        }

        let cb = get_circuit_breaker();
        if !cb.allow() {
            return Err("circuit breaker open".to_string());
        }

        let client = reqwest::Client::new();

        let request_body = MinimaxRequest {
            model: "MiniMax-M2.7".to_string(),
            messages: vec![MinimaxMessage {
                role: "user".to_string(),
                content: prompt.to_string(),
            }],
        };

        let mut last_err = String::new();
        for _ in 0..5 {
            let response = client
                .post(&self.url)
                .header("Content-Type", "application/json")
                .header("Authorization", format!("Bearer {}", self.api_key))
                .json(&request_body)
                .send()
                .await;

            match response {
                Ok(resp) => {
                    if resp.status().is_success() {
                        let result: MinimaxResponse = resp.json().await.map_err(|e| e.to_string())?;
                        cb.record_success();
                        if let Some(choice) = result.choices.first() {
                            let content = choice.message.content.clone();
                            get_prompt_cache().set(prompt, &content);
                            return Ok(content);
                        } else {
                            last_err = "empty response from minimax".to_string();
                            cb.record_failure();
                            tokio::time::sleep(Duration::from_secs(1)).await;
                            continue;
                        }
                    } else {
                        if resp.status().as_u16() >= 500 {
                            last_err = format!("API overloaded (status {})", resp.status());
                            tokio::time::sleep(Duration::from_secs(2)).await;
                            continue;
                        }
                        cb.record_failure();
                        let status = resp.status();
                        let text = resp.text().await.unwrap_or_default();
                        last_err = format!("API error (status {}): {}", status, text);
                        tokio::time::sleep(Duration::from_secs(1)).await;
                        continue;
                    }
                }
                Err(e) => {
                    cb.record_failure();
                    last_err = e.to_string();
                    tokio::time::sleep(Duration::from_secs(1)).await;
                    continue;
                }
            }
        }

        Err(format!("failed after 5 retries: {}", last_err))
    }

    pub async fn generate_embedding(&self, text: &str) -> Result<Vec<f32>, String> {
        let cb = get_circuit_breaker();
        if !cb.allow() {
            return Err("circuit breaker open".to_string());
        }

        let client = reqwest::Client::new();

        let request_body = serde_json::json!({
            "model": "embo-01",
            "type": "db",
            "texts": [text]
        });

        let mut last_err = String::new();
        for _ in 0..5 {
            let response = client
                .post("https://api.minimax.chat/v1/embeddings")
                .header("Content-Type", "application/json")
                .header("Authorization", format!("Bearer {}", self.api_key))
                .json(&request_body)
                .send()
                .await;

            match response {
                Ok(resp) => {
                    if resp.status().is_success() {
                        let result: serde_json::Value = resp.json().await.map_err(|e| e.to_string())?;
                        cb.record_success();
                        if let Some(vectors) = result["vectors"].as_array() {
                            if let Some(vector) = vectors.first() {
                                if let Some(array) = vector.as_array() {
                                    let f32_vec: Vec<f32> = array.iter().map(|v| v.as_f64().unwrap() as f32).collect();
                                    return Ok(f32_vec);
                                }
                            }
                        }
                        return Err("invalid response format".to_string());
                    } else {
                        if resp.status().as_u16() >= 500 {
                            last_err = format!("API overloaded (status {})", resp.status());
                            tokio::time::sleep(Duration::from_secs(2)).await;
                            continue;
                        }
                        cb.record_failure();
                        last_err = format!("API error (status {})", resp.status());
                        tokio::time::sleep(Duration::from_secs(1)).await;
                        continue;
                    }
                }
                Err(e) => {
                    cb.record_failure();
                    last_err = e.to_string();
                    tokio::time::sleep(Duration::from_secs(1)).await;
                    continue;
                }
            }
        }

        Err(format!("failed after 5 retries: {}", last_err))
    }
}

#[allow(dead_code)]
pub struct LocalLLMClient {
    endpoint: String,
    embed_endpoint: String,
    model: String,
}

#[allow(dead_code)]
impl LocalLLMClient {
    pub fn new() -> Self {
        let endpoint = std::env::var("OHC_LOCAL_LLM_ENDPOINT")
            .unwrap_or_else(|_| "http://127.0.0.1:11434/api/generate".to_string());
        let embed_endpoint = std::env::var("OHC_LOCAL_LLM_EMBED_ENDPOINT")
            .unwrap_or_else(|_| "http://127.0.0.1:11434/api/embeddings".to_string());
        let model = std::env::var("OHC_LOCAL_MODEL_NAME")
            .unwrap_or_else(|_| "llama3".to_string());
            
        LocalLLMClient { endpoint, embed_endpoint, model }
    }

    pub async fn reason(&self, prompt: &str) -> Result<String, String> {
        let client = reqwest::Client::new();
        let req_body = serde_json::json!({
            "model": self.model,
            "prompt": prompt,
            "stream": false,
        });

        let resp = client.post(&self.endpoint)
            .json(&req_body)
            .send()
            .await
            .map_err(|e| e.to_string())?;

        if !resp.status().is_success() {
            return Err(format!("local LLM error (status {})", resp.status()));
        }

        let result: serde_json::Value = resp.json().await.map_err(|e| e.to_string())?;
        let response = result["response"].as_str().ok_or("missing response field")?;
        Ok(response.to_string())
    }

    pub async fn generate_embedding(&self, text: &str) -> Result<Vec<f32>, String> {
        let client = reqwest::Client::new();
        let req_body = serde_json::json!({
            "model": self.model,
            "prompt": text,
        });

        let resp = client.post(&self.embed_endpoint)
            .json(&req_body)
            .send()
            .await
            .map_err(|e| e.to_string())?;

        if !resp.status().is_success() {
            return Err(format!("local LLM embedding error (status {})", resp.status()));
        }

        let result: serde_json::Value = resp.json().await.map_err(|e| e.to_string())?;
        let embedding = result["embedding"].as_array().ok_or("missing embedding field")?;
        let f32_vec: Vec<f32> = embedding.iter().map(|v| v.as_f64().unwrap() as f32).collect();
        Ok(f32_vec)
    }
}

#[allow(dead_code)]
pub struct ResilientClient {
    primary: MinimaxClient,
    fallback: LocalLLMClient,
}

#[allow(dead_code)]
impl ResilientClient {
    pub fn new(primary: MinimaxClient) -> Self {
        ResilientClient {
            primary,
            fallback: LocalLLMClient::new(),
        }
    }

    pub async fn reason(&self, prompt: &str) -> Result<String, String> {
        match self.primary.reason(prompt).await {
            Ok(res) => Ok(res),
            Err(e) => {
                tracing::warn!("Primary LLM failed: {}. Falling back to local.", e);
                self.fallback.reason(prompt).await
            }
        }
    }

    pub async fn generate_embedding(&self, text: &str) -> Result<Vec<f32>, String> {
        match self.primary.generate_embedding(text).await {
            Ok(res) => Ok(res),
            Err(e) => {
                tracing::warn!("Primary LLM failed: {}. Falling back to local.", e);
                self.fallback.generate_embedding(text).await
            }
        }
    }
}

