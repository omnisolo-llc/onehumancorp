use super::{Tool, ToolExecutor};
use ohc_builtin_agent_core::types::ToolError;
use serde_json::{json, Value};
use std::sync::Arc;
use ledger_proto::ohc::ledger::{GetBalanceRequest, GetStatementRequest, ledger_service_client::LedgerServiceClient};

pub struct GetBalanceExecutor;

#[async_trait::async_trait]
impl ToolExecutor for GetBalanceExecutor {
    async fn execute(
        &self,
        args: Value,
    ) -> Result<String, ToolError> {
        let tenant_id = std::env::var("OHC_TENANT_ID").unwrap_or_else(|_| "test_tenant".to_string());
        let account_id = args["account_id"].as_str().unwrap_or("main");

        let mut client = LedgerServiceClient::connect("http://[::1]:50051")
            .await
            .map_err(|e| ToolError::Fatal(format!("Failed to connect to Ledger Service: {}", e)))?;

        let req = tonic::Request::new(GetBalanceRequest {
            tenant_id: tenant_id.clone(),
            account_id: account_id.to_string(),
        });

        match client.get_balance(req).await {
            Ok(resp) => {
                let r = resp.into_inner();
                Ok(json!({
                    "status": "success",
                    "balance": r.balance,
                    "currency": r.currency
                }).to_string())
            }
            Err(_) => Ok(json!({
                "status": "error",
                "message": "Account not found"
            }).to_string()),
        }
    }
}

pub fn get_balance_tool() -> Tool {
    Tool {
        name: "get_ledger_balance".to_string(),
        description: "Get the current balance of a ledger account.".to_string(),
        is_read_only: true,
        parameters: json!({
            "type": "object",
            "properties": {
                "account_id": {
                    "type": "string",
                    "description": "The account ID."
                }
            },
            "required": ["account_id"]
        }),
        execute: Arc::new(GetBalanceExecutor),
    }
}

pub struct GetStatementExecutor;

#[async_trait::async_trait]
impl ToolExecutor for GetStatementExecutor {
    async fn execute(
        &self,
        args: Value,
    ) -> Result<String, ToolError> {
        let tenant_id = std::env::var("OHC_TENANT_ID").unwrap_or_else(|_| "test_tenant".to_string());
        let account_id = args["account_id"].as_str().unwrap_or("main");

        let mut client = LedgerServiceClient::connect("http://[::1]:50051")
            .await
            .map_err(|e| ToolError::Fatal(format!("Failed to connect to Ledger Service: {}", e)))?;

        let req = tonic::Request::new(GetStatementRequest {
            tenant_id: tenant_id.clone(),
            account_id: account_id.to_string(),
        });

        match client.get_statement(req).await {
            Ok(resp) => {
                let r = resp.into_inner();
                let mut txs = Vec::new();
                for t in r.transactions {
                    txs.push(json!({
                        "tx_id": t.tx_id,
                        "amount": t.amount,
                        "currency": t.currency,
                        "timestamp": t.timestamp
                    }));
                }
                Ok(json!({
                    "status": "success",
                    "transactions": txs
                }).to_string())
            }
            Err(e) => Err(ToolError::Fatal(format!("Query failed: {}", e))),
        }
    }
}

pub fn get_statement_tool() -> Tool {
    Tool {
        name: "get_ledger_statement".to_string(),
        description: "Get a statement of transactions for a ledger account.".to_string(),
        is_read_only: true,
        parameters: json!({
            "type": "object",
            "properties": {
                "account_id": {
                    "type": "string",
                    "description": "The account ID."
                }
            },
            "required": ["account_id"]
        }),
        execute: Arc::new(GetStatementExecutor),
    }
}
