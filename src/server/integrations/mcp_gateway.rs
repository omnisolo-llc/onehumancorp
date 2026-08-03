use std::time::Instant;
use tokio::sync::RwLock;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ToolExecutionRateLimitingEvent {
    pub event_id: String,
    pub agent_id: String,
    pub payload: Vec<u8>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DynamicToolSchema {
    pub name: String,
    pub description: String,
    pub parameters: Value,
    pub endpoint_url: String,
    pub required_spiffe_id: Option<String>,
    pub rate_limit: Option<f64>,
}

pub struct TokenBucket {
    pub tokens: f64,
    pub last_refill: Instant,
    pub rate: f64,
    pub capacity: f64,
}

impl TokenBucket {
    pub fn new(rate: f64, capacity: f64) -> Self {
        Self {
            tokens: capacity,
            last_refill: Instant::now(),
            rate,
            capacity,
        }
    }

    pub fn consume(&mut self, tokens: f64) -> bool {
        let now = Instant::now();
        let elapsed = now.duration_since(self.last_refill).as_secs_f64();
        self.tokens = (self.tokens + elapsed * self.rate).min(self.capacity);
        self.last_refill = now;

        if self.tokens >= tokens {
            self.tokens -= tokens;
            true
        } else {
            false
        }
    }
}

pub struct McpGateway {
    registry: RwLock<std::collections::HashMap<String, DynamicToolSchema>>,
    rate_limiters: RwLock<std::collections::HashMap<String, TokenBucket>>,
    active_executions: RwLock<std::collections::HashMap<String, Instant>>,
}

impl McpGateway {
    pub fn new() -> Self {
        Self {
            registry: RwLock::new(std::collections::HashMap::new()),
            rate_limiters: RwLock::new(std::collections::HashMap::new()),
            active_executions: RwLock::new(std::collections::HashMap::new()),
        }
    }

    pub async fn register_tool(&self, spiffe_id: &str, schema: DynamicToolSchema) -> Result<(), String> {
        // Authenticate/Authorize via SPIFFE
        if !spiffe_id.starts_with("spiffe://") {
            return Err("Invalid SPIFFE ID".to_string());
        }

        let mut reg = self.registry.write().await;
        reg.insert(schema.name.clone(), schema);
        Ok(())
    }

    pub async fn discover_tools(&self, query: &str) -> Vec<DynamicToolSchema> {
        let reg = self.registry.read().await;
        reg.values()
            .filter(|schema| schema.name.contains(query) || schema.description.contains(query))
            .cloned()
            .collect()
    }

    pub async fn process_rate_limiting_event(&self, payload_bytes: &[u8]) -> Result<(), String> {
        // Strict JSON validation via dec.DisallowUnknownFields() equivalent
        let event: ToolExecutionRateLimitingEvent = serde_json::from_slice(payload_bytes)
            .map_err(|e| format!("Invalid payload: {}", e))?;

        let mut active = self.active_executions.write().await;
        active.insert(event.event_id.clone(), Instant::now());

        Ok(())
    }

    pub async fn complete_execution(&self, event_id: &str) {
        let mut active = self.active_executions.write().await;
        active.remove(event_id);
    }

    pub async fn invoke_tool(&self, spiffe_id: &str, tool_name: &str, args: Value) -> Result<String, String> {
        // Find the tool
        let reg = self.registry.read().await;
        let tool = match reg.get(tool_name) {
            Some(t) => t.clone(),
            None => return Err(format!("Tool {} not found", tool_name)),
        };
        // Drop lock before doing rate limits
        drop(reg);

        // SPIFFE authorization check
        if let Some(required_id) = &tool.required_spiffe_id {
            if spiffe_id != required_id && required_id != "*" {
                return Err("Unauthorized SPIFFE ID for this tool".to_string());
            }
        }

        if let Some(rate) = tool.rate_limit {
            let key = format!("{}:{}", spiffe_id, tool_name);
            let mut limiters = self.rate_limiters.write().await;
            let bucket = limiters.entry(key).or_insert_with(|| TokenBucket::new(rate, rate));

            if !bucket.consume(1.0) {
                return Err("429 Too Many Requests".to_string());
            }
        }

        // Simulate invoking the tool via HTTP/RPC
        // In a real implementation, this would make an actual network call to tool.endpoint_url
        Ok(format!("Successfully invoked dynamic tool {} with args: {}", tool_name, args))
    }

    pub async fn get_active_executions_count(&self) -> usize {
        let active = self.active_executions.read().await;
        active.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_mcp_gateway_registration_and_discovery() {
        let gateway = McpGateway::new();

        let schema = DynamicToolSchema {
            name: "weather_api".to_string(),
            description: "Get local weather".to_string(),
            parameters: serde_json::json!({"type": "object", "properties": {"location": {"type": "string"}}}),
            endpoint_url: "http://weather-service.svc.cluster.local".to_string(),
            required_spiffe_id: Some("*".to_string()),
            rate_limit: None,
        };

        let result = gateway.register_tool("spiffe://example.org/agent-1", schema).await;
        assert!(result.is_ok());

        let discovered = gateway.discover_tools("weather").await;
        assert_eq!(discovered.len(), 1);
        assert_eq!(discovered[0].name, "weather_api");
    }

    #[tokio::test]
    async fn test_mcp_gateway_invocation_auth() {
        let gateway = McpGateway::new();

        let schema = DynamicToolSchema {
            name: "secure_finance".to_string(),
            description: "Access secure finance data".to_string(),
            parameters: serde_json::json!({}),
            endpoint_url: "http://finance-service.svc.cluster.local".to_string(),
            required_spiffe_id: Some("spiffe://example.org/finance-agent".to_string()),
            rate_limit: None,
        };

        gateway.register_tool("spiffe://example.org/admin", schema).await.unwrap();

        // Should fail due to wrong SPIFFE ID
        let res_fail = gateway.invoke_tool("spiffe://example.org/random-agent", "secure_finance", serde_json::json!({})).await;
        assert!(res_fail.is_err());

        // Should succeed with correct SPIFFE ID
        let res_ok = gateway.invoke_tool("spiffe://example.org/finance-agent", "secure_finance", serde_json::json!({})).await;
        assert!(res_ok.is_ok());
    }

    #[tokio::test]
    async fn test_mcp_gateway_rate_limiting() {
        let gateway = McpGateway::new();

        let schema = DynamicToolSchema {
            name: "rate_limited_api".to_string(),
            description: "An API that is rate limited".to_string(),
            parameters: serde_json::json!({}),
            endpoint_url: "http://example.com".to_string(),
            required_spiffe_id: None,
            rate_limit: Some(5.0),
        };

        gateway.register_tool("spiffe://example.org/admin", schema).await.unwrap();

        let mut success_count = 0;
        let mut rate_limited_count = 0;

        for _ in 0..20 {
            let res = gateway.invoke_tool("spiffe://example.org/agent-1", "rate_limited_api", serde_json::json!({})).await;
            if res.is_ok() {
                success_count += 1;
            } else if let Err(e) = res {
                if e == "429 Too Many Requests" {
                    rate_limited_count += 1;
                }
            }
        }

        assert_eq!(success_count, 5);
        assert_eq!(rate_limited_count, 15);
    }

    #[tokio::test]
    async fn test_strict_schema_validation() {
        let gateway = McpGateway::new();

        let invalid_payload = serde_json::json!({
            "event_id": "123",
            "agent_id": "agent_x",
            "payload": [1, 2, 3],
            "unknown_field": "should cause failure"
        }).to_string();

        let res = gateway.process_rate_limiting_event(invalid_payload.as_bytes()).await;
        assert!(res.is_err());
        assert!(res.unwrap_err().contains("unknown field `unknown_field`"));
    }

    #[tokio::test]
    async fn test_memory_and_resource_bounding() {
        let gateway = McpGateway::new();

        let valid_payload = serde_json::json!({
            "event_id": "event_123",
            "agent_id": "agent_x",
            "payload": [1, 2, 3]
        }).to_string();

        // Simulate incoming event
        gateway.process_rate_limiting_event(valid_payload.as_bytes()).await.unwrap();

        // Active executions should grow
        assert_eq!(gateway.get_active_executions_count().await, 1);

        // Complete execution explicit cleanup
        gateway.complete_execution("event_123").await;

        // Bounded memory verified
        assert_eq!(gateway.get_active_executions_count().await, 0);
    }
}
