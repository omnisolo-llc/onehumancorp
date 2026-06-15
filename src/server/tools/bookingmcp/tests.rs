#[cfg(test)]
mod tests {
    use crate::bookingmcp::server::BookingMcpServer;
    use crate::ohc::orchestration::McpInvokeRequest;
    use ::server_auth::orchestration::AuthInfo;

    #[tokio::test]
    async fn test_booking_mcp_server_get_tools() {
        let server = BookingMcpServer::new(None);
        let tools = server.get_tools();
        assert_eq!(tools.len(), 2);
        assert_eq!(tools[0].id, "check_availability");
        assert_eq!(tools[1].id, "create_appointment");
    }

    #[tokio::test]
    async fn test_booking_mcp_server_invoke_tool_unknown() {
        let server = BookingMcpServer::new(None);
        let req = McpInvokeRequest {
            tool_id: "unknown_tool".to_string(),
            params: "{}".to_string(),
            spiffe_id: "test".to_string(),
        };
        let auth_info = AuthInfo {
            spiffe_id: "test".to_string(),
            org_id: "test_org".to_string(),
            agent_id: "test_agent".to_string(),
        };
        let res = server.invoke_tool(&req, auth_info).await;
        assert!(res.is_err());
        assert_eq!(res.unwrap_err().code(), tonic::Code::NotFound);
    }
}
