use axum::{Json, Router, routing::post};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;

use crate::agent::{Agent, AgentRunConfig};
use crate::visual_workflow::{WorkflowExecutor, WorkflowGraph};

/// HTTP Server AppState
pub struct VisualWorkflowState {
    pub default_agent: Arc<Agent>,
    pub tools: Vec<crate::tools::Tool>,
    pub sub_agents: HashMap<String, Arc<Agent>>,
    pub default_config: AgentRunConfig,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkflowRunRequest {
    pub graph: WorkflowGraph,
    pub inputs: HashMap<String, String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkflowSchemaResponse {
    pub schema: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkflowValidateRequest {
    pub graph: WorkflowGraph,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkflowRunResponse {
    pub success: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

pub async fn handle_workflow_run(
    axum::extract::State(state): axum::extract::State<Arc<VisualWorkflowState>>,
    Json(payload): Json<WorkflowRunRequest>,
) -> Json<WorkflowRunResponse> {
    let executor = WorkflowExecutor::new(
        payload.graph,
        state.default_agent.clone(),
        state.tools.clone(),
        state.sub_agents.clone(),
        state.default_config.clone(),
        None,
    );

    match executor.execute(payload.inputs).await {
        Ok(result) => Json(WorkflowRunResponse {
            success: true,
            result: Some(result),
            error: None,
        }),
        Err(e) => Json(WorkflowRunResponse {
            success: false,
            result: None,
            error: Some(e),
        }),
    }
}

pub async fn handle_workflow_schema(
    axum::extract::State(_state): axum::extract::State<Arc<VisualWorkflowState>>,
) -> Json<WorkflowSchemaResponse> {
    let graph = WorkflowGraph { nodes: vec![], edges: vec![] };
    use crate::visual_workflow::BlockConnectUI;
    Json(WorkflowSchemaResponse {
        schema: graph.generate_ui_schema(),
    })
}

pub async fn handle_workflow_validate(
    axum::extract::State(_state): axum::extract::State<Arc<VisualWorkflowState>>,
    Json(req): Json<WorkflowValidateRequest>,
) -> Json<WorkflowRunResponse> {
    if req.graph.nodes.is_empty() {
        return Json(WorkflowRunResponse { success: false, result: None, error: Some("Empty graph".to_string()) });
    }
    Json(WorkflowRunResponse { success: true, result: Some("Valid".to_string()), error: None })
}

pub fn create_router(state: Arc<VisualWorkflowState>) -> Router {
    Router::new()
        .route("/api/workflow/run", post(handle_workflow_run))
        .with_state(state)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::{ChatRequest, ChatResponse, Message, Usage};
    use crate::visual_workflow::{Edge, Node, NodeType};
    use axum::{
        body::Body,
        http::{Request, StatusCode},
    };
    use tower::ServiceExt;

    struct MockVisualClientLlm;
    #[async_trait::async_trait]
    impl crate::llm::LlmClient for MockVisualClientLlm {
        async fn chat(
            &self,
            req: ChatRequest,
        ) -> Result<ChatResponse, Box<dyn std::error::Error + Send + Sync>> {
            let last_user = req.messages.last().unwrap().content.clone();
            Ok(ChatResponse {
                message: Message::assistant(format!("Mocked: {}", last_user)),
                usage: Usage::default(),
                stop_reason: "stop".to_string(),
                response_id: Some("id1".to_string()),
            })
        }
    }

    #[tokio::test]
    async fn test_visual_workflow_client_endpoint() {
        let agent = Arc::new(Agent::new(Arc::new(MockVisualClientLlm), vec![]));

        let state = Arc::new(VisualWorkflowState {
            default_agent: agent,
            tools: vec![],
            sub_agents: HashMap::new(),
            default_config: AgentRunConfig::default(),
        });

        let app = create_router(state);

        let graph = WorkflowGraph {
            nodes: vec![
                Node {
                    id: "in".to_string(),
                    node_type: NodeType::Input {
                        name: "input_var".to_string(),
                    },
                },
                Node {
                    id: "llm1".to_string(),
                    node_type: NodeType::Llm {
                        prompt_template: "Input was: {{in}}".to_string(),
                    },
                },
                Node {
                    id: "out".to_string(),
                    node_type: NodeType::Output,
                },
            ],
            edges: vec![
                Edge {
                    source: "in".to_string(),
                    target: "llm1".to_string(),
                },
                Edge {
                    source: "llm1".to_string(),
                    target: "out".to_string(),
                },
            ],
        };

        let mut inputs = HashMap::new();
        inputs.insert("in".to_string(), "test_data".to_string());

        let req_body = serde_json::to_string(&WorkflowRunRequest { graph, inputs }).unwrap();

        let response = app
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/api/workflow/run")
                    .header("Content-Type", "application/json")
                    .body(Body::from(req_body))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let body_bytes = axum::body::to_bytes(response.into_body(), usize::MAX)
            .await
            .unwrap();
        let resp: WorkflowRunResponse = serde_json::from_slice(&body_bytes).unwrap();

        assert!(resp.success);
        assert_eq!(resp.result.unwrap(), "Mocked: Input was: test_data");
    }
}
