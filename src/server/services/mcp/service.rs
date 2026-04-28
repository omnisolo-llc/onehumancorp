use tonic::{Request, Response, Status};
use crate::ohc::orchestration::*;
use crate::ohc::orchestration::mcp_service_server::McpService;
use std::sync::{Arc, RwLock};
use crate::integrations::registry::IntegrationsRegistry;
use crate::integrations::rate_limiter::rate_limiter::*;
use std::env;

pub struct MyMcpService {
    dynamic_tools: RwLock<Vec<McpToolProto>>,
    registry: Arc<IntegrationsRegistry>,
    hub: Arc<crate::hub::Hub>,
    rate_limiter: Box<dyn RateLimiterManager>,
}

impl MyMcpService {
    pub fn new(registry: Arc<IntegrationsRegistry>, hub: Arc<crate::hub::Hub>) -> Self {
        let is_cloud = env::var("OHC_MULTITENANT").unwrap_or_default() == "true";
        let rate_limiter = crate::integrations::rate_limiter::rate_limiter::create_rate_limiter(
            is_cloud,
            None,
            100, // 100 requests capacity
            10.0 // 10 requests per second
        );

        MyMcpService {
            dynamic_tools: RwLock::new(Vec::new()),
            registry,
            hub,
            rate_limiter,
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

        crate::auth::grpc::validate_spiffe_id(&req.spiffe_id)?;

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
        let tenant_id = request.metadata()
            .get("organization_id")
            .and_then(|v| v.to_str().ok())
            .unwrap_or("system")
            .to_string();

        let req = request.into_inner();
        
        if req.tool_id.is_empty() {
            return Err(Status::invalid_argument("toolId is required"));
        }

        // Apply Rate Limiting
        let bucket = format!("tool:{}", req.tool_id);

        match self.rate_limiter.request_tokens(&tenant_id, &bucket, 1).await {
            Ok(true) => {
                // Proceed
            }
            Ok(false) => {
                return Err(Status::resource_exhausted(format!("Rate limit exceeded for tool {}", req.tool_id)));
            }
            Err(e) => {
                return Err(Status::internal(format!("Rate limiter error: {}", e)));
            }
        }

        return match req.tool_id.as_str() {
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

#[cfg(test)]
mod tests {
    use super::*;
    use tonic::Request;
    use crate::ohc::orchestration::{McpRegisterRequest, McpToolProto};

    use std::sync::Arc;
    use crate::integrations::registry::IntegrationsRegistry;
    use tokio::sync::mpsc;
    use sqlx::postgres::PgPoolOptions;

    async fn setup_test_service() -> MyMcpService {
        let registry = Arc::new(IntegrationsRegistry::new());
        // For testing, we create a dummy pool and hub. In unit tests,
        // the pool might fail if no Postgres is available, but sqlx AnyPool can't be used easily.
        // We will just create a dummy pool connecting to a non-existent DB since register_tool doesn't hit DB.
        let pool = PgPoolOptions::new()
            .connect_lazy("postgres://postgres:postgres@localhost:5432/ohc_test")
            .unwrap();
        let (tx, _) = mpsc::channel(1);
        let hub = Arc::new(crate::hub::Hub::new(tx, pool));
        MyMcpService::new(registry, hub)
    }

    #[tokio::test]
    async fn test_register_tool_valid_spiffe() {
        let service = setup_test_service().await;
        let req = McpRegisterRequest {
            tool: Some(McpToolProto {
                id: "test-tool".to_string(),
                name: "Test Tool".to_string(),
                description: "A test tool".to_string(),
                category: "testing".to_string(),
                status: "active".to_string(),
            }),
            spiffe_id: "spiffe://onehumancorp.io/agent/test".to_string(),
        };

        let response = service.register_tool(Request::new(req)).await;
        assert!(response.is_ok());
        assert_eq!(response.unwrap().into_inner().status, "registered");
    }

    #[tokio::test]
    async fn test_register_tool_invalid_spiffe() {
        let service = setup_test_service().await;
        let req = McpRegisterRequest {
            tool: Some(McpToolProto {
                id: "test-tool".to_string(),
                name: "Test Tool".to_string(),
                description: "A test tool".to_string(),
                category: "testing".to_string(),
                status: "active".to_string(),
            }),
            spiffe_id: "spiffe://evil.com/agent/test".to_string(),
        };

        let response = service.register_tool(Request::new(req)).await;
        assert!(response.is_err());
        assert_eq!(response.unwrap_err().code(), tonic::Code::PermissionDenied);
    }
}
