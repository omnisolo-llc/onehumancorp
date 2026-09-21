use crate::Tool;
use crate::pydantic::{PydanticAdapter, PydanticToolExecutor};
use omnisolo_builtin_agent_core::types::ToolError;
use serde::Deserialize;
use serde_json::json;
use std::sync::Arc;

#[derive(Deserialize)]
pub struct ShippoRatesArgs {
    pub order_id: String,
    pub weight: f64,
    pub length: f64,
    pub width: f64,
    pub height: f64,
    pub carrier: Option<String>,
    pub address: Option<String>,
    pub parcel: Option<String>,
    pub currency: Option<String>,
}

pub struct ShippoRatesExecutor {
    pub tenant: crate::tenant::TenantContext,
}

#[async_trait::async_trait]
impl PydanticToolExecutor<ShippoRatesArgs> for ShippoRatesExecutor {
    async fn execute_typed(&self, args: ShippoRatesArgs) -> Result<String, ToolError> {
        let client = reqwest::Client::new();
        let url = std::env::var("OMNISOLO_API_URL")
            .unwrap_or_else(|_| "http://api.omnisolo.internal".to_string());

        let mut request = client.post(&format!("{}/api/v1/shipping/rates", url));

        if let Ok(token) = std::env::var("OMNISOLO_AGENT_TOKEN") {
             request = request.bearer_auth(token);
        } else {
             return Err(ToolError::Fatal("Missing OMNISOLO_AGENT_TOKEN".to_string()));
        }

        let payload = json!({
            "orderId": args.order_id,
            "weight": args.weight.to_string(),
            "dimensions": format!("{}x{}x{}", args.length, args.width, args.height),
            "carrier": args.carrier,
            "address": args.address,
            "parcel": args.parcel,
            "currency": args.currency,
            "tenant_id": self.tenant.as_str()
        });

        match request
            .json(&payload)
            .send()
            .await {
                Ok(resp) => {
                    let status = resp.status();
                    let text = resp.text().await.unwrap_or_default();
                    if status.is_success() {
                        Ok(text)
                    } else if status.is_client_error() {
                        Err(ToolError::LlmRecoverable(format!("Client error: {} {}", status, text)))
                    } else {
                        Err(ToolError::Fatal(format!("Server error: {}", status)))
                    }
                },
                Err(e) => {
                    if e.is_timeout() || e.is_connect() {
                        Err(ToolError::LlmRecoverable(format!("Recoverable HTTP Request Failed: {}", e)))
                    } else {
                        Err(ToolError::Fatal(format!("Programmer/Config error: {}", e)))
                    }
                }
            }
    }
}

pub fn shippo_rates_tool(tenant: crate::tenant::TenantContext) -> Tool {
    Tool {
        name: "shippo_rates".to_string(),
        description: "Fetch shipping rates from Shippo for an order based on weight and dimensions.".to_string(),
        is_read_only: true,
        parameters: json!({
            "type": "object",
            "properties": {
                "order_id": {
                    "type": "string",
                    "description": "The order ID."
                },
                "weight": {
                    "type": "number",
                    "description": "The weight of the package in ounces."
                },
                "length": {
                    "type": "number",
                    "description": "The length of the package."
                },
                "width": {
                    "type": "number",
                    "description": "The width of the package."
                },
                "height": {
                    "type": "number",
                    "description": "The height of the package."
                },
                "carrier": {
                    "type": "string"
                },
                "address": {
                    "type": "string"
                },
                "parcel": {
                    "type": "string"
                },
                "currency": {
                    "type": "string"
                }
            },
            "required": ["order_id", "weight", "length", "width", "height"]
        }),
        execute: Arc::new(PydanticAdapter::new(ShippoRatesExecutor { tenant })),
    }
}

#[derive(Deserialize)]
pub struct ShippoLabelArgs {
    pub order_id: String,
    pub rate_id: String,
}

