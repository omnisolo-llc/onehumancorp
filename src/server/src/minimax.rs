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

fn get_circuit_breaker() -> &'static CircuitBreaker {
    GLOBAL_CIRCUIT_BREAKER.get_or_init(|| CircuitBreaker::new(3, Duration::from_secs(120)))
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
                        Ok(choice.message.content.clone())
                    } else {
                        Err("empty response from minimax".to_string())
                    }
                } else {
                    cb.record_failure();
                    let status = resp.status();
                    let text = resp.text().await.unwrap_or_default();
                    Err(format!("API error (status {}): {}", status, text))
                }
            }
            Err(e) => {
                cb.record_failure();
                Err(e.to_string())
            }
        }
    }
}
