use super::provider::{LocalFSProvider, CloudFSProvider, FileSystemProvider};
use super::server::HybridFSMcpServer;
use std::sync::Arc;
use tempfile::tempdir;
use crate::ohc::orchestration::McpInvokeRequest;

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

#[tokio::test]
async fn test_local_fs_provider_path_traversal() {
    let dir = tempdir().unwrap();
    let provider = LocalFSProvider::new(dir.path().to_path_buf());

    // Should fail with permission denied on traversal
    let res = provider.read_file("../test.txt").await;
    assert!(res.is_err());
    assert_eq!(res.unwrap_err().kind(), std::io::ErrorKind::PermissionDenied);

    let res2 = provider.read_file("/etc/passwd").await;
    assert!(res2.is_err());
    assert_eq!(res2.unwrap_err().kind(), std::io::ErrorKind::PermissionDenied);

    let res3 = provider.write_file("../test2.txt", b"hack").await;
    assert!(res3.is_err());
    assert_eq!(res3.unwrap_err().kind(), std::io::ErrorKind::PermissionDenied);
}

#[tokio::test]
async fn test_cloud_fs_provider_path_traversal() {
    let dir = tempdir().unwrap();
    let tenant_id = "tenant-123".to_string();
    let provider = CloudFSProvider::new(tenant_id.clone(), dir.path().to_path_buf());

    // Should fail with permission denied on traversal
    let res = provider.read_file("../test.txt").await;
    assert!(res.is_err());
    assert_eq!(res.unwrap_err().kind(), std::io::ErrorKind::PermissionDenied);

    let res2 = provider.read_file("/etc/passwd").await;
    assert!(res2.is_err());
    assert_eq!(res2.unwrap_err().kind(), std::io::ErrorKind::PermissionDenied);

    let res3 = provider.write_file("../test2.txt", b"hack").await;
    assert!(res3.is_err());
    assert_eq!(res3.unwrap_err().kind(), std::io::ErrorKind::PermissionDenied);
}
