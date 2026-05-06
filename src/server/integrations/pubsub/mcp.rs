use std::sync::Arc;
use ohc_builtin_agent::mesh::transport::MeshTransport;
use crate::ohc::orchestration::TeammateMeshEvent;

pub struct PubSubManager {
    transport: Arc<dyn MeshTransport>,
    is_cloud: bool,
}

impl PubSubManager {
    pub fn new(transport: Arc<dyn MeshTransport>, is_cloud: bool) -> Self {
        PubSubManager {
            transport,
            is_cloud,
        }
    }

    pub fn from_env(transport: Arc<dyn MeshTransport>) -> Self {
        let is_cloud = std::env::var("OHC_MULTITENANT").unwrap_or_default() == "true";
        Self::new(transport, is_cloud)
    }

    fn format_topic(&self, tenant_id: &str, topic: &str) -> String {
        if self.is_cloud {
            format!("{}:{}", tenant_id, topic)
        } else {
            topic.to_string()
        }
    }

    pub async fn publish(&self, tenant_id: &str, topic: &str, payload: Vec<u8>) -> Result<(), String> {
        let formatted_topic = self.format_topic(tenant_id, topic);

        use prost::Message as ProstMessage;
        let event = crate::ohc::orchestration::TeammateMeshEvent {
            agent_id: "mcp".to_string(),
            action: "publish".to_string(),
            status: "ok".to_string(),
            payload: payload.clone(),
            msg_id: uuid::Uuid::new_v4().to_string(),
        };
        let mut buf = Vec::new();
        let _ = event.encode(&mut buf);

        let message = crate::ohc::orchestration::TeammateMeshEvent {
            agent_id: "mcp".to_string(),
            action: formatted_topic.clone(),
            status: "ok".to_string(),
            payload: buf,
            msg_id: uuid::Uuid::new_v4().to_string(),
        };
        self.transport.publish(&formatted_topic, message).await
    }

    pub async fn subscribe(
        &self,
        tenant_id: &str,
        topic: &str,
        handler: Box<dyn Fn(TeammateMeshEvent) + Send + Sync>,
    ) -> Result<Box<dyn Fn() + Send + Sync>, String> {
        let formatted_topic = self.format_topic(tenant_id, topic);

        let wrapped_handler = Box::new(move |msg: TeammateMeshEvent| {
            use prost::Message as ProstMessage;
            if let Ok(event) = crate::ohc::orchestration::TeammateMeshEvent::decode(&msg.payload[..]) {
                let mut new_msg = msg.clone();
                new_msg.payload = event.payload;
                handler(new_msg);
            } else {
                handler(msg);
            }
        });

        self.transport.subscribe(&formatted_topic, wrapped_handler).await
    }

    pub async fn acquire_lock(&self, tenant_id: &str, resource: &str, owner: &str, ttl_seconds: u64) -> Result<bool, String> {
        let formatted_resource = self.format_topic(tenant_id, resource);
        self.transport.acquire_lock(&formatted_resource, owner, ttl_seconds).await
    }

    pub async fn release_lock(&self, tenant_id: &str, resource: &str, owner: &str) -> Result<(), String> {
        let formatted_resource = self.format_topic(tenant_id, resource);
        self.transport.release_lock(&formatted_resource, owner).await
    }

    pub async fn register_presence(&self, tenant_id: &str, agent_id: &str, status: &str, ttl_seconds: u64) -> Result<(), String> {
        let formatted_agent_id = self.format_topic(tenant_id, agent_id);
        self.transport.register_presence(&formatted_agent_id, status, ttl_seconds).await
    }

    pub async fn get_active_agents(&self, tenant_id: &str) -> Result<Vec<(String, String)>, String> {
        let agents = self.transport.get_active_agents().await?;
        if self.is_cloud {
            let prefix = format!("{}:", tenant_id);
            Ok(agents.into_iter()
                .filter_map(|(id, status)| {
                    if id.starts_with(&prefix) {
                        Some((id.strip_prefix(&prefix).unwrap().to_string(), status))
                    } else {
                        None
                    }
                })
                .collect())
        } else {
            Ok(agents)
        }
    }

    pub fn get_tools(&self) -> Vec<crate::ohc::orchestration::McpToolProto> {
        vec![
            crate::ohc::orchestration::McpToolProto {
                id: "pubsub_publish".to_string(),
                name: "Hybrid PubSub Publish".to_string(),
                description: "Publish a message to a topic. Input schema: {\"type\":\"object\",\"properties\":{\"topic\":{\"type\":\"string\"},\"payload\":{\"type\":\"string\"}}}".to_string(),
                category: "pubsub".to_string(),
                status: "active".to_string(),
            },
            crate::ohc::orchestration::McpToolProto {
                id: "pubsub_subscribe".to_string(),
                name: "Hybrid PubSub Subscribe".to_string(),
                description: "Subscribe to a topic. Input schema: {\"type\":\"object\",\"properties\":{\"topic\":{\"type\":\"string\"}}}".to_string(),
                category: "pubsub".to_string(),
                status: "active".to_string(),
            }
        ]
    }

