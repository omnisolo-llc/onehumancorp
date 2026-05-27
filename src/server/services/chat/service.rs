use tonic::{Request, Response, Status};
use ::server_ohc::orchestration::*;
use ::server_ohc::orchestration::chat_service_server::ChatService;
use crate::integrations::registry::IntegrationsRegistry;

pub struct MyChatService {
    registry: std::sync::Arc<IntegrationsRegistry>,
}

impl MyChatService {
    pub fn new(registry: std::sync::Arc<IntegrationsRegistry>) -> Self {
        MyChatService { registry }
    }
}

#[tonic::async_trait]
impl ChatService for MyChatService {
    async fn test_connection(
        &self,
        request: Request<::server_ohc::orchestration::ChatTestRequest>,
    ) -> Result<Response<::server_ohc::orchestration::ChatTestResponse>, Status> {
        let req = request.into_inner();
        
        match self.registry.test_connection(&req.integration_id, req.clone()) {
            Ok(_) => Ok(Response::new(::server_ohc::orchestration::ChatTestResponse { success: true })),
            Err(e) => Err(Status::invalid_argument(e)),
        }
    }

    async fn get_chat_messages(
        &self,
        request: Request<::server_ohc::orchestration::GetChatMessagesRequest>,
    ) -> Result<Response<::server_ohc::orchestration::GetChatMessagesResponse>, Status> {
        let req = request.into_inner();
        let messages = self.registry.chat_messages(&req.integration_id);
        Ok(Response::new(::server_ohc::orchestration::GetChatMessagesResponse { messages }))
    }

    async fn send_chat_message(
        &self,
        request: Request<::server_ohc::orchestration::ChatSendRequest>,
    ) -> Result<Response<::server_ohc::orchestration::ChatMessage>, Status> {
        let req = request.into_inner();
        
        match self.registry.send_chat_message(&req.integration_id, &req.channel, &req.from_agent, &req.content, &req.thread_id) {
            Ok(msg) => Ok(Response::new(msg)),
            Err(e) => Err(Status::internal(e)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;

    #[tokio::test]
    async fn test_chat_service() {
        let registry = Arc::new(IntegrationsRegistry::new());
        let service = MyChatService::new(registry);

        let req = Request::new(::server_ohc::orchestration::ChatTestRequest {
            integration_id: "test-int".to_string(),
            bot_token: "".to_string(),
            chat_id: "".to_string(),
            webhook_url: "".to_string(),
            api_token: "".to_string(),
        });
        let resp = service.test_connection(req).await.unwrap();
        assert!(resp.get_ref().success);

        let req = Request::new(::server_ohc::orchestration::ChatSendRequest {
            integration_id: "test-int".to_string(),
            channel: "test-chan".to_string(),
            from_agent: "agent-1".to_string(),
            content: "hello".to_string(),
            thread_id: "thread-1".to_string(),
        });
        let resp = service.send_chat_message(req).await.unwrap();
        assert_eq!(resp.get_ref().content, "hello");

        let req = Request::new(::server_ohc::orchestration::GetChatMessagesRequest {
            integration_id: "test-int".to_string(),
        });
        let resp = service.get_chat_messages(req).await.unwrap();
        assert_eq!(resp.get_ref().messages.len(), 1);
        assert_eq!(resp.get_ref().messages[0].content, "hello");
    }
}
