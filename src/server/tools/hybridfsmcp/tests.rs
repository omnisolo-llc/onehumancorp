use super::provider::{LocalFSProvider, CloudFSProvider, FileSystemProvider};
use super::server::HybridFSMcpServer;
use std::sync::Arc;
use tempfile::tempdir;
use ::server_ohc::orchestration::McpInvokeRequest;

#[tokio::test]
async fn test_local_fs_provider() {
    let dir = tempdir().unwrap();
    let provider = LocalFSProvider::new(dir.path().to_path_buf());

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
async fn test_cloud_fs_provider() {
    let dir = tempdir().unwrap();
    let tenant_id = "tenant-123".to_string();
    let provider = CloudFSProvider::new(tenant_id.clone(), dir.path().to_path_buf());

    // Test write and read
    provider.write_file("test.txt", b"cloud content").await.unwrap();
    let content = provider.read_file("test.txt").await.unwrap();
    assert_eq!(content, b"cloud content");

    // Verify it was written to tenant dir
    let tenant_file = dir.path().join(&tenant_id).join("test.txt");
    assert!(tenant_file.exists());

    // Test list dir
    provider.write_file("docs/doc1.md", b"doc").await.unwrap();
    let entries = provider.list_dir("docs").await.unwrap();
    assert_eq!(entries, vec!["doc1.md"]);
}

#[tokio::test]
async fn test_hybrid_fs_mcp_server() {
    let dir = tempdir().unwrap();
    let provider = Arc::new(LocalFSProvider::new(dir.path().to_path_buf()));
    let server = HybridFSMcpServer::new(provider);

    let tools = server.get_tools();
    assert_eq!(tools.len(), 5);

    // Test fs_write_file tool
    let req = McpInvokeRequest {
        tool_id: "fs_hybrid_write".to_string(),
        action: "invoke".to_string(),
        params: r#"{"path":"server_test.txt","content":"from server"}"#.to_string(),
        agent_id: "agent-1".to_string(),
        spiffe_id: "spiffe-1".to_string(),
    };
    let resp = server.invoke_tool(&req, None).await.unwrap();
    let payload: serde_json::Value = serde_json::from_str(&resp.payload).unwrap();
    assert_eq!(payload["status"], "success");

    // Test fs_read_file tool
    let req = McpInvokeRequest {
        tool_id: "fs_hybrid_read".to_string(),
        action: "invoke".to_string(),
        params: r#"{"path":"server_test.txt"}"#.to_string(),
        agent_id: "agent-1".to_string(),
        spiffe_id: "spiffe-1".to_string(),
    };
    let resp = server.invoke_tool(&req, None).await.unwrap();
    let payload: serde_json::Value = serde_json::from_str(&resp.payload).unwrap();
    assert_eq!(payload["content"], "from server");
}

#[tokio::test]
async fn test_server_search() {
    let dir = tempdir().unwrap();
    let provider = Arc::new(LocalFSProvider::new(dir.path().to_path_buf()));
    let server = HybridFSMcpServer::new(provider);

    let req = McpInvokeRequest {
        tool_id: "fs_hybrid_write".to_string(),
        action: "invoke".to_string(),
        params: r#"{"path":"server_test.txt","content":"from server"}"#.to_string(),
        agent_id: "agent-1".to_string(),
        spiffe_id: "spiffe-1".to_string(),
    };
    server.invoke_tool(&req, None).await.unwrap();

    let req2 = McpInvokeRequest {
        tool_id: "fs_search_files".to_string(),
        action: "invoke".to_string(),
        params: r#"{"path":".","query":".txt"}"#.to_string(),
        agent_id: "agent-1".to_string(),
        spiffe_id: "spiffe-1".to_string(),
    };
    let res = server.invoke_tool(&req2, None).await.unwrap();
    assert!(res.payload.contains("server_test.txt"));
}

#[tokio::test]
async fn test_local_fs_provider_search() {
    let dir = tempdir().unwrap();
    let provider = LocalFSProvider::new(dir.path().to_path_buf());

    tokio::fs::create_dir_all(dir.path().join("dir")).await.unwrap();

    provider.write_file("dir/file1.txt", b"hello").await.unwrap();
    provider.write_file("dir/file2.md", b"world").await.unwrap();

    let entries = provider.search_files("dir", ".md").await.unwrap();
    assert_eq!(entries, vec!["file2.md"]);
}

#[tokio::test]
async fn test_provider_path_traversal() {
    let dir = tempdir().unwrap();
    let provider = LocalFSProvider::new(dir.path().to_path_buf());

    // Write a valid file
    provider.write_file("valid.txt", b"valid").await.unwrap();

    // Attempt directory traversal out of bounds
    let err = provider.write_file("../out_of_bounds.txt", b"invalid").await.unwrap_err();
    assert_eq!(err.kind(), std::io::ErrorKind::PermissionDenied);

    // Attempt directory traversal read
    let err = provider.read_file("../valid.txt").await.unwrap_err();
    assert_eq!(err.kind(), std::io::ErrorKind::PermissionDenied);

    // Attempt to write an absolute path that is stripped and becomes in bounds
    provider.write_file("/in_bounds.txt", b"absolute").await.unwrap();
    let content = provider.read_file("in_bounds.txt").await.unwrap();
    assert_eq!(content, b"absolute");

    // Check that an absolute path out of bounds doesn't traverse
    let err = provider.read_file("/../etc/passwd").await.unwrap_err();
    assert_eq!(err.kind(), std::io::ErrorKind::PermissionDenied);
}
