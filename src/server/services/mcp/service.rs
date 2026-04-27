use tonic::{Request, Response, Status};
use crate::ohc::orchestration::*;
use crate::ohc::orchestration::mcp_service_server::McpService;
use std::sync::{Arc, RwLock};
use crate::integrations::registry::IntegrationsRegistry;

pub struct MyMcpService {
    dynamic_tools: RwLock<Vec<McpToolProto>>,
    registry: Arc<IntegrationsRegistry>,
}

impl MyMcpService {
    pub fn new(registry: Arc<IntegrationsRegistry>) -> Self {
        MyMcpService {
            dynamic_tools: RwLock::new(Vec::new()),
            registry,
        }
    }
}

#[tonic::async_trait]
impl McpService for MyMcpService {
    async fn register_tool(
        &self,
        request: Request<McpRegisterRequest>,
    ) -> Result<Response<McpRegisterResponse>, Status> {
        let req = request.into_inner();
        let tool = req.tool.ok_or_else(|| Status::invalid_argument("tool is required"))?;

        if tool.id.is_empty() || tool.name.is_empty() {
            return Err(Status::invalid_argument("tool ID and name are required"));
        }

        let mut tools = self.dynamic_tools.write().unwrap();

        for t in tools.iter_mut() {
            if t.id == tool.id {
                *t = tool.clone();
                return Ok(Response::new(McpRegisterResponse {
                    status: "updated".to_string(),
                    tool: Some(tool),
                }));
            }
        }

        tools.push(tool.clone());
        Ok(Response::new(McpRegisterResponse {
            status: "registered".to_string(),
            tool: Some(tool),
        }))
    }

    async fn get_tools(
        &self,
        _request: Request<EmptyRequest>,
    ) -> Result<Response<McpToolsResponse>, Status> {
        let tools = self.dynamic_tools.read().unwrap();
        Ok(Response::new(McpToolsResponse {
            tools: tools.clone(),
        }))
    }

