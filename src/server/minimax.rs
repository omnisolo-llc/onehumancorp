use serde::{Deserialize, Serialize};
use std::sync::Mutex;
use std::sync::OnceLock;
use std::time::{Duration, Instant};
use ::server_pricing::prompt_caching::PromptCache;
use ::server_pricing::deduplication::{RequestDeduplicator, DeduplicationResult};
use ::server_pricing::compression::{minify_json_prompt};
use tokio_stream::Stream;
use std::pin::Pin;

pub struct CircuitBreaker {
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
        let mut failures = self.failures.lock().unwrap();
        if *failures >= self.max_failures {
            let last_failure = self.last_failure.lock().unwrap();
            if let Some(last) = *last_failure {
                if last.elapsed() > self.reset_timeout {
                    *failures = 0; // Reset failures so we can retry properly
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

    #[cfg(test)]
    pub fn reset_for_tests(&self) {
        let mut failures = self.failures.lock().unwrap();
        *failures = 0;
        let mut last_failure = self.last_failure.lock().unwrap();
        *last_failure = None;
    }
}

static GLOBAL_CIRCUIT_BREAKER: OnceLock<CircuitBreaker> = OnceLock::new();

pub fn get_circuit_breaker() -> &'static CircuitBreaker {
    GLOBAL_CIRCUIT_BREAKER.get_or_init(|| CircuitBreaker::new(3, Duration::from_secs(120)))
}

pub struct MinimaxClient {
    api_key: String,
    url: String,
    cache: PromptCache,
    deduplicator: std::sync::Arc<RequestDeduplicator>,
}

#[derive(Debug, Serialize)]
struct MinimaxRequest {
    model: String,
    messages: Vec<MinimaxMessage>,
    #[serde(skip_serializing_if = "Option::is_none")]
    stream: Option<bool>,
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
            cache: PromptCache::new(Duration::from_secs(300)),
            deduplicator: std::sync::Arc::new(RequestDeduplicator::new(Duration::from_secs(5))), // 5 minute TTL
        }
    }

    pub async fn reason(&self, prompt: &str) -> Result<String, String> {
        let prompt_clone = prompt.to_string();
        let deduplicator = self.deduplicator.clone();

        let result = deduplicator.deduplicate(&prompt_clone, || async {
            self.internal_reason(&prompt_clone).await.map(|resp| DeduplicationResult { response: resp })
        }).await?;

        Ok(result.response)
    }

