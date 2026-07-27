#[cfg(test)]
mod tests {
    use super::*;
    use tonic::Request;
    use crate::ohc::inbox::inbox_service_server::InboxService;

    #[tokio::test]
    async fn test_create_inbox() {
        let service = InboxServiceImplementation::default();
        let request = Request::new(crate::ohc::inbox::CreateInboxRequest {
            tenant_id: "test".to_string(),
            name: "test".to_string(),
            channel_type: "test".to_string(),
            config_json: "test".to_string(),
        });

        let result = service.create_inbox(request).await;
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().code(), tonic::Code::Unimplemented);
    }
}