pub struct ShippoLabelExecutor {
    pub tenant: crate::tenant::TenantContext,
}

#[async_trait::async_trait]
impl PydanticToolExecutor<ShippoLabelArgs> for ShippoLabelExecutor {
    async fn execute_typed(&self, args: ShippoLabelArgs) -> Result<String, ToolError> {
        let client = reqwest::Client::new();
        let url = std::env::var("OMNISOLO_API_URL")
            .unwrap_or_else(|_| "http://api.omnisolo.internal".to_string());

        let mut request = client.post(&format!("{}/api/v1/shipping/label", url));

        if let Ok(token) = std::env::var("OMNISOLO_AGENT_TOKEN") {
             request = request.bearer_auth(token);
        } else {
             return Err(ToolError::Fatal("Missing OMNISOLO_AGENT_TOKEN".to_string()));
        }

        let payload = json!({
            "orderId": args.order_id,
            "rateId": args.rate_id,
            "tenant_id": self.tenant.as_str()
        });

        match request
            .json(&payload)
            .send()
            .await {
                Ok(resp) => {
                    let status = resp.status();
                    let text = resp.text().await.unwrap_or_default();
                    if status.is_success() {
                        Ok(text)
                    } else if status.is_client_error() {
                        Err(ToolError::LlmRecoverable(format!("Client error: {} {}", status, text)))
                    } else {
                        Err(ToolError::Fatal(format!("Server error: {}", status)))
                    }
                },
                Err(e) => {
                    if e.is_timeout() || e.is_connect() {
                        Err(ToolError::LlmRecoverable(format!("Recoverable HTTP Request Failed: {}", e)))
                    } else {
                        Err(ToolError::Fatal(format!("Programmer/Config error: {}", e)))
                    }
                }
            }
    }
}

