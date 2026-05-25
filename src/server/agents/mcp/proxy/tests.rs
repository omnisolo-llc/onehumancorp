use super::server::ReverseTunnelServer;
use super::client::LocalProxyClient;
use ::server_ohc::mcp_proxy::mcp_reverse_tunnel_service_server::{McpReverseTunnelService, McpReverseTunnelServiceServer};
use ::server_ohc::mcp_proxy::mcp_reverse_tunnel_service_client::McpReverseTunnelServiceClient;
use tonic::transport::{Server, Endpoint};
use std::sync::Arc;
use tokio::net::TcpListener;
use tokio_stream::wrappers::TcpListenerStream;

#[tokio::test]
async fn test_proxy_tunnel() {
    let pool = sqlx::postgres::PgPoolOptions::new().after_release(|conn, _meta| { Box::pin(async move { use sqlx::Executor; conn.execute("DISCARD ALL").await?; Ok(true) }) }).after_release(|conn, _meta| { Box::pin(async move { use sqlx::Executor; conn.execute("DISCARD ALL").await?; Ok(true) }) })
        .max_connections(1)
        .acquire_timeout(std::time::Duration::from_millis(1))
        .connect_lazy("postgres://invalid:invalid@localhost:1/test")
        .unwrap();

    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();

    let server = ReverseTunnelServer { pool: Arc::new(pool) };

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
async fn test_hybrid_context_tool() {
    use super::client::HybridContextTool;

    let pool = sqlx::postgres::PgPoolOptions::new()
        .after_release(|conn, _meta| { Box::pin(async move { use sqlx::Executor; conn.execute("DISCARD ALL").await?; Ok(true) }) })
        .max_connections(1)
        .acquire_timeout(std::time::Duration::from_millis(1))
        .connect_lazy("postgres://invalid:invalid@localhost:1/test")
        .unwrap();

    let tool = HybridContextTool::new(pool);
    let (success, _, err) = tool.execute("{\"some\":\"context\"}").await;

    // It should fail due to invalid DB URL, but this covers the execution logic
    assert!(!success);
    assert!(err.contains("pool") || err.contains("connect") || err.contains("error") || err.contains("closed") || err.contains("timed out"));
}