    async fn invoke_tool(
        &self,
        request: Request<McpInvokeRequest>,
    ) -> Result<Response<McpInvokeResponse>, Status> {
        let req = request.into_inner();

        if req.tool_id.is_empty() {
            return Err(Status::invalid_argument("toolId is required"));
        }

        match req.tool_id.as_str() {
            "telegram-mcp" | "slack-mcp" | "teams-mcp" => {
                let params: serde_json::Value = serde_json::from_str(&req.params)
                    .map_err(|e| Status::invalid_argument(format!("invalid JSON params: {}", e)))?;

                let channel = params["channel"].as_str().unwrap_or_default();
                let from_agent = params["from_agent"].as_str().unwrap_or("system");
                let content = params["content"].as_str().ok_or_else(|| Status::invalid_argument("content is required"))?;
                let thread_id = params["thread_id"].as_str().unwrap_or_default();

                let msg = self.registry.send_chat_message(&req.tool_id, channel, from_agent, content, thread_id)
                    .map_err(|e| Status::internal(e))?;

                let resp_payload = serde_json::to_string(&msg).unwrap();
                Ok(Response::new(McpInvokeResponse { payload: resp_payload }))
            }
            "git-mcp" => {
                let params: serde_json::Value = serde_json::from_str(&req.params)
                    .map_err(|e| Status::invalid_argument(format!("invalid JSON params: {}", e)))?;

                let repo = params["repository"].as_str().unwrap_or_default();
                let title = params["title"].as_str().unwrap_or_default();
                let body = params["body"].as_str().unwrap_or_default();
                let source = params["source_branch"].as_str().unwrap_or_default();
                let target = params["target_branch"].as_str().unwrap_or("main");
                let created_by = params["created_by"].as_str().unwrap_or_default();

                let pr = self.registry.create_pull_request(&req.tool_id, repo, title, body, source, target, created_by)
                    .map_err(|e| Status::internal(e))?;

                let resp_payload = serde_json::to_string(&pr).unwrap();
                Ok(Response::new(McpInvokeResponse { payload: resp_payload }))
            }
            "jira-mcp" => {
                let params: serde_json::Value = serde_json::from_str(&req.params)
                    .map_err(|e| Status::invalid_argument(format!("invalid JSON params: {}", e)))?;

                let project = params["project"].as_str().unwrap_or_default();
                let title = params["title"].as_str().unwrap_or_default();
                let description = params["description"].as_str().unwrap_or_default();
                let created_by = params["created_by"].as_str().unwrap_or_default();
                let priority = params["priority"].as_str().unwrap_or("medium");

                let mut labels = Vec::new();
                if let Some(arr) = params["labels"].as_array() {
                    for l in arr {
                        if let Some(s) = l.as_str() {
                            labels.push(s.to_string());
                        }
                    }
                }

                let issue = self.registry.create_issue(&req.tool_id, project, title, description, created_by, priority, labels)
                    .map_err(|e| Status::internal(e))?;

                let resp_payload = serde_json::to_string(&issue).unwrap();
                Ok(Response::new(McpInvokeResponse { payload: resp_payload }))
            }
            "crdt_push" => {
                Ok(Response::new(McpInvokeResponse { payload: req.params }))
            }
            "crdt_pull" => {
                let mock_data = serde_json::json!({"crdt_state": "latest_mocked_state"});
                let resp_payload = serde_json::to_string(&mock_data).unwrap();
                Ok(Response::new(McpInvokeResponse { payload: resp_payload }))
            }
            "hybrid_sync" => {
                let params: serde_json::Value = serde_json::from_str(&req.params)
                    .map_err(|e| Status::invalid_argument(format!("invalid JSON params: {}", e)))?;

                let action = params["action"].as_str().unwrap_or_default();

                match action {
                    "sync_state" => {
                        let resp_payload = serde_json::to_string(&serde_json::json!({"status": "synced"})).unwrap();
                        Ok(Response::new(McpInvokeResponse { payload: resp_payload }))
                    }
                    "resolve_conflicts" => {
                        let local_hlc = params["local_hlc"].as_f64().unwrap_or(0.0);
                        let remote_hlc = params["remote_hlc"].as_f64().unwrap_or(0.0);

                        let winner = if local_hlc > remote_hlc {
                            "local"
                        } else if remote_hlc > local_hlc {
                            "remote"
                        } else {
                            "tie_broken_by_remote"
                        };

                        let resp_payload = serde_json::to_string(&serde_json::json!({"status": "resolved", "winner": winner})).unwrap();
                        Ok(Response::new(McpInvokeResponse { payload: resp_payload }))
                    }
                    _ => Err(Status::invalid_argument(format!("unknown action: {}", action))),
                }
            }
            "obsidian" => {
                let mock_result = serde_json::json!({"status": "mocked", "message": "Obsidian tool mocked in Cloud mode"});
                let resp_payload = serde_json::to_string(&mock_result).unwrap();
                Ok(Response::new(McpInvokeResponse { payload: resp_payload }))
            }
            "sync_audit_logs_to_cloud" => {
                let resp_payload = serde_json::to_string(&serde_json::json!({"status": "success"})).unwrap();
                Ok(Response::new(McpInvokeResponse { payload: resp_payload }))
            }
            "get_config" => {
                let resp_payload = serde_json::to_string(&serde_json::json!({"value": "mock_value"})).unwrap();
                Ok(Response::new(McpInvokeResponse { payload: resp_payload }))
            }
            "sync_config_to_cloud" => {
                let resp_payload = serde_json::to_string(&serde_json::json!({"status": "success"})).unwrap();
                Ok(Response::new(McpInvokeResponse { payload: resp_payload }))
            }
            _ => {
                Err(Status::unimplemented(format!("tool {} not implemented in stub", req.tool_id)))
            }
        }
    }
}
