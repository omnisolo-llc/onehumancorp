use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ConnectionToken {
    pub object: String,
    pub location: String,
    pub secret: String,
}

pub struct StripeTerminalClient {
    pub api_key: String,
}

impl StripeTerminalClient {
    pub fn new(api_key: String) -> Self {
        StripeTerminalClient { api_key }
    }

    pub async fn create_connection_token(&self, tenant_id: &str) -> Result<ConnectionToken, String> {
        // Record mock API telemetry cost for orchestration monitoring
        let _ = ::server_telemetry::record_api_call_cost(
            &crate::db::get_pool(),
            tenant_id,
            "stripe_terminal_connection_token",
            0.05 // Mock cost
        ).await;

        Ok(ConnectionToken {
            object: "terminal.connection_token".to_string(),
            location: "tml_12345".to_string(),
            secret: format!("tct_secret_mock_{}_{}", tenant_id, chrono::Utc::now().timestamp()),
        })
    }
}