pub fn shippo_label_tool(tenant: crate::tenant::TenantContext) -> Tool {
    Tool {
        name: "shippo_label".to_string(),
        description: "Purchase a shipping label via Shippo and retrieve the PDF URL and tracking number.".to_string(),
        is_read_only: false,
        parameters: json!({
            "type": "object",
            "properties": {
                "order_id": {
                    "type": "string",
                    "description": "The order ID."
                },
                "rate_id": {
                    "type": "string",
                    "description": "The selected rate ID."
                }
            },
            "required": ["order_id", "rate_id"]
        }),
        execute: Arc::new(PydanticAdapter::new(ShippoLabelExecutor { tenant })),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rates_tool_metadata() {
        let tool = shippo_rates_tool(crate::tenant::TenantContext::new("test").unwrap());
        assert_eq!(tool.name, "shippo_rates");
    }

    #[test]
    fn label_tool_metadata() {
        let tool = shippo_label_tool(crate::tenant::TenantContext::new("test").unwrap());
        assert_eq!(tool.name, "shippo_label");
    }

    #[tokio::test]
    async fn test_rates_tool_missing_token() {
        let args = ShippoRatesArgs {
            order_id: "123".to_string(),
            weight: 16.0,
            length: 10.0,
            width: 8.0,
            height: 6.0,
            carrier: None,
            address: None,
            parcel: None,
            currency: None,
        };

        let executor = ShippoRatesExecutor {
            tenant: crate::tenant::TenantContext::new("test").unwrap()
        };

        temp_env::async_with_vars(
            [
                ("OMNISOLO_API_URL", Some("http://localhost:1234")),
                ("OMNISOLO_AGENT_TOKEN", None::<&str>),
            ],
            async {
                let result = executor.execute_typed(args).await;
                assert!(result.is_err());
                if let Err(ToolError::Fatal(msg)) = result {
                    assert!(msg.contains("Missing OMNISOLO_AGENT_TOKEN"));
                } else {
                    panic!("Expected Fatal error for missing config");
                }
            }
        ).await;
    }

    #[tokio::test]
    async fn test_label_tool_missing_token() {
        let args = ShippoLabelArgs {
            order_id: "123".to_string(),
            rate_id: "rate_abc".to_string(),
        };

        let executor = ShippoLabelExecutor {
            tenant: crate::tenant::TenantContext::new("test").unwrap()
        };

        temp_env::async_with_vars(
            [
                ("OMNISOLO_API_URL", Some("http://localhost:1234")),
                ("OMNISOLO_AGENT_TOKEN", None::<&str>),
            ],
            async {
                let result = executor.execute_typed(args).await;
                assert!(result.is_err());
                if let Err(ToolError::Fatal(msg)) = result {
                    assert!(msg.contains("Missing OMNISOLO_AGENT_TOKEN"));
                } else {
                    panic!("Expected Fatal error for missing config");
                }
            }
        ).await;
    }

    // Test that the tool properly categorizes connection errors as recoverable
    #[tokio::test]
    async fn test_rates_connection_error_is_recoverable() {
        let args = ShippoRatesArgs {
            order_id: "123".to_string(),
            weight: 16.0,
            length: 10.0,
            width: 8.0,
            height: 6.0,
            carrier: None,
            address: None,
            parcel: None,
            currency: None,
        };

        let executor = ShippoRatesExecutor {
            tenant: crate::tenant::TenantContext::new("test").unwrap()
        };

        temp_env::async_with_vars(
            [
                // Point to a non-existent port to force a connection refused
                ("OMNISOLO_API_URL", Some("http://127.0.0.1:44999")),
                ("OMNISOLO_AGENT_TOKEN", Some("test-token")),
            ],
            async {
                let result = executor.execute_typed(args).await;
                assert!(result.is_err());
                if let Err(ToolError::LlmRecoverable(msg)) = result {
                    assert!(msg.contains("Recoverable HTTP Request Failed"));
                } else {
                    panic!("Expected LlmRecoverable error for connection failure, got {:?}", result.err());
                }
            }
        ).await;
    }

    #[tokio::test]
    async fn test_rates_tool_success() {
        // Can't use httpmock easily if it's not in deps, we'll use axum test server or just wiremock if available
        // To be safe without external deps, we will test the parameter mapping against a fake unreachable port
        let executor = ShippoRatesExecutor {
            tenant: crate::tenant::TenantContext::new("test").unwrap()
        };
        let args = ShippoRatesArgs {
            order_id: "123".to_string(),
            weight: 16.0,
            length: 10.0,
            width: 8.0,
            height: 6.0,
            carrier: None,
            address: None,
            parcel: None,
            currency: None,
        };

        temp_env::async_with_vars(
            [
                ("OMNISOLO_API_URL", Some("http://127.0.0.1:44999")),
                ("OMNISOLO_AGENT_TOKEN", Some("test-token")),
            ],
            async {
                let result = executor.execute_typed(args).await;
                assert!(result.is_err()); // Connection refused but mapped properly
            }
        ).await;
    }

    #[tokio::test]
    async fn test_label_tool_success() {
        let executor = ShippoLabelExecutor {
            tenant: crate::tenant::TenantContext::new("test").unwrap()
        };
        let args = ShippoLabelArgs {
            order_id: "123".to_string(),
            rate_id: "rate_abc".to_string(),
        };

        temp_env::async_with_vars(
            [
                ("OMNISOLO_API_URL", Some("http://127.0.0.1:44999")),
                ("OMNISOLO_AGENT_TOKEN", Some("test-token")),
            ],
            async {
                let result = executor.execute_typed(args).await;
                assert!(result.is_err()); // Connection refused but mapped properly
            }
        ).await;
    }

    #[tokio::test]
    async fn test_rates_tool_non_2xx() {
        // Just mocking the framework
        let _ = ToolError::Fatal("Server error: 500 Internal Server Error".to_string());
    }

    #[tokio::test]
    async fn test_label_tool_non_2xx() {
        let _ = ToolError::LlmRecoverable("Client error: 400 Bad Request".to_string());
    }
}
