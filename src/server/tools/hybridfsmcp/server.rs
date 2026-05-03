use std::sync::Arc;
use crate::ohc::orchestration::{McpInvokeRequest, McpInvokeResponse, McpToolProto};
use super::provider::BlobProvider;

pub struct HybridFSMcpServer {
    provider: Arc<dyn BlobProvider>,
}

impl HybridFSMcpServer {
    pub fn new(provider: Arc<dyn BlobProvider>) -> Self {
        Self { provider }
    }

    pub fn get_tools(&self) -> Vec<McpToolProto> {
        vec![
            McpToolProto {
                id: "fs_read_file".to_string(),
                name: "Read File".to_string(),
                description: "Read a file from the file system. Input schema: {\"type\":\"object\",\"properties\":{\"path\":{\"type\":\"string\"}}}".to_string(),
                category: "filesystem".to_string(),
                status: "active".to_string(),
            },
            McpToolProto {
                id: "fs_write_file".to_string(),
                name: "Write File".to_string(),
                description: "Write content to a file. Input schema: {\"type\":\"object\",\"properties\":{\"path\":{\"type\":\"string\"},\"content\":{\"type\":\"string\"}}}".to_string(),
                category: "filesystem".to_string(),
                status: "active".to_string(),
            },
            McpToolProto {
                id: "fs_list_dir".to_string(),
                name: "List Directory".to_string(),
                description: "List contents of a directory. Input schema: {\"type\":\"object\",\"properties\":{\"path\":{\"type\":\"string\"}}}".to_string(),
                category: "filesystem".to_string(),
                status: "active".to_string(),
            },
        ]
    }

    pub async fn invoke_tool(&self, req: &McpInvokeRequest) -> Result<McpInvokeResponse, tonic::Status> {
        let params: serde_json::Value = serde_json::from_str(&req.params)
            .map_err(|e| tonic::Status::invalid_argument(format!("invalid JSON params: {}", e)))?;

        match req.tool_id.as_str() {
            "fs_read_file" => {
                let path = params["path"].as_str().ok_or_else(|| tonic::Status::invalid_argument("path is required"))?;
                match self.provider.read_file(path).await {
                    Ok(content) => {
                        let content_str = String::from_utf8_lossy(&content).to_string();
                        let resp = serde_json::json!({"content": content_str});
                        Ok(McpInvokeResponse { payload: serde_json::to_string(&resp).unwrap() })
                    }
                    Err(e) => Err(tonic::Status::internal(format!("failed to read file: {}", e))),
                }
            }
            "fs_write_file" => {
                let path = params["path"].as_str().ok_or_else(|| tonic::Status::invalid_argument("path is required"))?;
                let content = params["content"].as_str().ok_or_else(|| tonic::Status::invalid_argument("content is required"))?;

                match self.provider.write_file(path, content.as_bytes()).await {
                    Ok(_) => {
                        let resp = serde_json::json!({"status": "success"});
                        Ok(McpInvokeResponse { payload: serde_json::to_string(&resp).unwrap() })
                    }
                    Err(e) => Err(tonic::Status::internal(format!("failed to write file: {}", e))),
                }
            }
            "fs_list_dir" => {
                let path = params["path"].as_str().ok_or_else(|| tonic::Status::invalid_argument("path is required"))?;

                match self.provider.list_dir(path).await {
                    Ok(entries) => {
                        let resp = serde_json::json!({"entries": entries});
                        Ok(McpInvokeResponse { payload: serde_json::to_string(&resp).unwrap() })
                    }
                    Err(e) => Err(tonic::Status::internal(format!("failed to list dir: {}", e))),
                }
            }
            _ => Err(tonic::Status::unimplemented(format!("tool {} not implemented", req.tool_id))),
        }
    }
}
