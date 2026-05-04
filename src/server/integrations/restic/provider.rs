use crate::integrations::catalog::{IntegrationProvider, ProviderMetadata};
use std::env;
use tokio::process::Command;
use crate::ohc::orchestration::{McpInvokeRequest, McpInvokeResponse, McpToolProto};

pub struct ResticProvider {
    pub metadata: ProviderMetadata,
}

impl ResticProvider {
    pub fn new() -> Self {
        Self {
            metadata: ProviderMetadata {
                id: "restic".to_string(),
                name: "Restic Local Snapshot".to_string(),
                category: "backup".to_string(),
                base_url: "local://".to_string(),
            },
        }
    }

    pub fn into_integration_provider(self) -> IntegrationProvider {
        IntegrationProvider {
            metadata: self.metadata,
        }
    }

    fn check_mode() -> Result<(), String> {
        let is_standalone = env::var("OHC_STANDALONE_MODE").unwrap_or_else(|_| "false".to_string()) == "true";
        if !is_standalone {
            return Err("Restic integration is unsupported in Cloud mode".to_string());
        }
        Ok(())
    }

    pub async fn snapshot(path: &str) -> Result<String, String> {
        Self::check_mode()?;

        if path.starts_with('-') {
            return Err("Invalid path argument".to_string());
        }

        let output = Command::new("restic")
            .arg("backup")
            .arg(path)
            .output()
            .await
            .map_err(|e| format!("Failed to execute restic: {}", e))?;

        if !output.status.success() {
            return Err(String::from_utf8_lossy(&output.stderr).to_string());
        }

        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    }

    pub async fn restore(snapshot_id: &str, target_path: &str) -> Result<String, String> {
        Self::check_mode()?;

        if snapshot_id.starts_with('-') || target_path.starts_with('-') {
            return Err("Invalid argument".to_string());
        }

        let output = Command::new("restic")
            .arg("restore")
            .arg(snapshot_id)
            .arg("--target")
            .arg(target_path)
            .output()
            .await
            .map_err(|e| format!("Failed to execute restic: {}", e))?;

        if !output.status.success() {
            return Err(String::from_utf8_lossy(&output.stderr).to_string());
        }

        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    }

    pub async fn status() -> Result<String, String> {
        Self::check_mode()?;

        let output = Command::new("restic")
            .arg("snapshots")
            .output()
            .await
            .map_err(|e| format!("Failed to execute restic: {}", e))?;

        if !output.status.success() {
            return Err(String::from_utf8_lossy(&output.stderr).to_string());
        }

        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    }

    // Explicitly exposing MCP tools
    pub fn get_tools(&self) -> Vec<McpToolProto> {
        vec![
            McpToolProto {
                id: "restic_snapshot".to_string(),
                name: "Restic Snapshot".to_string(),
                description: "Perform a Restic snapshot. Input schema: {\"type\":\"object\",\"properties\":{\"path\":{\"type\":\"string\"}},\"required\":[\"path\"]}".to_string(),
                category: "backup".to_string(),
                status: "active".to_string(),
            },
            McpToolProto {
                id: "restic_restore".to_string(),
                name: "Restic Restore".to_string(),
                description: "Perform a Restic restore. Input schema: {\"type\":\"object\",\"properties\":{\"snapshot_id\":{\"type\":\"string\"},\"target_path\":{\"type\":\"string\"}},\"required\":[\"snapshot_id\",\"target_path\"]}".to_string(),
                category: "backup".to_string(),
                status: "active".to_string(),
            },
            McpToolProto {
                id: "restic_status".to_string(),
                name: "Restic Status".to_string(),
                description: "Check Restic status. Input schema: {\"type\":\"object\",\"properties\":{}}".to_string(),
                category: "backup".to_string(),
                status: "active".to_string(),
            },
        ]
    }

    pub async fn invoke_tool(&self, req: &McpInvokeRequest) -> Result<McpInvokeResponse, tonic::Status> {
        let params: serde_json::Value = serde_json::from_str(&req.params)
            .map_err(|e| tonic::Status::invalid_argument(format!("invalid JSON params: {}", e)))?;

        match req.tool_id.as_str() {
            "restic_snapshot" => {
                let path = params["path"].as_str().unwrap_or("");
                match Self::snapshot(path).await {
                    Ok(output) => {
                        let resp = serde_json::json!({"output": output});
                        Ok(McpInvokeResponse { payload: serde_json::to_string(&resp).unwrap() })
                    }
                    Err(e) => Err(tonic::Status::internal(format!("failed to run snapshot: {}", e))),
                }
            }
            "restic_restore" => {
                let snapshot_id = params["snapshot_id"].as_str().unwrap_or("");
                let target_path = params["target_path"].as_str().unwrap_or("");

                match Self::restore(snapshot_id, target_path).await {
                    Ok(output) => {
                        let resp = serde_json::json!({"output": output});
                        Ok(McpInvokeResponse { payload: serde_json::to_string(&resp).unwrap() })
                    }
                    Err(e) => Err(tonic::Status::internal(format!("failed to run restore: {}", e))),
                }
            }
            "restic_status" => {
                match Self::status().await {
                    Ok(output) => {
                        let resp = serde_json::json!({"output": output});
                        Ok(McpInvokeResponse { payload: serde_json::to_string(&resp).unwrap() })
                    }
                    Err(e) => Err(tonic::Status::internal(format!("failed to run status: {}", e))),
                }
            }
            _ => Err(tonic::Status::not_found("tool not found")),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_restic_provider_metadata() {
        let provider = ResticProvider::new();
        assert_eq!(provider.metadata.id, "restic");
        assert_eq!(provider.metadata.name, "Restic Local Snapshot");
        assert_eq!(provider.metadata.category, "backup");
        assert_eq!(provider.metadata.base_url, "local://");
    }

    #[tokio::test]
    async fn test_restic_cloud_mode() {
        // Without safe env injection, simply verify default returns err since standalone defaults false.
        let result = ResticProvider::snapshot("test").await;
        assert!(result.is_err());
    }

    #[test]
    fn test_get_tools() {
        let provider = ResticProvider::new();
        let tools = provider.get_tools();
        assert_eq!(tools.len(), 3);
        assert_eq!(tools[0].id, "restic_snapshot");
        assert_eq!(tools[1].id, "restic_restore");
        assert_eq!(tools[2].id, "restic_status");
    }

    #[tokio::test]
    async fn test_invoke_tool_cloud_mode() {
        let provider = ResticProvider::new();

        let req = McpInvokeRequest {
            tool_id: "restic_status".to_string(),
            params: "{}".to_string(),
            action: "".to_string(),
            agent_id: "".to_string(),
            spiffe_id: "".to_string()
        };

        let res = provider.invoke_tool(&req).await;
        assert!(res.is_err());
        assert!(res.unwrap_err().message().contains("unsupported in Cloud mode"));
    }
}
