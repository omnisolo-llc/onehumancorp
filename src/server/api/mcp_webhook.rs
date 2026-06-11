use axum::{
    extract::{Json, State},
    http::{HeaderMap, StatusCode},
    response::IntoResponse,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx::Row;
use std::collections::BTreeSet;
use std::sync::Arc;
use crate::hub::Hub;
use uuid::Uuid;

#[derive(Debug, Deserialize)]
pub struct McpWebhookPayload {
    pub task_id: Uuid,
    pub status: String,
    pub result: Option<Value>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn configured_payments_and_logistics_patterns_resolve_to_route_metadata() {
        let config = WebhookTunnelConfig::from_spec(
            "payments:stripe,mercadopago;logistics:doordash,shippo",
        )
        .expect("config should parse");

        let stripe = config
            .resolve_path("/webhooks/payments/stripe/tunnel-pay-123")
            .expect("stripe payment tunnel should resolve");
        assert_eq!(stripe.class, WebhookTunnelClass::Payments);
        assert_eq!(stripe.service, "stripe");
        assert_eq!(stripe.tunnel_id.as_str(), "tunnel-pay-123");
        assert_eq!(stripe.upstream_path, "/api/billing/webhook/stripe");

        let shippo = config
            .resolve_path("/webhooks/logistics/shippo/tunnel-ship-456")
            .expect("shippo logistics tunnel should resolve");
        assert_eq!(shippo.class, WebhookTunnelClass::Logistics);
        assert_eq!(shippo.service, "shippo");
        assert_eq!(shippo.tunnel_id.as_str(), "tunnel-ship-456");
        assert_eq!(shippo.upstream_path, "/api/fulfillment/webhook/shippo");
    }

    #[test]
    fn unsupported_classes_and_unconfigured_services_are_rejected() {
        let config = WebhookTunnelConfig::from_spec("payments:stripe;logistics:shippo")
            .expect("config should parse");

        assert_eq!(
            config
                .resolve_path("/webhooks/crm/salesforce/tunnel-123")
                .expect_err("unsupported webhook class must be rejected"),
            WebhookTunnelError::UnsupportedClass("crm".to_string())
        );
        assert_eq!(
            config
                .resolve_path("/webhooks/payments/razorpay/tunnel-123")
                .expect_err("unconfigured payment service must be rejected"),
            WebhookTunnelError::UnsupportedService {
                class: WebhookTunnelClass::Payments,
                service: "razorpay".to_string(),
            }
        );
        assert_eq!(
            config
                .resolve_path("/webhooks/logistics/doordash/tunnel-123")
                .expect_err("unconfigured logistics service must be rejected"),
            WebhookTunnelError::UnsupportedService {
                class: WebhookTunnelClass::Logistics,
                service: "doordash".to_string(),
            }
        );
    }

    #[test]
    fn invalid_tunnel_ids_and_urls_are_rejected() {
        let config = WebhookTunnelConfig::from_spec("payments:stripe;logistics:shippo")
            .expect("config should parse");

        for path in [
            "/webhooks/payments/stripe/../secret",
            "/webhooks/payments/stripe/http://evil.example",
            "/webhooks/payments/stripe/tunnel id",
            "/webhooks/logistics/shippo/%2e%2e",
            "/webhooks/logistics/shippo/-starts-with-dash",
            "/webhooks/logistics/shippo/ends-with-dash-",
        ] {
            assert_eq!(
                config
                    .resolve_path(path)
                    .expect_err("invalid tunnel ID should be rejected"),
                WebhookTunnelError::InvalidTunnelId
            );
        }
    }

    #[test]
    fn malformed_config_specs_are_rejected() {
        assert_eq!(
            WebhookTunnelConfig::from_spec("payments:stripe;unknown:thing")
                .expect_err("unknown class in config should fail"),
            WebhookTunnelError::UnsupportedClass("unknown".to_string())
        );
        assert_eq!(
            WebhookTunnelConfig::from_spec("payments:http://evil.example")
                .expect_err("provider names are not URLs"),
            WebhookTunnelError::UnsupportedService {
                class: WebhookTunnelClass::Payments,
                service: "http://evil.example".to_string(),
            }
        );
    }
}

#[derive(Debug, Serialize)]
pub struct McpWebhookResponse {
    pub success: bool,
    pub message: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum WebhookTunnelClass {
    Payments,
    Logistics,
}

impl WebhookTunnelClass {
    fn parse(raw: &str) -> Result<Self, WebhookTunnelError> {
        match raw {
            "payments" => Ok(Self::Payments),
            "logistics" => Ok(Self::Logistics),
            other => Err(WebhookTunnelError::UnsupportedClass(other.to_string())),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WebhookTunnelError {
    InvalidPattern,
    InvalidTunnelId,
    UnsupportedClass(String),
    UnsupportedService {
        class: WebhookTunnelClass,
        service: String,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WebhookTunnelId(String);

impl WebhookTunnelId {
    fn parse(raw: &str) -> Result<Self, WebhookTunnelError> {
        let valid_len = (3..=64).contains(&raw.len());
        let chars_ok = raw
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || c == '-');
        let boundary_ok = raw
            .chars()
            .next()
            .zip(raw.chars().last())
            .map(|(first, last)| first.is_ascii_alphanumeric() && last.is_ascii_alphanumeric())
            .unwrap_or(false);

        if valid_len && chars_ok && boundary_ok {
            Ok(Self(raw.to_string()))
        } else {
            Err(WebhookTunnelError::InvalidTunnelId)
        }
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WebhookTunnelRoute {
    pub class: WebhookTunnelClass,
    pub service: String,
    pub tunnel_id: WebhookTunnelId,
    pub upstream_path: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WebhookTunnelConfig {
    payments: BTreeSet<String>,
    logistics: BTreeSet<String>,
}

impl WebhookTunnelConfig {
    pub fn from_env() -> Result<Self, WebhookTunnelError> {
        match std::env::var("MCP_WEBHOOK_TUNNEL_PATTERNS") {
            Ok(spec) => Self::from_spec(&spec),
            Err(_) => Self::default_patterns(),
        }
    }

    pub fn default_patterns() -> Result<Self, WebhookTunnelError> {
        Self::from_spec("payments:stripe,mercadopago,razorpay,alipay;logistics:shippo,easypost,doordash")
    }

    pub fn from_spec(spec: &str) -> Result<Self, WebhookTunnelError> {
        let mut config = Self {
            payments: BTreeSet::new(),
            logistics: BTreeSet::new(),
        };

        for class_spec in spec.split(';').map(str::trim).filter(|s| !s.is_empty()) {
            let (class_raw, services_raw) = class_spec
                .split_once(':')
                .ok_or(WebhookTunnelError::InvalidPattern)?;
            let class = WebhookTunnelClass::parse(class_raw.trim())?;

            for service in services_raw.split(',').map(str::trim).filter(|s| !s.is_empty()) {
                if upstream_path_for(class, service).is_none() {
                    return Err(WebhookTunnelError::UnsupportedService {
                        class,
                        service: service.to_string(),
                    });
                }

                match class {
                    WebhookTunnelClass::Payments => {
                        config.payments.insert(service.to_string());
                    }
                    WebhookTunnelClass::Logistics => {
                        config.logistics.insert(service.to_string());
                    }
                }
            }
        }

        Ok(config)
    }

    pub fn resolve_path(&self, path: &str) -> Result<WebhookTunnelRoute, WebhookTunnelError> {
        let rest = path
            .strip_prefix("/webhooks/")
            .ok_or(WebhookTunnelError::InvalidPattern)?;
        let mut parts = rest.splitn(3, '/');
        let class_raw = parts.next().ok_or(WebhookTunnelError::InvalidPattern)?;
        let service = parts.next().ok_or(WebhookTunnelError::InvalidPattern)?;
        let tunnel_id_raw = parts.next().ok_or(WebhookTunnelError::InvalidPattern)?;

        let class = WebhookTunnelClass::parse(class_raw)?;
        let allowed_services = match class {
            WebhookTunnelClass::Payments => &self.payments,
            WebhookTunnelClass::Logistics => &self.logistics,
        };
        if !allowed_services.contains(service) {
            return Err(WebhookTunnelError::UnsupportedService {
                class,
                service: service.to_string(),
            });
        }

        if tunnel_id_raw.contains('/')
            || tunnel_id_raw.contains(':')
            || tunnel_id_raw.contains('%')
            || tunnel_id_raw.contains('\\')
        {
            return Err(WebhookTunnelError::InvalidTunnelId);
        }

        let tunnel_id = WebhookTunnelId::parse(tunnel_id_raw)?;
        let upstream_path = upstream_path_for(class, service)
            .ok_or_else(|| WebhookTunnelError::UnsupportedService {
                class,
                service: service.to_string(),
            })?
            .to_string();

        Ok(WebhookTunnelRoute {
            class,
            service: service.to_string(),
            tunnel_id,
            upstream_path,
        })
    }
}

fn upstream_path_for(class: WebhookTunnelClass, service: &str) -> Option<&'static str> {
    match (class, service) {
        (WebhookTunnelClass::Payments, "stripe") => Some("/api/billing/webhook/stripe"),
        (WebhookTunnelClass::Payments, "mercadopago") => Some("/api/billing/webhook/mercadopago"),
        (WebhookTunnelClass::Payments, "razorpay") => Some("/api/billing/webhook/razorpay"),
        (WebhookTunnelClass::Payments, "alipay") => Some("/api/billing/webhook/alipay"),
        (WebhookTunnelClass::Logistics, "shippo") => Some("/api/fulfillment/webhook/shippo"),
        (WebhookTunnelClass::Logistics, "easypost") => Some("/api/fulfillment/webhook/easypost"),
        (WebhookTunnelClass::Logistics, "doordash") => Some("/api/fulfillment/webhook/doordash"),
        _ => None,
    }
}


use axum::extract::Path;
use crate::agents::mcp::proxy::server::ReverseTunnelServer;

use axum::body::Bytes;

pub async fn handle_relay_webhook(
    State(server): State<ReverseTunnelServer>,
    Path(agent_id): Path<String>,
    body: Bytes,
) -> impl IntoResponse {
    match server.forward_webhook(&agent_id, body.to_vec()).await {
        Ok(_) => (StatusCode::OK, Json(McpWebhookResponse {
            success: true,
            message: "Webhook forwarded successfully".to_string(),
        })),
        Err(e) => {
            tracing::error!("Failed to forward webhook to agent {}: {}", agent_id, e);
            (StatusCode::NOT_FOUND, Json(McpWebhookResponse {
                success: false,
                message: "Agent not connected or error forwarding".to_string(),
            }))
        }
    }
}

pub async fn handle_mcp_webhook(
    State(hub): State<Arc<Hub>>,
    headers: HeaderMap,
    Json(payload): Json<McpWebhookPayload>,
) -> impl IntoResponse {
    // Basic Bearer token verification.
    // In a real implementation, you would check against a specific integration token
    // or verify an HMAC signature of the payload.
    let expected_token = std::env::var("MCP_WEBHOOK_SECRET").unwrap_or_else(|_| "secret-token".to_string());

    let auth_header = headers.get("Authorization").and_then(|v| v.to_str().ok());

    if auth_header != Some(&format!("Bearer {}", expected_token)) {
        tracing::warn!("Unauthorized MCP webhook access attempt");
        return (
            StatusCode::UNAUTHORIZED,
            Json(McpWebhookResponse {
                success: false,
                message: "Unauthorized".to_string(),
            }),
        );
    }

    let t_id = payload.task_id;

    let task = sqlx::query(
        "SELECT organization_id, assigned_agent_id FROM shared_tasks WHERE id = $1",
    )
    .bind(t_id.to_string())
    .fetch_optional(&hub.pool)
    .await;

    match task {
        Ok(Some(task)) => {
            let result_payload = payload.result.unwrap_or(Value::Null);
            if let Err(e) = sqlx::query(
                "UPDATE shared_tasks SET status = $1, payload = $2, updated_at = NOW() WHERE id = $3",
            )
            .bind(&payload.status)
            .bind(result_payload)
            .bind(t_id.to_string())
            .execute(&hub.pool)
            .await
            {
                ::server_telemetry::record_error_signal("Failed to update MCP task ");
                tracing::error!("Failed to update MCP task {}: {}", t_id, e);
                return (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(McpWebhookResponse {
                        success: false,
                        message: "Failed to update task status".to_string(),
                    }),
                );
            }

            // In a real KAIROS implementation, this would trigger agent resumption via the orchestrator.
            // For now, we simulate reactivating the agent.
            let tnt_id: Option<String> = task.try_get("organization_id").ok();
            let agent_id: Option<String> = task.try_get("assigned_agent_id").ok();
            tracing::info!(
                "KAIROS Hook: Reactivating agent {:?} for org {:?} (Task {})",
                agent_id,
                tnt_id,
                t_id
            );

            (
                StatusCode::OK,
                Json(McpWebhookResponse {
                    success: true,
                    message: "Task updated and agent reactivated".to_string(),
                }),
            )
        }
        Ok(None) => {
            (
                StatusCode::NOT_FOUND,
                Json(McpWebhookResponse {
                    success: false,
                    message: "Task not found".to_string(),
                }),
            )
        }
        Err(e) => {
            ::server_telemetry::record_error_signal("Database error fetching MCP task ");
            tracing::error!("Database error fetching MCP task {}: {}", t_id, e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(McpWebhookResponse {
                    success: false,
                    message: "Database error".to_string(),
                }),
            )
        }
    }
}
