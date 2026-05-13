use tonic::{Request, Response, Status};
use crate::ohc::orchestration::*;
use crate::ohc::orchestration::chat_service_server::ChatService;
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
        request: Request<ChatTestRequest>,
    ) -> Result<Response<ChatTestResponse>, Status> {
        let req = request.into_inner();
        
        match self.registry.test_connection(&req.integration_id, req.clone()) {
            Ok(_) => Ok(Response::new(ChatTestResponse { success: true })),
            Err(e) => Err(Status::invalid_argument(e)),
        }
    }

    async fn get_chat_messages(
        &self,
        request: Request<GetChatMessagesRequest>,
    ) -> Result<Response<GetChatMessagesResponse>, Status> {
        let tenant_id = request.extensions().get::<crate::auth::Claims>()
            .and_then(|c| c.organization_id.clone())
            .ok_or_else(|| Status::permission_denied("Missing organization_id in claims"))?;
        let req = request.into_inner();
        let messages = self.registry.chat_messages(&tenant_id, &req.integration_id);
        Ok(Response::new(GetChatMessagesResponse { messages }))
    }

    async fn send_chat_message(
        &self,
        request: Request<ChatSendRequest>,
    ) -> Result<Response<ChatMessage>, Status> {
        let tenant_id = request.extensions().get::<crate::auth::Claims>()
            .and_then(|c| c.organization_id.clone())
            .ok_or_else(|| Status::permission_denied("Missing organization_id in claims"))?;
        let req = request.into_inner();
        
        match self.registry.send_chat_message(&tenant_id, &req.integration_id, &req.channel, &req.from_agent, &req.content, &req.thread_id) {
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

        let req = Request::new(ChatTestRequest {
            integration_id: "test-int".to_string(),
            bot_token: "".to_string(),
            chat_id: "".to_string(),
            webhook_url: "".to_string(),
            api_token: "".to_string(),
        });
        let resp = service.test_connection(req).await.unwrap();
        assert!(resp.get_ref().success);

        let mut req = Request::new(ChatSendRequest {
            integration_id: "test-int".to_string(),
            channel: "test-chan".to_string(),
            from_agent: "agent-1".to_string(),
            content: "hello".to_string(),
            thread_id: "thread-1".to_string(),
        });
        req.extensions_mut().insert(crate::auth::Claims {
            sub: "test".to_string(),
            username: "test".to_string(),
            email: "test".to_string(),
            roles: vec![],
            organization_id: Some("tenant1".to_string()),
            session_id: None,
            iat: 0,
            exp: 0,
            jti: "test".to_string(),
        });
        let resp = service.send_chat_message(req).await.unwrap();
        assert_eq!(resp.get_ref().content, "hello");

        let mut req = Request::new(GetChatMessagesRequest {
            integration_id: "test-int".to_string(),
        });
        req.extensions_mut().insert(crate::auth::Claims {
            sub: "test".to_string(),
            username: "test".to_string(),
            email: "test".to_string(),
            roles: vec![],
            organization_id: Some("tenant1".to_string()),
            session_id: None,
            iat: 0,
            exp: 0,
            jti: "test".to_string(),
        });
        let resp = service.get_chat_messages(req).await.unwrap();
        assert_eq!(resp.get_ref().messages.len(), 1);
        assert_eq!(resp.get_ref().messages[0].content, "hello");
    }
}
