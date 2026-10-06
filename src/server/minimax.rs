use ::server_pricing::compression::minify_json_prompt;
use ::server_pricing::deduplication::{DeduplicationResult, RequestDeduplicator};
use ::server_pricing::prompt_caching::PromptCache;
use serde::Serialize;
use std::pin::Pin;
use std::sync::Mutex;
use std::sync::OnceLock;
use std::time::{Duration, Instant};
use tokio_stream::Stream;

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
            if let Some(last) = *last_failure
                && last.elapsed() > self.reset_timeout
            {
                *failures = 0; // Reset failures so we can retry properly
                return true;
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
    embed_url: String,
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

impl MinimaxClient {
    fn validate_credentials(&self) -> Result<(), String> {
        let key = self.api_key.trim().to_ascii_lowercase();
        if key.is_empty()
            || key == "fake-key"
            || key.starts_with("dummy")
            || key.starts_with("ci-")
            || key.starts_with("mock")
            || key.starts_with("test")
            || key.contains("placeholder")
        {
            return Err("MiniMax requires a configured provider credential".into());
        }
        Ok(())
    }

    pub fn new(api_key: String) -> Self {
        MinimaxClient {
            api_key,
            url: "https://api.minimax.io/v1/chat/completions".to_string(),
            embed_url: "https://api.minimax.chat/v1/embeddings".to_string(),
            cache: PromptCache::new(Duration::from_secs(300)),
            deduplicator: std::sync::Arc::new(RequestDeduplicator::new(Duration::from_secs(5))), // 5 minute TTL
        }
    }

    pub async fn reason(&self, prompt: &str) -> Result<String, String> {
        self.validate_credentials()?;
        let prompt_clone = prompt.to_string();
        let deduplicator = self.deduplicator.clone();

        let result = deduplicator
            .deduplicate(&prompt_clone, || async {
                self.internal_reason(&prompt_clone)
                    .await
                    .map(|resp| DeduplicationResult { response: resp })
            })
            .await?;

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
        if let (Some(cached), _cost_cents) = self
            .cache
            .get_with_cost_cents(&optimized_prompt, "minimax-text-01")
        {
            tracing::info!("Prompt cache hit (saved ~{} tokens)", cached.token_count); // pii-safe
            return Ok(cached.text);
        }

        let cb = get_circuit_breaker();
        if !cb.allow() {
            return Err("circuit breaker open".to_string());
        }

        let client = provider_http_client()?;

        let request_body = MinimaxRequest {
            model: std::env::var("MINIMAX_MODEL").unwrap_or_else(|_| "MiniMax-M3".to_string()),
            messages: vec![MinimaxMessage {
                role: "user".to_string(),
                content: optimized_prompt.clone(),
            }],
            stream: Some(false),
        };

        let content = async {
            let body = send_inference(
                client
                    .post(&self.url)
                    .bearer_auth(&self.api_key)
                    .json(&request_body),
            )
            .await?;
            let result: serde_json::Value =
                serde_json::from_slice(&body).map_err(|_| "MiniMax returned malformed JSON")?;
            completed_minimax_text(&result)
        }
        .await
        .inspect_err(|_| cb.record_failure())?;
        cb.record_success();
        self.cache
            .set(&optimized_prompt, &content, optimized_prompt.len() / 4);
        Ok(content)
    }

    pub async fn reason_stream(
        &self,
        prompt: &str,
    ) -> Pin<Box<dyn Stream<Item = Result<String, String>> + Send>> {
        if let Err(error) = self.validate_credentials() {
            return Box::pin(tokio_stream::once(Err(error)));
        }
        let optimized_prompt = if prompt.starts_with('{') {
            minify_json_prompt(prompt)
        } else {
            PromptCache::truncate_context(prompt, 2000)
        };
        if let (Some(cached), _) = self
            .cache
            .get_with_cost_cents(&optimized_prompt, "minimax-text-01")
        {
            return Box::pin(tokio_stream::once(Ok(cached.text)));
        }
        let api_key = self.api_key.clone();
        let url = self.url.clone();
        let request_body = MinimaxRequest {
            model: std::env::var("MINIMAX_MODEL").unwrap_or_else(|_| "MiniMax-M3".to_string()),
            messages: vec![MinimaxMessage {
                role: "user".to_string(),
                content: optimized_prompt,
            }],
            stream: Some(true),
        };
        let (tx, rx) = tokio::sync::mpsc::channel(100);
        tokio::spawn(async move {
            if let Err(error) = stream_minimax(&url, &api_key, &request_body, &tx).await {
                let _ = tx.send(Err(error)).await;
            }
        });
        Box::pin(tokio_stream::wrappers::ReceiverStream::new(rx))
    }

    pub async fn generate_embedding(&self, text: &str) -> Result<Vec<f32>, String> {
        self.validate_credentials()?;
        validate_embedding_input(text)?;
        let cb = get_circuit_breaker();
        if !cb.allow() {
            return Err("circuit breaker open".into());
        }
        let request_body = serde_json::json!({
            "model": "embo-01", "type": "db", "texts": [text]
        });
        let embedding = async {
            let body = send_inference(
                provider_http_client()?
                    .post(&self.embed_url)
                    .bearer_auth(&self.api_key)
                    .json(&request_body),
            )
            .await?;
            let result: serde_json::Value =
                serde_json::from_slice(&body).map_err(|_| "MiniMax returned malformed JSON")?;
            validate_minimax_envelope(&result)?;
            let vectors = result["vectors"]
                .as_array()
                .filter(|vectors| vectors.len() == 1)
                .ok_or("MiniMax must return exactly one vector for one input")?;
            parse_embedding(&vectors[0])
        }
        .await
        .inspect_err(|_| cb.record_failure())?;
        cb.record_success();
        Ok(embedding)
    }
}

fn provider_http_client() -> Result<reqwest::Client, String> {
    reqwest::Client::builder()
        .timeout(Duration::from_secs(60))
        .redirect(reqwest::redirect::Policy::none())
        .build()
        .map_err(|_| "Cannot configure provider transport".into())
}

const MAX_PROVIDER_RESPONSE_BYTES: usize = 10 * 1024 * 1024;
const MAX_PROVIDER_TEXT_BYTES: usize = 8 * 1024 * 1024;
const MAX_EMBEDDING_INPUT_BYTES: usize = 1024 * 1024;
const MAX_EMBEDDING_DIMENSIONS: usize = 65_536;

async fn send_inference(request: reqwest::RequestBuilder) -> Result<Vec<u8>, String> {
    // Even a timeout or HTTP 5xx may follow accepted, billable inference.
    // Never redispatch automatically; callers must make an explicit new attempt.
    let mut response = request
        .send()
        .await
        .map_err(|_| "Provider outcome is unknown; no automatic retry was made")?;
    if !response.status().is_success() {
        return Err(format!(
            "Provider returned HTTP {}; no automatic retry was made",
            response.status().as_u16()
        ));
    }
    if response
        .content_length()
        .is_some_and(|length| length > MAX_PROVIDER_RESPONSE_BYTES as u64)
    {
        return Err("Provider response exceeded the safety limit".into());
    }
    let mut body = Vec::new();
    while let Some(chunk) = response.chunk().await.map_err(
        |_| "Provider response was interrupted; outcome is unknown and no automatic retry was made",
    )? {
        if body.len().saturating_add(chunk.len()) > MAX_PROVIDER_RESPONSE_BYTES {
            return Err("Provider response exceeded the safety limit".into());
        }
        body.extend_from_slice(&chunk);
    }
    Ok(body)
}

fn validate_embedding_input(text: &str) -> Result<(), String> {
    if text.trim().is_empty() || text.len() > MAX_EMBEDDING_INPUT_BYTES {
        return Err("Embedding input must be nonempty and within the size limit".into());
    }
    Ok(())
}

fn validate_minimax_envelope(value: &serde_json::Value) -> Result<(), String> {
    if value.get("error").is_some_and(|error| !error.is_null()) {
        return Err("MiniMax returned an API error".into());
    }
    if let Some(base) = value.get("base_resp")
        && base.get("status_code").and_then(serde_json::Value::as_i64) != Some(0)
    {
        return Err("MiniMax returned an unsuccessful provider status".into());
    }
    Ok(())
}

fn completed_minimax_text(value: &serde_json::Value) -> Result<String, String> {
    validate_minimax_envelope(value)?;
    let choice = &value["choices"][0];
    if choice["finish_reason"].as_str() != Some("stop") {
        return Err("MiniMax did not complete the requested response".into());
    }
    let text = choice["message"]["content"]
        .as_str()
        .filter(|text| !text.trim().is_empty() && text.len() <= MAX_PROVIDER_TEXT_BYTES)
        .ok_or("MiniMax did not return nonempty text")?;
    Ok(text.to_string())
}

fn parse_embedding(value: &serde_json::Value) -> Result<Vec<f32>, String> {
    let array = value
        .as_array()
        .filter(|array| !array.is_empty() && array.len() <= MAX_EMBEDDING_DIMENSIONS)
        .ok_or("Provider did not return a nonempty embedding")?;
    array
        .iter()
        .map(|value| {
            value
                .as_f64()
                .map(|number| number as f32)
                .filter(|number| number.is_finite())
                .ok_or_else(|| "Provider embedding contains an invalid numeric value".into())
        })
        .collect()
}

// SSE transport chunks are arbitrary bytes, not complete JSON or UTF-8 frames.
// Keep partial lines/events until their delimiters arrive and require both the
// provider's successful finish reason and the protocol's final marker.
#[derive(Default)]
struct MinimaxStreamDecoder {
    pending: Vec<u8>,
    data: String,
    has_text: bool,
    finished: bool,
    ended: bool,
}
impl MinimaxStreamDecoder {
    fn push(&mut self, bytes: &[u8]) -> Result<Vec<String>, String> {
        const MAX_EVENT: usize = 1024 * 1024;
        if self.pending.len().saturating_add(bytes.len()) > MAX_EVENT {
            return Err("MiniMax stream event exceeded the safety limit".into());
        }
        self.pending.extend_from_slice(bytes);
        let mut output = Vec::new();
        while let Some(end) = self.pending.iter().position(|byte| *byte == b'\n') {
            let line: Vec<_> = self.pending.drain(..=end).collect();
            let line = std::str::from_utf8(&line[..line.len() - 1])
                .map_err(|_| "MiniMax stream contains invalid UTF-8")?;
            let line = line.strip_suffix('\r').unwrap_or(line);
            if line.is_empty() {
                if let Some(content) = self.event()? {
                    output.push(content);
                }
                if self.ended {
                    break;
                }
            } else if let Some(data) = line.strip_prefix("data:") {
                let data = data.strip_prefix(' ').unwrap_or(data);
                if self.data.len().saturating_add(data.len()).saturating_add(1) > MAX_EVENT {
                    return Err("MiniMax stream event exceeded the safety limit".into());
                }
                self.data.push_str(data);
                self.data.push('\n');
            }
        }
        Ok(output)
    }

    fn event(&mut self) -> Result<Option<String>, String> {
        if self.data.is_empty() {
            return Ok(None);
        }
        let data = std::mem::take(&mut self.data);
        let data = data.trim_end_matches('\n');
        if data == "[DONE]" {
            if !self.finished || !self.has_text {
                return Err("MiniMax stream ended without completed nonempty text".into());
            }
            self.ended = true;
            return Ok(None);
        }
        let value: serde_json::Value =
            serde_json::from_str(data).map_err(|_| "MiniMax stream contains malformed JSON")?;
        validate_minimax_envelope(&value)?;
        let choices = value["choices"]
            .as_array()
            .ok_or("MiniMax stream is missing choices")?;
        // The optional usage-only event has no choices.
        let Some(choice) = choices.first() else {
            return Ok(None);
        };
        if choices.len() != 1
            || choice
                .get("index")
                .is_some_and(|index| index.as_u64() != Some(0))
        {
            return Err("MiniMax stream returned an unexpected choice sequence".into());
        }
        if self.finished {
            return Err("MiniMax stream returned choice data after completion".into());
        }
        if let Some(reason) = choice
            .get("finish_reason")
            .filter(|reason| !reason.is_null())
        {
            if reason.as_str() != Some("stop") {
                return Err("MiniMax stream stopped before completing the response".into());
            }
            self.finished = true;
        }
        let content = choice["delta"].get("content");
        match content {
            None | Some(serde_json::Value::Null) => Ok(None),
            Some(serde_json::Value::String(text)) => {
                self.has_text |= !text.trim().is_empty();
                Ok((!text.is_empty()).then(|| text.clone()))
            }
            Some(_) => Err("MiniMax stream contains invalid text content".into()),
        }
    }

    fn finish(self) -> Result<(), String> {
        if self.ended {
            Ok(())
        } else {
            Err("MiniMax stream was interrupted before confirmed completion".into())
        }
    }
}

async fn stream_minimax(
    url: &str,
    api_key: &str,
    request: &MinimaxRequest,
    tx: &tokio::sync::mpsc::Sender<Result<String, String>>,
) -> Result<(), String> {
    let response = provider_http_client()?
        .post(url)
        .bearer_auth(api_key)
        .json(request)
        .send()
        .await
        .map_err(|_| "MiniMax stream outcome is unknown; no automatic retry was made")?;
    if !response.status().is_success() {
        return Err(format!(
            "MiniMax stream returned HTTP {}",
            response.status().as_u16()
        ));
    }
    let mut stream = response.bytes_stream();
    let mut decoder = MinimaxStreamDecoder::default();
    let mut received = 0usize;
    use tokio_stream::StreamExt;
    while let Some(chunk) = stream.next().await {
        let chunk = chunk.map_err(|_| "MiniMax stream response was interrupted")?;
        received = received.saturating_add(chunk.len());
        if received > 10 * 1024 * 1024 {
            return Err("MiniMax stream exceeded the safety limit".into());
        }
        for content in decoder.push(&chunk)? {
            if tx.send(Ok(content)).await.is_err() {
                return Ok(());
            }
        }
        if decoder.ended {
            break;
        }
    }
    decoder.finish()
}

#[path = "local_generation.rs"]
pub mod local_generation;

pub struct LocalLLMClient {
    endpoint: String,
    embed_endpoint: String,
    model: String,
    cache: PromptCache,
    deduplicator: std::sync::Arc<RequestDeduplicator>,
}

impl Default for LocalLLMClient {
    fn default() -> Self {
        Self::new()
    }
}

impl LocalLLMClient {
    /// Preserve observed quantities for callers that expose usage. The older
    /// string-only API remains non-billable compatibility, not a zero-cost meter.
    pub async fn reason_with_usage(
        &self,
        prompt: &str,
        maximum_output: i32,
    ) -> Result<local_generation::ObservedGeneration, String> {
        local_generation::generate(&self.endpoint, &self.model, prompt, maximum_output).await
    }

    pub fn new() -> Self {
        let endpoint = std::env::var("OMNISOLO_LOCAL_LLM_ENDPOINT")
            .unwrap_or_else(|_| "http://127.0.0.1:11434/api/generate".to_string());
        let embed_endpoint = std::env::var("OMNISOLO_LOCAL_LLM_EMBED_ENDPOINT")
            .unwrap_or_else(|_| "http://127.0.0.1:11434/api/embeddings".to_string());
        let model =
            std::env::var("OMNISOLO_LOCAL_MODEL_NAME").unwrap_or_else(|_| "llama3".to_string());

        LocalLLMClient {
            endpoint,
            embed_endpoint,
            model,
            cache: PromptCache::new(Duration::from_secs(300)),
            deduplicator: std::sync::Arc::new(RequestDeduplicator::new(Duration::from_secs(5))),
        }
    }

    pub async fn reason(&self, prompt: &str) -> Result<String, String> {
        let prompt_clone = prompt.to_string();
        let deduplicator = self.deduplicator.clone();

        let result = deduplicator
            .deduplicate(&prompt_clone, || async {
                self.internal_reason(&prompt_clone)
                    .await
                    .map(|resp| DeduplicationResult { response: resp })
            })
            .await?;

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

        if let (Some(cached), _cost_cents) = self
            .cache
            .get_with_cost_cents(&optimized_prompt, &self.model)
        {
            tracing::info!("Prompt cache hit (saved ~{} tokens)", cached.token_count); // pii-safe
            return Ok(cached.text);
        }

        let client = provider_http_client()?;
        let req_body = serde_json::json!({
            "model": self.model,
            "prompt": optimized_prompt,
            "stream": false,
        });

        let result = async {
            let body = send_inference(client.post(&self.endpoint).json(&req_body)).await?;
            local_generation::parse_generation(&body, &self.model)
        }
        .await
        .inspect_err(|_| cb.record_failure())?;
        cb.record_success();
        self.cache
            .set(&optimized_prompt, &result.text, optimized_prompt.len() / 4);
        Ok(result.text)
    }

    pub async fn generate_embedding(&self, text: &str) -> Result<Vec<f32>, String> {
        validate_embedding_input(text)?;
        let req_body = serde_json::json!({ "model": self.model, "prompt": text });
        let body = send_inference(
            provider_http_client()?
                .post(&self.embed_endpoint)
                .json(&req_body),
        )
        .await?;
        let result: serde_json::Value =
            serde_json::from_slice(&body).map_err(|_| "Local model returned malformed JSON")?;
        parse_embedding(&result["embedding"])
    }
}

#[cfg(test)]
#[path = "minimax_tests.rs"]
pub(crate) mod truthful_provider_tests;
