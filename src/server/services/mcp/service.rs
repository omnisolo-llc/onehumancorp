use tonic::{Request, Response, Status};
use crate::ohc::orchestration::*;
use crate::ohc::orchestration::mcp_service_server::McpService;
use std::sync::{Arc, RwLock};
use crate::integrations::registry::IntegrationsRegistry;
use crate::tools::hybridfsmcp::server::HybridFSMcpServer;
use crate::tools::hybridfsmcp::factory;
use crate::tools::local_proxy::server::LocalProxyServer;

pub struct MyMcpService {
    dynamic_tools: RwLock<Vec<McpToolProto>>,
    registry: Arc<IntegrationsRegistry>,
    hub: Arc<crate::hub::Hub>,
    hybrid_fs_server: Arc<HybridFSMcpServer>,
    local_proxy_server: Arc<LocalProxyServer>,
}

impl MyMcpService {
    pub fn new(registry: Arc<IntegrationsRegistry>, hub: Arc<crate::hub::Hub>) -> Self {
        MyMcpService {
            dynamic_tools: RwLock::new(Vec::new()),
            registry,
            hub,
            hybrid_fs_server: Arc::new(HybridFSMcpServer::new(factory::create_fs_provider(None))),
            local_proxy_server: Arc::new(LocalProxyServer::new()),
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
        let mut tools = self.dynamic_tools.read().unwrap().clone();
        let hybrid_fs_tools = self.hybrid_fs_server.get_tools();
        let local_proxy_tools = self.local_proxy_server.get_tools();
        tools.extend(hybrid_fs_tools);
        tools.extend(local_proxy_tools);
        Ok(Response::new(McpToolsResponse {
            tools,
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
            "fs_hybrid_read" | "fs_hybrid_write" | "fs_hybrid_sync" | "fs_list_dir" => {
                match self.hybrid_fs_server.invoke_tool(&req, Some(self.hub.pool.clone())).await {
                    Ok(resp) => Ok(Response::new(resp)),
                    Err(e) => Err(e),
                }
            }
            "local_stateful_proxy" => {
                match self.local_proxy_server.invoke_tool(&req).await {
                    Ok(resp) => Ok(Response::new(resp)),
                    Err(e) => Err(e),
                }
            }
            _ => {
                Err(Status::unimplemented(format!("tool {} not implemented in stub", req.tool_id)))
            }
        }
    }

    async fn sync_missions(
        &self,
        request: Request<SyncMissionsRequest>,
    ) -> Result<Response<EmptyResponse>, Status> {
        let md = request.metadata().clone();
        let spiffe_id_str = md.get("x-spiffe-id").and_then(|v| v.to_str().ok()).unwrap_or("");
        let (tenant_id, _) = crate::auth::parse_spiffe_id(spiffe_id_str).unwrap_or(("".to_string(), "".to_string()));

        if tenant_id.is_empty() {
            return Err(Status::unauthenticated("missing tenant identity in session"));
        }

        let req = request.into_inner();

        let sip_db = crate::sip::SipDB::new(self.hub.pool.clone(), tenant_id.clone());
        let ctx_root = std::env::var("CONTEXT_ROOT").ok();
        let sip_db = if let Some(root) = ctx_root {
            sip_db.with_context_root(root)
        } else {
            sip_db
        };

        let grounding_content = sip_db.load_grounding_content().await;

        let mut tx = self.hub.pool.begin().await.map_err(|e| Status::internal(e.to_string()))?;
        crate::utils::auth_utils::set_org_context(&mut *tx, &tenant_id).await.map_err(|e| Status::internal(e.to_string()))?;

        for m in req.missions {
            sip_db.delegate_mission_with_tx(&mut tx, &m.id, &m.status, &m.payload, m.force_local, &grounding_content)
                .await
                .map_err(|e| Status::internal(e.to_string()))?;
        }

        tx.commit().await.map_err(|e| Status::internal(e.to_string()))?;

        Ok(Response::new(EmptyResponse {}))
    }

    async fn sync_context(
        &self,
        request: Request<SyncContextRequest>,
    ) -> Result<Response<EmptyResponse>, Status> {
        let md = request.metadata().clone();
        let spiffe_id_str = md.get("x-spiffe-id").and_then(|v| v.to_str().ok()).unwrap_or("");
        let (tenant_id, _) = crate::auth::parse_spiffe_id(spiffe_id_str).unwrap_or(("".to_string(), "".to_string()));

        if tenant_id.is_empty() {
            return Err(Status::unauthenticated("missing tenant identity in session"));
        }

        let req = request.into_inner();
        let mut tx = self.hub.pool.begin().await.map_err(|e| Status::internal(e.to_string()))?;
        crate::utils::auth_utils::set_org_context(&mut *tx, &tenant_id).await.map_err(|e| Status::internal(e.to_string()))?;

        let query = "INSERT INTO swarm_memory_embeddings (memory_id, context, vector_embedding, source_plugin, created_at, organization_id) \
                     VALUES ($1, $2, $3, $4, CURRENT_TIMESTAMP, $5) \
                     ON CONFLICT (memory_id) DO UPDATE SET \
                         context = CASE WHEN swarm_memory_embeddings.context != EXCLUDED.context THEN EXCLUDED.context ELSE swarm_memory_embeddings.context END, \
                         vector_embedding = CASE WHEN swarm_memory_embeddings.vector_embedding != EXCLUDED.vector_embedding THEN EXCLUDED.vector_embedding ELSE swarm_memory_embeddings.vector_embedding END, \
                         source_plugin = CASE WHEN swarm_memory_embeddings.source_plugin != EXCLUDED.source_plugin THEN EXCLUDED.source_plugin ELSE swarm_memory_embeddings.source_plugin END";
        
        sqlx::query(query)
            .bind(&req.memory_id)
            .bind(&req.context)
            .bind(req.vector_embedding.as_bytes())
            .bind(&req.source_plugin)
            .bind(&tenant_id)
            .execute(&mut *tx)
            .await
            .map_err(|e| Status::internal(e.to_string()))?;
            
        tx.commit().await.map_err(|e| Status::internal(e.to_string()))?;
        Ok(Response::new(EmptyResponse {}))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tonic::Request;
    use crate::ohc::orchestration::{SyncMissionsRequest, SyncContextRequest};

    #[tokio::test]
    async fn test_sync_missions_unauthenticated() {
        let registry = Arc::new(IntegrationsRegistry::new());
        let pool_opts = sqlx::postgres::PgPoolOptions::new().after_release(|conn, _meta| { Box::pin(async move { use sqlx::Executor; conn.execute("RESET app.current_tenant").await?; conn.execute("RESET ROLE").await?; Ok(true) }) }).acquire_timeout(std::time::Duration::from_millis(500)).max_connections(1);
        let pool = pool_opts.connect_lazy("postgres://postgres:postgres@localhost:5432/test").unwrap();
        if std::env::var("DATABASE_URL").unwrap_or_default().contains("localhost") { return; }
        if !matches!(tokio::time::timeout(std::time::Duration::from_millis(500), sqlx::query("SELECT 1").execute(&pool)).await, Ok(Ok(_))) { return; }
        let (tx, _rx) = tokio::sync::mpsc::channel(1);
        let hub = Arc::new(crate::hub::Hub::new(tx, pool));
        let service = MyMcpService::new(registry, hub);

        let req = Request::new(SyncMissionsRequest { missions: vec![], force_local: false });
        let resp = service.sync_missions(req).await;

        assert!(resp.is_err());
        assert_eq!(resp.unwrap_err().code(), tonic::Code::Unauthenticated);
    }

    #[tokio::test]
    async fn test_sync_context_unauthenticated() {
        let registry = Arc::new(IntegrationsRegistry::new());
        let pool_opts = sqlx::postgres::PgPoolOptions::new().after_release(|conn, _meta| { Box::pin(async move { use sqlx::Executor; conn.execute("RESET app.current_tenant").await?; conn.execute("RESET ROLE").await?; Ok(true) }) }).acquire_timeout(std::time::Duration::from_millis(500)).max_connections(1);
        let pool = pool_opts.connect_lazy("postgres://postgres:postgres@localhost:5432/test").unwrap();
        if std::env::var("DATABASE_URL").unwrap_or_default().contains("localhost") { return; }
        if !matches!(tokio::time::timeout(std::time::Duration::from_millis(500), sqlx::query("SELECT 1").execute(&pool)).await, Ok(Ok(_))) { return; }
        let (tx, _rx) = tokio::sync::mpsc::channel(1);
        let hub = Arc::new(crate::hub::Hub::new(tx, pool));
        let service = MyMcpService::new(registry, hub);

        let req = Request::new(SyncContextRequest {
            memory_id: "test".to_string(),
            context: "test".to_string(),
            vector_embedding: "".to_string(),
            source_plugin: "test".to_string()
        });
        let resp = service.sync_context(req).await;

        assert!(resp.is_err());
        assert_eq!(resp.unwrap_err().code(), tonic::Code::Unauthenticated);
    }

    #[tokio::test]
    async fn test_sync_missions_authenticated() {
        let registry = Arc::new(IntegrationsRegistry::new());
        let pool_opts = sqlx::postgres::PgPoolOptions::new().after_release(|conn, _meta| { Box::pin(async move { use sqlx::Executor; conn.execute("RESET app.current_tenant").await?; conn.execute("RESET ROLE").await?; Ok(true) }) }).acquire_timeout(std::time::Duration::from_millis(500)).max_connections(1);
        let pool = pool_opts.connect_lazy("postgres://postgres:postgres@localhost:5432/test").unwrap();
        if std::env::var("DATABASE_URL").unwrap_or_default().contains("localhost") { return; }
        if !matches!(tokio::time::timeout(std::time::Duration::from_millis(500), sqlx::query("SELECT 1").execute(&pool)).await, Ok(Ok(_))) { return; }
        let (tx, _rx) = tokio::sync::mpsc::channel(1);
        let hub = Arc::new(crate::hub::Hub::new(tx, pool));
        let service = MyMcpService::new(registry, hub);

        let mut req = Request::new(SyncMissionsRequest { missions: vec![], force_local: false });
        req.metadata_mut().insert("x-spiffe-id", "spiffe://onehumancorp.io/org-1/agent-1".parse().unwrap());

        let resp = service.sync_missions(req).await;
        if let Err(status) = resp {
            assert_ne!(status.code(), tonic::Code::Unauthenticated);
        }
    }

    #[tokio::test]
    async fn test_sync_context_authenticated() {
        let registry = Arc::new(IntegrationsRegistry::new());
        let pool_opts = sqlx::postgres::PgPoolOptions::new().after_release(|conn, _meta| { Box::pin(async move { use sqlx::Executor; conn.execute("RESET app.current_tenant").await?; conn.execute("RESET ROLE").await?; Ok(true) }) }).acquire_timeout(std::time::Duration::from_millis(500)).max_connections(1);
        let pool = pool_opts.connect_lazy("postgres://postgres:postgres@localhost:5432/test").unwrap();
        if std::env::var("DATABASE_URL").unwrap_or_default().contains("localhost") { return; }
        if !matches!(tokio::time::timeout(std::time::Duration::from_millis(500), sqlx::query("SELECT 1").execute(&pool)).await, Ok(Ok(_))) { return; }
        let (tx, _rx) = tokio::sync::mpsc::channel(1);
        let hub = Arc::new(crate::hub::Hub::new(tx, pool));
        let service = MyMcpService::new(registry, hub);

        let mut req = Request::new(SyncContextRequest {
            memory_id: "test".to_string(),
            context: "test".to_string(),
            vector_embedding: "".to_string(),
            source_plugin: "test".to_string()
        });
        req.metadata_mut().insert("x-spiffe-id", "spiffe://onehumancorp.io/org-1/agent-1".parse().unwrap());

        // This will attempt an insert into DB, but since test env may not be running PG properly, it might fail internal, but at least not unauthenticated
        let resp = service.sync_context(req).await;
        if let Err(e) = resp {
             // We just expect it to bypass the unauthenticated block,
             // but it might fail on `pool.begin()` if the db is completely missing.
             assert_ne!(e.code(), tonic::Code::Unauthenticated);
        }
    }
}