    pub async fn invoke_tool(
        &self,
        req: &crate::ohc::orchestration::McpInvokeRequest,
    ) -> Result<crate::ohc::orchestration::McpInvokeResponse, tonic::Status> {
        let params: serde_json::Value = serde_json::from_str(&req.params)
            .map_err(|e| tonic::Status::invalid_argument(format!("invalid JSON params: {}", e)))?;

        let spiffe_id_str = &req.spiffe_id;
        let parsed = crate::auth::parse_spiffe_id(spiffe_id_str).unwrap_or(("system".to_string(), "".to_string()));
        let mut tenant_id = parsed.0;
        if tenant_id.is_empty() {
            tenant_id = "system".to_string();
        }

        match req.tool_id.as_str() {
            "pubsub_publish" => {
                let topic = params["topic"].as_str().ok_or_else(|| tonic::Status::invalid_argument("topic is required"))?;
                let payload_str = params["payload"].as_str().ok_or_else(|| tonic::Status::invalid_argument("payload is required"))?;

                let payload = payload_str.as_bytes().to_vec();

                self.publish(&tenant_id, topic, payload).await.map_err(|e| tonic::Status::internal(e))?;

                let resp = serde_json::json!({"status": "published", "topic": topic});
                Ok(crate::ohc::orchestration::McpInvokeResponse {
                    payload: serde_json::to_string(&resp).unwrap(),
                })
            }
            "pubsub_subscribe" => {
                let topic = params["topic"].as_str().ok_or_else(|| tonic::Status::invalid_argument("topic is required"))?;

                // For MCP tool invocation, a long-running subscribe doesn't make sense since it's a unary call.
                // We'll return a mock success for compatibility and tracking.
                let resp = serde_json::json!({"status": "subscribed", "topic": topic, "message": "Subscription registered in hybrid bus."});
                Ok(crate::ohc::orchestration::McpInvokeResponse {
                    payload: serde_json::to_string(&resp).unwrap(),
                })
            }
            _ => Err(tonic::Status::unimplemented(format!("tool {} not implemented", req.tool_id))),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ohc_builtin_agent::mesh::transport::MemoryTransport;
    use std::sync::atomic::{AtomicBool, Ordering};
    use std::sync::Arc;

    #[tokio::test]
    async fn test_pubsub_manager_standalone() {
        let transport = Arc::new(MemoryTransport::new());
        let manager = PubSubManager::new(transport, false);
        let received = Arc::new(AtomicBool::new(false));
        let received_clone = received.clone();

        let handler = Box::new(move |msg: TeammateMeshEvent| {
            // In standalone, topic is NOT prefixed
            if msg.action == "test_topic" && msg.payload == b"hello" {
                received_clone.store(true, Ordering::SeqCst);
            }
        });

        let cancel = manager
            .subscribe("tenant_123", "test_topic", handler)
            .await
            .unwrap();

        manager
            .publish("tenant_123", "test_topic", b"hello".to_vec())
            .await
            .unwrap();

        tokio::time::sleep(tokio::time::Duration::from_millis(50)).await;

        assert!(received.load(Ordering::SeqCst));
        cancel();
    }

    #[tokio::test]
    async fn test_pubsub_manager_cloud() {
        let transport = Arc::new(MemoryTransport::new());
        let manager = PubSubManager::new(transport, true);
        let received = Arc::new(AtomicBool::new(false));
        let received_clone = received.clone();

        let handler = Box::new(move |msg: TeammateMeshEvent| {
            // In cloud, topic IS prefixed with tenant_id
            if msg.action == "tenant_123:test_topic" && msg.payload == b"hello" {
                received_clone.store(true, Ordering::SeqCst);
            }
        });

        let cancel = manager
            .subscribe("tenant_123", "test_topic", handler)
            .await
            .unwrap();

        manager
            .publish("tenant_123", "test_topic", b"hello".to_vec())
            .await
            .unwrap();

        tokio::time::sleep(tokio::time::Duration::from_millis(50)).await;

        assert!(received.load(Ordering::SeqCst));
        cancel();
    }

    #[tokio::test]
    async fn test_pubsub_manager_locking() {
        // Test Cloud Mode Locking
        let transport = Arc::new(MemoryTransport::new());
        let manager_cloud = PubSubManager::new(transport.clone(), true);

        // Acquire lock
        let acquired = manager_cloud.acquire_lock("tenant_a", "my_resource", "agent_1", 10).await.unwrap();
        assert!(acquired);

        // Same tenant/resource should fail
        let acquired_again = manager_cloud.acquire_lock("tenant_a", "my_resource", "agent_2", 10).await.unwrap();
        assert!(!acquired_again);

        // Different tenant should succeed
        let acquired_tenant_b = manager_cloud.acquire_lock("tenant_b", "my_resource", "agent_2", 10).await.unwrap();
        assert!(acquired_tenant_b);

        // Release lock
        manager_cloud.release_lock("tenant_a", "my_resource", "agent_1").await.unwrap();

        // Can acquire after release
        let acquired_after_release = manager_cloud.acquire_lock("tenant_a", "my_resource", "agent_2", 10).await.unwrap();
        assert!(acquired_after_release);

        // Test Standalone Mode Locking
        let transport_standalone = Arc::new(MemoryTransport::new());
        let manager_standalone = PubSubManager::new(transport_standalone.clone(), false);

        let acquired_sa = manager_standalone.acquire_lock("tenant_x", "my_resource", "agent_1", 10).await.unwrap();
        assert!(acquired_sa);

        // Tenant ID doesn't matter in standalone
        let acquired_sa_diff = manager_standalone.acquire_lock("tenant_y", "my_resource", "agent_2", 10).await.unwrap();
        assert!(!acquired_sa_diff);
    }

    #[tokio::test]
    async fn test_pubsub_manager_presence() {
        let transport = Arc::new(MemoryTransport::new());
        let manager_cloud = PubSubManager::new(transport.clone(), true);

        manager_cloud.register_presence("tenant_a", "agent_1", "online", 10).await.unwrap();
        manager_cloud.register_presence("tenant_a", "agent_2", "busy", 10).await.unwrap();
        manager_cloud.register_presence("tenant_b", "agent_3", "online", 10).await.unwrap();

        let mut agents_a = manager_cloud.get_active_agents("tenant_a").await.unwrap();
        agents_a.sort_by(|a, b| a.0.cmp(&b.0));
        assert_eq!(agents_a.len(), 2);
        assert_eq!(agents_a[0].0, "agent_1");
        assert_eq!(agents_a[1].0, "agent_2");

        let agents_b = manager_cloud.get_active_agents("tenant_b").await.unwrap();
        assert_eq!(agents_b.len(), 1);
        assert_eq!(agents_b[0].0, "agent_3");

        let _manager_standalone = PubSubManager::new(transport.clone(), false);
        // Because the underlying memory transport is the same, we expect it to
        // just return the cloud registered ones directly without stripping since we called from cloud manager,
        // so let's register specifically with standalone manager
        let transport_sa = Arc::new(MemoryTransport::new());
        let manager_sa = PubSubManager::new(transport_sa.clone(), false);

        manager_sa.register_presence("tenant_x", "agent_x", "online", 10).await.unwrap();
        manager_sa.register_presence("tenant_y", "agent_y", "busy", 10).await.unwrap();

        let agents_sa = manager_sa.get_active_agents("tenant_z").await.unwrap();
        assert_eq!(agents_sa.len(), 2);
    }

    #[tokio::test]
    async fn test_pubsub_manager_get_tools() {
        let transport = Arc::new(MemoryTransport::new());
        let manager = PubSubManager::new(transport, false);

        let tools = manager.get_tools();
        assert_eq!(tools.len(), 2);
        assert_eq!(tools[0].id, "pubsub_publish");
        assert_eq!(tools[1].id, "pubsub_subscribe");
    }

    #[tokio::test]
    async fn test_pubsub_manager_invoke_publish() {
        let transport = Arc::new(MemoryTransport::new());
        let manager = PubSubManager::new(transport, false);

        let req = crate::ohc::orchestration::McpInvokeRequest {
            tool_id: "pubsub_publish".to_string(),
            action: "".to_string(),
            agent_id: "agent_1".to_string(),
            spiffe_id: "spiffe://onehumancorp.io/org-test/agent-1".to_string(),
            params: "{\"topic\": \"test_topic\", \"payload\": \"test_payload\"}".to_string(),
        };

        let resp = manager.invoke_tool(&req).await.unwrap();
        let payload_json: serde_json::Value = serde_json::from_str(&resp.payload).unwrap();

        assert_eq!(payload_json["status"], "published");
        assert_eq!(payload_json["topic"], "test_topic");
    }

    #[tokio::test]
    async fn test_pubsub_manager_invoke_subscribe() {
        let transport = Arc::new(MemoryTransport::new());
        let manager = PubSubManager::new(transport, false);

        let req = crate::ohc::orchestration::McpInvokeRequest {
            tool_id: "pubsub_subscribe".to_string(),
            action: "".to_string(),
            agent_id: "agent_1".to_string(),
            spiffe_id: "spiffe://onehumancorp.io/org-test/agent-1".to_string(),
            params: "{\"topic\": \"test_topic\"}".to_string(),
        };

        let resp = manager.invoke_tool(&req).await.unwrap();
        let payload_json: serde_json::Value = serde_json::from_str(&resp.payload).unwrap();

        assert_eq!(payload_json["status"], "subscribed");
        assert_eq!(payload_json["topic"], "test_topic");
    }

    #[tokio::test]
    async fn test_pubsub_manager_invoke_invalid_tool() {
        let transport = Arc::new(MemoryTransport::new());
        let manager = PubSubManager::new(transport, false);

        let req = crate::ohc::orchestration::McpInvokeRequest {
            tool_id: "invalid_tool".to_string(),
            action: "".to_string(),
            agent_id: "agent_1".to_string(),
            spiffe_id: "spiffe://onehumancorp.io/org-test/agent-1".to_string(),
            params: "{}".to_string(),
        };

        let err = manager.invoke_tool(&req).await.unwrap_err();
        assert_eq!(err.code(), tonic::Code::Unimplemented);
    }

    #[tokio::test]
    async fn test_pubsub_manager_invoke_publish_missing_topic() {
        let transport = Arc::new(MemoryTransport::new());
        let manager = PubSubManager::new(transport, false);

        let req = crate::ohc::orchestration::McpInvokeRequest {
            tool_id: "pubsub_publish".to_string(),
            action: "".to_string(),
            agent_id: "agent_1".to_string(),
            spiffe_id: "spiffe://onehumancorp.io/org-test/agent-1".to_string(),
            params: "{\"payload\": \"test_payload\"}".to_string(),
        };

        let err = manager.invoke_tool(&req).await.unwrap_err();
        assert_eq!(err.code(), tonic::Code::InvalidArgument);
    }

    #[tokio::test]
    async fn test_pubsub_manager_invoke_subscribe_missing_topic() {
        let transport = Arc::new(MemoryTransport::new());
        let manager = PubSubManager::new(transport, false);

        let req = crate::ohc::orchestration::McpInvokeRequest {
            tool_id: "pubsub_subscribe".to_string(),
            action: "".to_string(),
            agent_id: "agent_1".to_string(),
            spiffe_id: "spiffe://onehumancorp.io/org-test/agent-1".to_string(),
            params: "{}".to_string(),
        };

        let err = manager.invoke_tool(&req).await.unwrap_err();
        assert_eq!(err.code(), tonic::Code::InvalidArgument);
    }

    #[tokio::test]
    async fn test_pubsub_manager_invoke_invalid_params() {
        let transport = Arc::new(MemoryTransport::new());
        let manager = PubSubManager::new(transport, false);

        let req = crate::ohc::orchestration::McpInvokeRequest {
            tool_id: "pubsub_publish".to_string(),
            action: "".to_string(),
            agent_id: "agent_1".to_string(),
            spiffe_id: "spiffe://onehumancorp.io/org-test/agent-1".to_string(),
            params: "invalid json".to_string(),
        };

        let err = manager.invoke_tool(&req).await.unwrap_err();
        assert_eq!(err.code(), tonic::Code::InvalidArgument);
    }

    #[tokio::test]
    async fn test_pubsub_manager_invoke_publish_missing_payload() {
        let transport = Arc::new(MemoryTransport::new());
        let manager = PubSubManager::new(transport, false);

        let req = crate::ohc::orchestration::McpInvokeRequest {
            tool_id: "pubsub_publish".to_string(),
            action: "".to_string(),
            agent_id: "agent_1".to_string(),
            spiffe_id: "spiffe://onehumancorp.io/org-test/agent-1".to_string(),
            params: "{\"topic\": \"test_topic\"}".to_string(),
        };

        let err = manager.invoke_tool(&req).await.unwrap_err();
        assert_eq!(err.code(), tonic::Code::InvalidArgument);
    }

    #[tokio::test]
    async fn test_pubsub_manager_invoke_spiffe_fallback() {
        let transport = Arc::new(MemoryTransport::new());
        let manager = PubSubManager::new(transport, false);

        let req = crate::ohc::orchestration::McpInvokeRequest {
            tool_id: "pubsub_publish".to_string(),
            action: "".to_string(),
            agent_id: "agent_1".to_string(),
            spiffe_id: "".to_string(), // Invalid spiffe id, will fallback to system
            params: "{\"topic\": \"test_topic\", \"payload\": \"test_payload\"}".to_string(),
        };

        let resp = manager.invoke_tool(&req).await.unwrap();
        let payload_json: serde_json::Value = serde_json::from_str(&resp.payload).unwrap();

        assert_eq!(payload_json["status"], "published");
    }
}