    async fn internal_reason(&self, prompt: &str) -> Result<String, String> {
        let optimized_prompt = if prompt.starts_with('{') {
            minify_json_prompt(prompt)
        } else {
            let reduced = ::server_pricing::compression::reduce_tokens(prompt);
            PromptCache::truncate_context(&reduced, 2000)
        };

        // 1. Check Cache
        if let (Some(cached), _cost_cents) = self.cache.get_with_cost_cents(&optimized_prompt, "minimax-text-01") {
            tracing::info!("Prompt cache hit (saved ~{} tokens)", cached.token_count); // pii-safe
            return Ok(cached.text);
        }

        if self.api_key == "fake-key" {
            let lower_prompt = optimized_prompt.to_lowercase();
            if lower_prompt.contains("maya") {
                return Ok(r#"{
                    "business_name": "Maya's Cakes",
                    "business_type": "Bakery",
                    "categories": ["food", "physical"],
                    "initial_products": [{"name": "Custom Vegan Cake", "price": "45.00", "variants": [{"name": "6-inch", "price_modifier": "0.00"}, {"name": "8-inch", "price_modifier": "15.00"}]}],
                    "suggested_features": ["menu", "booking", "online_store"]
                }"#.to_string());
            } else if lower_prompt.contains("alex") || lower_prompt.contains("art shop") {
                return Ok(r#"{
                    "business_name": "Alex Art",
                    "business_type": "Retail",
                    "categories": ["art"],
                    "initial_products": [{"name": "Painting", "price": "100.00"}],
                    "suggested_features": ["online_store"]
                }"#.to_string());
            } else if lower_prompt.contains("carlos") {
                return Ok(r#"{
                    "business_name": "Carlos Plumbing",
                    "business_type": "Service",
                    "categories": ["service"],
                    "initial_products": [{"name": "Pipe Fix", "price": "80.00"}],
                    "suggested_features": ["booking"]
                }"#.to_string());

            } else if lower_prompt.contains("e2e_mock_trigger_expert_team_analysis") {
                if lower_prompt.contains("you are an expert in") {
                    let rand_num = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos();
                    let role = if lower_prompt.contains("researcher") { "Chapter 1 Chapter 2 unique words " }
                    else if lower_prompt.contains("financial") { "Chapter 3 Chapter 4 unique terms " }
                    else if lower_prompt.contains("strategic") { "Chapter 5 Chapter 6 unique ideas " }
                    else if lower_prompt.contains("process") { "Chapter 7 unique process " }
                    else { "Chapter 8 unique quality " };
                    return Ok(format!("{}{}", role, rand_num));
                } else if lower_prompt.contains("synthesize") {
                    return Ok("Combined Executive Summary:\nIndustry Researcher: Done.\nFinancial Analyst: Done.\nStrategic Analyst: Done.\nProcess Supervisor: Done.\nQuality Auditor: Done.\n\nOverall Strategy:\nProceed based on above.\nChart: Included.\nAnalysis: Completed.\n\n".to_string() + &" word".repeat(20000));
                }
                return Ok("Combined Executive Summary:\nIndustry Researcher: Done.\nFinancial Analyst: Done.\nStrategic Analyst: Done.\nProcess Supervisor: Done.\nQuality Auditor: Done.\n\nOverall Strategy:\nProceed based on above.\nChart: Included.\nAnalysis: Completed.\n\n".to_string() + &" word".repeat(20000));
            } else if lower_prompt.contains("e2e_mock_trigger_expert_team_failure") {
                return Ok("Short output".to_string());
            } else if lower_prompt.contains("marketing_strategist") || lower_prompt.contains("marketing strategist") {
                 return Ok(r#"{
                    "agent_id": "marketing_strategist",
                    "role": "Marketing Strategist",
                    "contribution": "Plan accepted after repair with launch workstreams defined. The operations looks solid.",
                    "handoff_to": ["sales_engineer"],
                    "confidence": 0.95
                }"#.to_string());
            } else if lower_prompt.contains("sales_engineer") || lower_prompt.contains("sales engineer") {
                 return Ok(r#"{
                    "agent_id": "sales_engineer",
                    "role": "Sales Engineer",
                    "contribution": "Plan accepted after repair with launch workstreams defined. The operations looks solid.",
                    "handoff_to": ["operations_planner"],
                    "confidence": 0.95
                }"#.to_string());
            } else if lower_prompt.contains("operations_planner") || lower_prompt.contains("operations planner") {
                 return Ok(r#"{
                    "agent_id": "operations_planner",
                    "role": "Operations Planner",
                    "contribution": "Plan accepted after repair with launch workstreams defined. The operations looks solid.",
                    "handoff_to": ["quality_reviewer"],
                    "confidence": 0.95
                }"#.to_string());
            } else if lower_prompt.contains("quality_reviewer") || lower_prompt.contains("quality reviewer") {
                return Ok(r#"{
                    "agent_id": "quality_reviewer",
                    "role": "Quality Reviewer",
                    "contribution": "Final review resolves the prior agent contributions into launch steps.",
                    "handoff_to": [],
                    "confidence": 0.95
                }"#.to_string());
            } else if lower_prompt.contains("you are an ai order and task triage assistant") || lower_prompt.contains("you are an omni-context work triage agent") {
                if lower_prompt.contains("vegan options") {
                    if lower_prompt.contains("omni-context") {
                        return Ok(r#"{
                          "operations_context": null,
                          "sales_context": null,
                          "customer_context": "Drafted reply to e2e from instagram.",
                          "final_draft": "Hi there! Yes, we do offer vegan options. I see you've previously ordered with us. Would you like to see our menu?"
                        }"#.to_string());
                    } else {
                        return Ok(r#"{
                            "priority": "Medium",
                            "feature_type": "instagram_dm",
                            "context_summary": "Customer asking about vegan options",
                            "action_type": "Draft Reply",
                            "action_payload": "Hi there! Yes, we do offer vegan options. I see you've previously ordered with us. Would you like to see our menu?"
                        }"#.to_string());
                    }
                } else if lower_prompt.contains("schedule") || lower_prompt.contains("calendar") {
                    if lower_prompt.contains("omni-context") {
                        return Ok(r#"{
                          "operations_context": "Checked schedule: Available next Tuesday.",
                          "sales_context": null,
                          "customer_context": "Drafted reply.",
                          "final_draft": "Hello! Checked schedule: Available next Tuesday."
                        }"#.to_string());
                    } else {
                        return Ok(r#"{
                            "priority": "Medium",
                            "feature_type": "instagram_dm",
                            "context_summary": "Customer needs to schedule",
                            "action_type": "Draft Booking",
                            "action_payload": "{\"service_id\":\"custom_cake\",\"start_time\":\"2024-08-01T14:00:00Z\",\"end_time\":\"2024-08-01T15:00:00Z\"}"
                        }"#.to_string());
                    }
                } else if lower_prompt.contains("quote") || lower_prompt.contains("price") {
                    if lower_prompt.contains("omni-context") {
                        return Ok(r#"{
                          "operations_context": null,
                          "sales_context": "Generated quote: $150.",
                          "customer_context": "Drafted reply.",
                          "final_draft": "Hello! Generated quote: $150."
                        }"#.to_string());
                    } else {
                        return Ok(r#"{
                            "priority": "Medium",
                            "feature_type": "instagram_dm",
                            "context_summary": "Customer needs quote",
                            "action_type": "Draft Quote",
                            "action_payload": "{\"total_amount_cents\":15000,\"required_deposit_cents\":5000,\"line_items\":[{\"description\":\"Custom Cake\",\"unit_price_cents\":15000,\"quantity\":1,\"is_optional\":false}]}"
                        }"#.to_string());
                    }
                } else {
                    if lower_prompt.contains("omni-context") {
                        return Ok(r#"{
                          "operations_context": null,
                          "sales_context": null,
                          "customer_context": "Drafted reply.",
                          "final_draft": "Thanks for reaching out! We will review this and get back to you soon."
                        }"#.to_string());
                    } else {
                        return Ok(r#"{
                            "priority": "Medium",
                            "feature_type": "general",
                            "context_summary": "Customer inquiry",
                            "action_type": "Draft Reply",
                            "action_payload": "Thanks for reaching out! We will review this and get back to you soon."
                        }"#.to_string());
                    }
                }

            } else {
                return Ok(r#"{
                    "business_name": "Generic Business",
"priority": "urgent",
"context_summary": "Customer needs sink fixed tomorrow",
"action_type": "Draft Booking",
"action_payload": "I can fix it tomorrow at 2 PM.",
"feature_type": "instagram_dm",
                    "business_type": "Retail",
                    "categories": ["physical"],
                    "initial_products": [{"name": "Item 1", "price": "10.00"}],
                    "suggested_features": ["online_store"]
                }"#.to_string());
            }
        }

        let cb = get_circuit_breaker();
        if !cb.allow() {
            return Err("circuit breaker open".to_string());
        }

        let client = reqwest::Client::new();

        let request_body = MinimaxRequest {
            model: std::env::var("MINIMAX_MODEL").unwrap_or_else(|_| "MiniMax-M3".to_string()),
            messages: vec![MinimaxMessage {
                role: "user".to_string(),
                content: optimized_prompt.clone(),
            }],
            stream: Some(false),
        };

        let mut last_err = String::new();
        for _ in 0..3 {
            let response_future = client.post(&self.url).header("Content-Type", "application/json").header("Authorization", format!("Bearer {}", self.api_key)).json(&request_body).send();
            let response = tokio::time::timeout(Duration::from_secs(60), response_future).await.map_err(|e| e.to_string()).and_then(|r| r.map_err(|e| e.to_string()));

            match response {
                Ok(resp) => {
                    if resp.status().is_success() {
                        let result: MinimaxResponse = resp.json().await.map_err(|e| e.to_string())?;
                        cb.record_success();
                        if let Some(choice) = result.choices.first() {
                            let content = choice.message.content.clone();
                            // 3. Update Cache
                            self.cache.set(&optimized_prompt, &content, optimized_prompt.len() / 4); // rough token estimate
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

        Err(format!("failed after 3 retries: {}", last_err))
    }

    pub async fn reason_stream(&self, prompt: &str) -> Pin<Box<dyn Stream<Item = Result<String, String>> + Send>> {
        let api_key = self.api_key.clone();
        let url = self.url.clone();
        let optimized_prompt = if prompt.starts_with('{') {
            minify_json_prompt(prompt)
        } else {
            PromptCache::truncate_context(prompt, 2000)
        };

        let (tx, rx) = tokio::sync::mpsc::channel(100);

        // 1. Check Cache
        if let (Some(cached), _cost_cents) = self.cache.get_with_cost_cents(&optimized_prompt, "minimax-text-01") {
            tracing::info!("Prompt cache hit in stream (saved ~{} tokens)", cached.token_count); // pii-safe
            let cached_text = cached.text.clone();
            tokio::spawn(async move {
                let _ = tx.send(Ok(cached_text)).await;
            });
            return Box::pin(tokio_stream::wrappers::ReceiverStream::new(rx));
        }

        if self.api_key == "fake-key" {
            let (tx, rx) = tokio::sync::mpsc::channel(1);
            tokio::spawn(async move {
                let mock_json = r#"{"choices": [{"delta": {"content": "{\"business_name\": \"Generic Business\"}"}}]}"#;
                let mock_response = format!("data: {}\n\ndata: [DONE]\n\n", mock_json);
                for line in mock_response.lines() {
                    if line.starts_with("data: ") {
                        let json_str = &line[6..];
                        let _ = tx.send(Ok(json_str.to_string())).await;
                    }
                }
            });
            return Box::pin(tokio_stream::wrappers::ReceiverStream::new(rx));
        }

        tokio::spawn(async move {
            let client = reqwest::Client::new();
            let request_body = MinimaxRequest {
                model: "MiniMax-M2.7".to_string(),
                messages: vec![MinimaxMessage {
                    role: "user".to_string(),
                    content: optimized_prompt,
                }],
                stream: Some(true),
            };

            let response = client
                .post(&url)
                .header("Content-Type", "application/json")
                .header("Authorization", format!("Bearer {}", api_key))
                .json(&request_body)
                .send()
                .await;

            match response {
                Ok(resp) => {
                    if resp.status().is_success() {
                        let mut stream = resp.bytes_stream();
                        use tokio_stream::StreamExt;
                        while let Some(chunk_res) = stream.next().await {
                            match chunk_res {
                                Ok(chunk) => {
                                    let text = String::from_utf8_lossy(&chunk).to_string();
                                    // Parse SSE data: data: {"choices": [{"delta": {"content": "..."}}]}
                                    // Note: lossy conversion might corrupt characters split across chunks.
                                    // Ideally use a stateful UTF-8 decoder.
                                    for line in text.lines() {
                                        if line.starts_with("data: ") {
                                            let json_str = &line[6..];
                                            if json_str == "[DONE]" { break; }
                                            if let Ok(val) = serde_json::from_str::<serde_json::Value>(json_str) {
                                                if let Some(content) = val["choices"][0]["delta"]["content"].as_str() {
                                                    let _ = tx.send(Ok(content.to_string())).await;
                                                }
                                            }
                                        }
                                    }
                                }
                                Err(e) => { let _ = tx.send(Err(e.to_string())).await; }
                            }
                        }
                    } else {
                        let _ = tx.send(Err(format!("Stream error: {}", resp.status()))).await;
                    }
                }
                Err(e) => { let _ = tx.send(Err(e.to_string())).await; }
            }
        });

        Box::pin(tokio_stream::wrappers::ReceiverStream::new(rx))
    }

    pub async fn generate_embedding(&self, text: &str) -> Result<Vec<f32>, String> {
        let cb = get_circuit_breaker();
        if !cb.allow() {
            return Err("circuit breaker open".to_string());
        }

        if self.api_key == "fake-key" {
            return Ok(vec![0.1; 1536]);
        }

        let client = reqwest::Client::new();

        let request_body = serde_json::json!({
            "model": "embo-01",
            "type": "db",
            "texts": [text]
        });

        let mut last_err = String::new();
        for _ in 0..3 {
            let response_future = client.post("https://api.minimax.chat/v1/embeddings").header("Content-Type", "application/json").header("Authorization", format!("Bearer {}", self.api_key)).json(&request_body).send();
            let response = tokio::time::timeout(Duration::from_secs(60), response_future).await.map_err(|e| e.to_string()).and_then(|r| r.map_err(|e| e.to_string()));

            match response {
                Ok(resp) => {
                    if resp.status().is_success() {
                        let result: serde_json::Value = resp.json().await.map_err(|e| e.to_string())?;

                        // Handle Minimax base_resp envelope
                        if let Some(base_resp) = result.get("base_resp") {
                            let code = base_resp.get("status_code").and_then(|c| c.as_i64()).unwrap_or(0);
                            if code != 0 && code != 1000 {
                                cb.record_failure();
                                let msg = base_resp.get("status_msg").and_then(|m| m.as_str()).unwrap_or("unknown error");
                                last_err = format!("API error (status {}): {}", code, msg);
                                tokio::time::sleep(Duration::from_secs(1)).await;
                                continue;
                            }
                        }

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

        Err(format!("failed after 3 retries: {}", last_err))
    }

}

pub struct LocalLLMClient {
    endpoint: String,
    embed_endpoint: String,
    model: String,
    cache: PromptCache,
    deduplicator: std::sync::Arc<RequestDeduplicator>,
}

impl LocalLLMClient {
    pub fn new() -> Self {
        let endpoint = std::env::var("OHC_LOCAL_LLM_ENDPOINT")
            .unwrap_or_else(|_| "http://127.0.0.1:11434/api/generate".to_string());
        let embed_endpoint = std::env::var("OHC_LOCAL_LLM_EMBED_ENDPOINT")
            .unwrap_or_else(|_| "http://127.0.0.1:11434/api/embeddings".to_string());
        let model = std::env::var("OHC_LOCAL_MODEL_NAME")
            .unwrap_or_else(|_| "llama3".to_string());
            
        LocalLLMClient { endpoint, embed_endpoint, model, cache: PromptCache::new(Duration::from_secs(300)), deduplicator: std::sync::Arc::new(RequestDeduplicator::new(Duration::from_secs(5))) }
    }

    pub async fn reason(&self, prompt: &str) -> Result<String, String> {
        let prompt_clone = prompt.to_string();
        let deduplicator = self.deduplicator.clone();

        let result = deduplicator.deduplicate(&prompt_clone, || async {
            self.internal_reason(&prompt_clone).await.map(|resp| DeduplicationResult { response: resp })
        }).await?;

        Ok(result.response)
    }

    async fn internal_reason(&self, prompt: &str) -> Result<String, String> {
        let cb = get_circuit_breaker();
        if !cb.allow() {
            return Err("circuit breaker open".to_string());
        }

        let optimized_prompt = if prompt.starts_with('{') {
            minify_json_prompt(prompt)
        } else {
            let reduced = ::server_pricing::compression::reduce_tokens(prompt);
            PromptCache::truncate_context(&reduced, 2000)
        };

        if let (Some(cached), _cost_cents) = self.cache.get_with_cost_cents(&optimized_prompt, &self.model) {
            tracing::info!("Prompt cache hit (saved ~{} tokens)", cached.token_count); // pii-safe
            return Ok(cached.text);
        }

        let client = reqwest::Client::new();
        let req_body = serde_json::json!({
            "model": self.model,
            "prompt": optimized_prompt,
            "stream": false,
        });

        let mut last_err = String::new();
        for _ in 0..3 {
            let response_future = client.post(&self.endpoint).json(&req_body).send();
            let response = tokio::time::timeout(Duration::from_secs(60), response_future).await.map_err(|e| e.to_string()).and_then(|r| r.map_err(|e| e.to_string()));

            match response {
                Ok(resp) => {
                    if resp.status().is_success() {
                        let result_res: Result<serde_json::Value, _> = resp.json().await.map_err(|e| e.to_string());
                        if let Ok(result) = result_res {
                            if let Some(response) = result["response"].as_str() {
                                cb.record_success();
                                self.cache.set(&optimized_prompt, response, optimized_prompt.len() / 4);
                                return Ok(response.to_string());
                            } else {
                                last_err = "missing response field".to_string();
                            }
                        } else {
                            last_err = "invalid JSON response".to_string();
                        }
                    } else {
                        if resp.status().as_u16() >= 500 {
                            last_err = format!("API overloaded (status {})", resp.status());
                            tokio::time::sleep(Duration::from_secs(2)).await;
                            continue;
                        }
                        last_err = format!("local LLM error (status {})", resp.status());
                    }
                }
                Err(e) => {
                    last_err = e;
                }
            }
            cb.record_failure();
            tokio::time::sleep(Duration::from_secs(1)).await;
        }

        Err(format!("failed after 3 retries: {}", last_err))
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

