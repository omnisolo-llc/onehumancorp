#![allow(dead_code)]

use std::sync::Mutex;
use std::sync::OnceLock;
use std::time::{Duration, Instant};
use serde::{Serialize, Deserialize};

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
                return false;
            }
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

static GLOBAL_PLANE_CIRCUIT_BREAKER: OnceLock<CircuitBreaker> = OnceLock::new();

fn get_circuit_breaker() -> &'static CircuitBreaker {
    GLOBAL_PLANE_CIRCUIT_BREAKER.get_or_init(|| CircuitBreaker::new(3, Duration::from_secs(30)))
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Issue {
    pub id: String,
    pub name: String,
    pub description_html: String,
    pub state: String,
    pub priority: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct IssueListResponse {
    results: Vec<Issue>,
}

pub struct Client {
    pub base_url: String,
    pub api_key: String,
    pub workspace: String,
    pub project: String,
    http_client: reqwest::Client,
    cb: &'static CircuitBreaker,
}

impl Client {
    pub fn new_from_env() -> Self {
        let base_url = std::env::var("PLANE_URL")
            .unwrap_or_else(|_| "http://plane-api:8000".to_string());
        let api_key = std::env::var("PLANE_API_KEY").unwrap_or_default();
        let workspace = std::env::var("PLANE_WORKSPACE").unwrap_or_default();
        let project = std::env::var("PLANE_PROJECT").unwrap_or_default();

        Client {
            base_url,
            api_key,
            workspace,
            project,
            http_client: reqwest::Client::new(),
            cb: get_circuit_breaker(),
        }
    }

    pub async fn list_open_issues(&self) -> Result<Vec<Issue>, String> {
        if self.workspace.is_empty() || self.project.is_empty() {
            return Err("plane client: workspace and project must be set".to_string());
        }

        if !self.cb.allow() {
            return Err("plane API circuit breaker is open".to_string());
        }

        let path = format!("/api/v1/workspaces/{}/projects/{}/issues/?state=open", self.workspace, self.project);
        let url = format!("{}{}", self.base_url, path);

        let mut req = self.http_client.get(&url)
            .header("Accept", "application/json");

        if !self.api_key.is_empty() {
            req = req.header("x-api-key", &self.api_key);
        }

        let resp = req.send().await.map_err(|e| {
            self.cb.record_failure();
            e.to_string()
        })?;

        if resp.status().is_server_error() {
            self.cb.record_failure();
        } else {
            self.cb.record_success();
        }

        if !resp.status().is_success() {
            let status = resp.status();
            let err_body = resp.text().await.unwrap_or_default();
            return Err(format!("plane API GET {} returned {}: {}", url, status, err_body));
        }

        let result: IssueListResponse = resp.json().await.map_err(|e| e.to_string())?;
        Ok(result.results)
    }

    pub async fn update_issue_status(&self, issue_id: &str, state_id: &str) -> Result<(), String> {
        if !self.cb.allow() {
            return Err("plane API circuit breaker is open".to_string());
        }

        let path = format!("/api/v1/workspaces/{}/projects/{}/issues/{}/", self.workspace, self.project, issue_id);
        let url = format!("{}{}", self.base_url, path);

        let body = serde_json::json!({
            "state": state_id,
        });

        let mut req = self.http_client.patch(&url)
            .header("Content-Type", "application/json")
            .header("Accept", "application/json")
            .json(&body);

        if !self.api_key.is_empty() {
            req = req.header("x-api-key", &self.api_key);
        }

        let resp = req.send().await.map_err(|e| {
            self.cb.record_failure();
            e.to_string()
        })?;

        if resp.status().is_server_error() {
            self.cb.record_failure();
        } else {
            self.cb.record_success();
        }

        if !resp.status().is_success() {
            let status = resp.status();
            let err_body = resp.text().await.unwrap_or_default();
            return Err(format!("plane API PATCH {} returned {}: {}", url, status, err_body));
        }

        Ok(())
    }
}
