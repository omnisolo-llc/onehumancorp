use super::client::{LocalFSSyncTool, LocalProxyClient};
use super::server::ReverseTunnelServer;
use ::server_ohc::mcp_proxy::mcp_reverse_tunnel_service_client::McpReverseTunnelServiceClient;
use ::server_ohc::mcp_proxy::mcp_reverse_tunnel_service_server::{
    McpReverseTunnelService, McpReverseTunnelServiceServer,
};
use std::fs;
use std::sync::Arc;
use tempfile::tempdir;
use tokio::net::TcpListener;
use tokio_stream::wrappers::TcpListenerStream;
use tonic::transport::{Endpoint, Server};

#[tokio::test]
async fn test_proxy_tunnel() {
    let pool = sqlx::postgres::PgPoolOptions::new()
        .after_release(|conn, _meta| {
            Box::pin(async move {
                use sqlx::Executor;
                conn.execute("DISCARD ALL").await?;
                Ok(true)
            })
        })
        .after_release(|conn, _meta| {
            Box::pin(async move {
                use sqlx::Executor;
                conn.execute("DISCARD ALL").await?;
                Ok(true)
            })
        })
        .max_connections(1)
        .acquire_timeout(std::time::Duration::from_millis(1))
        .connect_lazy("postgres://invalid:invalid@localhost:1/test")
        .unwrap();

    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();

    let server = ReverseTunnelServer {
        pool: Arc::new(pool),
    };

    tokio::spawn(async move {
        let _ = Server::builder()
            .add_service(McpReverseTunnelServiceServer::new(server))
            .serve_with_incoming(TcpListenerStream::new(listener))
            .await;
    });

    tokio::time::sleep(std::time::Duration::from_millis(50)).await;

    let endpoint = Endpoint::from_shared(format!("http://{}", addr)).unwrap();
    let channel = endpoint.connect().await.unwrap();
    let grpc_client = McpReverseTunnelServiceClient::new(channel);

    let mut client = LocalProxyClient::new_with_channel(grpc_client, "spiffe://test".to_string());

    let _ = client.start().await;

    tokio::time::sleep(std::time::Duration::from_millis(100)).await;
}

#[tokio::test]
async fn test_local_fs_sync_tool_read() {
    let dir = tempdir().unwrap();
    let base_path = dir.path().to_str().unwrap().to_string();
    let tool = LocalFSSyncTool::new(base_path.clone());

    let task_dir = dir.path().join(".agent-task");
    fs::create_dir_all(&task_dir).unwrap();
    fs::write(task_dir.join("test.txt"), "hello read").unwrap();

    let params = serde_json::json!({
        "action": "read",
        "path": ".agent-task/test.txt"
    });

    let res = tool.execute(&params.to_string()).await.unwrap();
    assert_eq!(res.0, true);
    assert_eq!(res.1, "hello read");
}

#[tokio::test]
async fn test_local_fs_sync_tool_write() {
    let dir = tempdir().unwrap();
    let base_path = dir.path().to_str().unwrap().to_string();
    let tool = LocalFSSyncTool::new(base_path.clone());

    let params = serde_json::json!({
        "action": "write",
        "path": ".agent-task/test2.txt",
        "content": "hello write"
    });

    let res = tool.execute(&params.to_string()).await.unwrap();
    assert_eq!(res.0, true);

    let content = fs::read_to_string(dir.path().join(".agent-task/test2.txt")).unwrap();
    assert_eq!(content, "hello write");
}

#[tokio::test]
async fn test_local_fs_sync_tool_sync() {
    let dir = tempdir().unwrap();
    let base_path = dir.path().to_str().unwrap().to_string();
    let tool = LocalFSSyncTool::new(base_path.clone());

    let task_dir = dir.path().join(".agent-task");
    fs::create_dir_all(&task_dir).unwrap();
    fs::write(task_dir.join("test3.txt"), "hello sync").unwrap();

    let params = serde_json::json!({
        "action": "sync",
        "path": ".agent-task/test3.txt"
    });

    let res = tool.execute(&params.to_string()).await.unwrap();
    assert_eq!(res.0, true);
    assert_eq!(res.1, "Synced");

    let params_missing = serde_json::json!({
        "action": "sync",
        "path": ".agent-task/missing.txt"
    });
    let res_missing = tool.execute(&params_missing.to_string()).await.unwrap();
    assert_eq!(res_missing.0, false);
}

#[tokio::test]
async fn test_local_fs_sync_tool_validation() {
    let dir = tempdir().unwrap();
    let base_path = dir.path().to_str().unwrap().to_string();
    let tool = LocalFSSyncTool::new(base_path.clone());

    // Path must start with .agent-task/
    let params = serde_json::json!({
        "action": "read",
        "path": "test.txt"
    });
    let res = tool.execute(&params.to_string()).await.unwrap();
    assert_eq!(res.0, false);
    assert!(res.2.contains("must start with .agent-task/"));

    // Path traversal attempt
    let params2 = serde_json::json!({
        "action": "read",
        "path": ".agent-task/../test.txt"
    });
    let res2 = tool.execute(&params2.to_string()).await.unwrap();
    assert_eq!(res2.0, false);
    assert!(res2.2.contains("traversal attempt"));

    // Invalid json params
    let res3 = tool.execute("invalid json").await.unwrap();
    assert_eq!(res3.0, false);
    assert!(res3.2.contains("Invalid params"));

    // Invalid action
    let params4 = serde_json::json!({
        "action": "delete",
        "path": ".agent-task/test.txt"
    });
    let res4 = tool.execute(&params4.to_string()).await.unwrap();
    assert_eq!(res4.0, false);
    assert!(res4.2.contains("Invalid action"));
}
