use super::provider::{LocalBlobProvider, S3BlobProvider, BlobProvider};
use super::server::HybridFSMcpServer;
use std::sync::Arc;
use tempfile::tempdir;
use crate::ohc::orchestration::McpInvokeRequest;

#[tokio::test]
async fn test_local_blob_provider() {
    let dir = tempdir().unwrap();
    let provider = LocalBlobProvider::new(dir.path().to_path_buf());

    // Test write and read
    provider.write_file("test.txt", b"hello world").await.unwrap();
    let content = provider.read_file("test.txt").await.unwrap();
    assert_eq!(content, b"hello world");

    // Test list dir
    provider.write_file("dir/file1.txt", b"1").await.unwrap();
    provider.write_file("dir/file2.txt", b"2").await.unwrap();

    let mut entries = provider.list_dir("dir").await.unwrap();
    entries.sort();
    assert_eq!(entries, vec!["file1.txt", "file2.txt"]);
}

#[tokio::test]
async fn test_s3_blob_provider() {
    let tenant_id = "tenant-123".to_string();
    let bucket_name = "test-bucket".to_string();
    let provider = S3BlobProvider::new(tenant_id.clone(), bucket_name);

    // The underlying S3Provider is a stub. `write_blob` succeeds doing nothing,
    // and `read_blob` returns an empty vec.
    provider.write_file("test.txt", b"cloud content").await.unwrap();
    let content = provider.read_file("test.txt").await.unwrap();
    assert_eq!(content, b""); // It's a stub so it returns empty

    // List dir is also a stub returning an empty vec.
    let entries = provider.list_dir("docs").await.unwrap();
    assert!(entries.is_empty());
}

#[tokio::test]
async fn test_hybrid_fs_mcp_server() {
    let dir = tempdir().unwrap();
    let provider = Arc::new(LocalBlobProvider::new(dir.path().to_path_buf()));
    let server = HybridFSMcpServer::new(provider);

    let tools = server.get_tools();
    assert_eq!(tools.len(), 3);

    // Test fs_write_file tool
    let req = McpInvokeRequest {
        tool_id: "fs_write_file".to_string(),
        action: "invoke".to_string(),
        params: r#"{"path":"server_test.txt","content":"from server"}"#.to_string(),
        agent_id: "agent-1".to_string(),
        spiffe_id: "spiffe-1".to_string(),
    };
    let resp = server.invoke_tool(&req).await.unwrap();
    let payload: serde_json::Value = serde_json::from_str(&resp.payload).unwrap();
    assert_eq!(payload["status"], "success");

    // Test fs_read_file tool
    let req = McpInvokeRequest {
        tool_id: "fs_read_file".to_string(),
        action: "invoke".to_string(),
        params: r#"{"path":"server_test.txt"}"#.to_string(),
        agent_id: "agent-1".to_string(),
        spiffe_id: "spiffe-1".to_string(),
    };
    let resp = server.invoke_tool(&req).await.unwrap();
    let payload: serde_json::Value = serde_json::from_str(&resp.payload).unwrap();
    assert_eq!(payload["content"], "from server");
}
