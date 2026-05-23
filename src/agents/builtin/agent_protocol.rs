use axum::{
    extract::{Path, State},
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use uuid::Uuid;

use crate::codex_runner::Runner;
use crate::agent::AgentRunConfig;

/// Agent Protocol Task Schema
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Task {
    pub task_id: String,
    pub input: Option<String>,
    pub additional_input: Option<serde_json::Value>,
    pub artifacts: Vec<Artifact>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskRequestBody {
    pub input: Option<String>,
    pub additional_input: Option<serde_json::Value>,
}

/// Agent Protocol Step Schema
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Step {
    pub task_id: String,
    pub step_id: String,
    pub name: Option<String>,
    pub status: StepStatus,
    pub output: Option<String>,
    pub additional_output: Option<serde_json::Value>,
    pub artifacts: Vec<Artifact>,
    pub is_last: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum StepStatus {
    Created,
    Running,
    Completed,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StepRequestBody {
    pub input: Option<String>,
    pub additional_input: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Artifact {
    pub artifact_id: String,
    pub file_name: String,
    pub relative_path: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskListResponse {
    pub tasks: Vec<Task>,
    pub pagination: Pagination,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StepListResponse {
    pub steps: Vec<Step>,
    pub pagination: Pagination,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Pagination {
    pub total_items: usize,
    pub total_pages: usize,
    pub current_page: usize,
    pub page_size: usize,
}

/// Thread-safe in-memory store for Agent Protocol
pub struct AgentProtocolState {
    pub runner: Arc<Runner>,
    pub tasks: RwLock<HashMap<String, Task>>,
    pub steps: RwLock<HashMap<String, Vec<Step>>>,
}

pub type SharedState = Arc<AgentProtocolState>;

/// POST /ap/v1/agent/tasks
async fn create_task(
    State(state): State<SharedState>,
    Json(payload): Json<TaskRequestBody>,
) -> Json<Task> {
    let task_id = Uuid::new_v4().to_string();
    let task = Task {
        task_id: task_id.clone(),
        input: payload.input,
        additional_input: payload.additional_input,
        artifacts: vec![],
    };

    let mut tasks = state.tasks.write().await;
    tasks.insert(task_id.clone(), task.clone());

    let mut steps = state.steps.write().await;
    steps.insert(task_id, vec![]);

    Json(task)
}

/// GET /ap/v1/agent/tasks
async fn list_tasks(State(state): State<SharedState>) -> Json<TaskListResponse> {
    let tasks = state.tasks.read().await;
    let task_list: Vec<Task> = tasks.values().cloned().collect();

    Json(TaskListResponse {

        pagination: Pagination {
            total_items: task_list.len(),

            total_pages: 1,
            current_page: 1,
            page_size: std::cmp::max(1, task_list.len()),
        },
        tasks: task_list,
    })
}

/// GET /ap/v1/agent/tasks/{task_id}
async fn get_task(
    State(state): State<SharedState>,
    Path(task_id): Path<String>,
) -> Result<Json<Task>, axum::http::StatusCode> {
    let tasks = state.tasks.read().await;
    match tasks.get(&task_id) {
        Some(task) => Ok(Json(task.clone())),
        None => Err(axum::http::StatusCode::NOT_FOUND),
    }
}

/// POST /ap/v1/agent/tasks/{task_id}/steps
async fn execute_step(
    State(state): State<SharedState>,
    Path(task_id): Path<String>,
    Json(payload): Json<StepRequestBody>,
) -> Result<Json<Step>, axum::http::StatusCode> {
    // Check if task exists
    {
        let tasks = state.tasks.read().await;
        if !tasks.contains_key(&task_id) {
            return Err(axum::http::StatusCode::NOT_FOUND);
        }
    }

    let step_id = Uuid::new_v4().to_string();
    let input = payload.input.unwrap_or_default();

    // To simulate a step, we execute a single blocking execution via the runner
    // (For Agent Protocol, normally it's an event-driven step, but for this demo,
    // run_sync_blocking will fulfill the prompt execution).
    let runner_clone = state.runner.clone();

    let output = match tokio::task::spawn_blocking(move || {
        let cfg = AgentRunConfig::default();
        runner_clone.run_sync_blocking(&cfg, &input)
    }).await {
        Ok(Ok(out)) => out,
        Ok(Err(e)) => format!("Error: {}", e),
        Err(e) => format!("Panic: {}", e),
    };

    let step = Step {
        task_id: task_id.clone(),
        step_id: step_id.clone(),
        name: Some("Execution Step".to_string()),
        status: StepStatus::Completed,
        output: Some(output),
        additional_output: None,
        artifacts: vec![],
        is_last: true, // Simplified for this implementation
    };

    let mut steps = state.steps.write().await;
    if let Some(task_steps) = steps.get_mut(&task_id) {
        task_steps.push(step.clone());
    }

    Ok(Json(step))
}

/// GET /ap/v1/agent/tasks/{task_id}/steps
async fn list_steps(
    State(state): State<SharedState>,
    Path(task_id): Path<String>,
) -> Result<Json<StepListResponse>, axum::http::StatusCode> {
    let steps = state.steps.read().await;
    match steps.get(&task_id) {
        Some(task_steps) => {
            let total = task_steps.len();
            Ok(Json(StepListResponse {
                steps: task_steps.clone(),
                pagination: Pagination {
                    total_items: total,
                    total_pages: 1,
                    current_page: 1,
                    page_size: std::cmp::max(1, total),
                },
            }))
        }
        None => Err(axum::http::StatusCode::NOT_FOUND),
    }
}

/// GET /ap/v1/agent/tasks/{task_id}/steps/{step_id}
async fn get_step(
    State(state): State<SharedState>,
    Path((task_id, step_id)): Path<(String, String)>,
) -> Result<Json<Step>, axum::http::StatusCode> {
    let steps = state.steps.read().await;
    if let Some(task_steps) = steps.get(&task_id) {
        if let Some(step) = task_steps.iter().find(|s| s.step_id == step_id) {
            return Ok(Json(step.clone()));
        }
    }
    Err(axum::http::StatusCode::NOT_FOUND)
}


pub fn create_agent_protocol_router(runner: Arc<Runner>) -> Router {
    let state = Arc::new(AgentProtocolState {
        runner,
        tasks: RwLock::new(HashMap::new()),
        steps: RwLock::new(HashMap::new()),
    });

    Router::new()
        .route("/ap/v1/agent/tasks", post(create_task).get(list_tasks))
        .route("/ap/v1/agent/tasks/{task_id}", get(get_task))
        .route("/ap/v1/agent/tasks/{task_id}/steps", post(execute_step).get(list_steps))
        .route("/ap/v1/agent/tasks/{task_id}/steps/{step_id}", get(get_step))
        .with_state(state)
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::{
        body::Body,
        http::{Request, StatusCode},
    };
    use tower::ServiceExt;
    use std::sync::Arc;
    use ohc_builtin_agent_core::types::{ChatRequest, ChatResponse, Message, Usage};
    use crate::llm::LlmClient;
    use crate::agent::Agent;

    struct MockLlmClient;

    #[async_trait::async_trait]
    impl LlmClient for MockLlmClient {
        async fn chat(&self, req: ChatRequest) -> Result<ChatResponse, Box<dyn std::error::Error + Send + Sync>> {
            Ok(ChatResponse {
                message: Message::assistant("Step executed successfully"),
                usage: Usage::default(),
                stop_reason: "stop".to_string(),
                response_id: Some("mock-id".to_string()),
            })
        }
    }

    async fn build_test_app() -> Router {
        let client = Arc::new(MockLlmClient);
        let agent = Arc::new(Agent::new(client, vec![]));
        let runner = Arc::new(Runner::new(agent));
        create_agent_protocol_router(runner)
    }

    #[tokio::test]
    async fn test_agent_protocol_flow() {
        let app = build_test_app().await;

        // 1. Create Task
        let req_body = serde_json::json!({
            "input": "Write a hello world program"
        });

        let response = app.clone()
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/ap/v1/agent/tasks")
                    .header("Content-Type", "application/json")
                    .body(Body::from(req_body.to_string()))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
        let body_bytes = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap();
        let task: Task = serde_json::from_slice(&body_bytes).unwrap();

        assert_eq!(task.input.unwrap(), "Write a hello world program");
        let task_id = task.task_id;

        // 2. Execute Step
        let step_req_body = serde_json::json!({
            "input": "Execute it"
        });

        let response = app.clone()
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri(&format!("/ap/v1/agent/tasks/{}/steps", task_id))
                    .header("Content-Type", "application/json")
                    .body(Body::from(step_req_body.to_string()))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
        let body_bytes = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap();
        let step: Step = serde_json::from_slice(&body_bytes).unwrap();

        assert_eq!(step.task_id, task_id);
        assert_eq!(step.output.unwrap(), "Step executed successfully");
        assert_eq!(step.status, StepStatus::Completed);
        let step_id = step.step_id;

        // 3. List Steps
        let response = app.clone()
            .oneshot(
                Request::builder()
                    .method("GET")
                    .uri(&format!("/ap/v1/agent/tasks/{}/steps", task_id))
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
        let body_bytes = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap();
        let step_list: StepListResponse = serde_json::from_slice(&body_bytes).unwrap();

        assert_eq!(step_list.steps.len(), 1);
        assert_eq!(step_list.steps[0].step_id, step_id);

        // 4. Get Task
        let response = app.clone()
            .oneshot(
                Request::builder()
                    .method("GET")
                    .uri(&format!("/ap/v1/agent/tasks/{}", task_id))
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
        let body_bytes = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap();
        let retrieved_task: Task = serde_json::from_slice(&body_bytes).unwrap();

        assert_eq!(retrieved_task.task_id, task_id);
    }
}
