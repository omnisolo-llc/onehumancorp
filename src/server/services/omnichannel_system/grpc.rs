use crate::ohc::omnichannel::omnichannel_service_server::OmnichannelService;
use crate::ohc::omnichannel::{
    CreateConversationRequest, CreateConversationResponse, CreateInboxRequest,
    CreateInboxResponse, CreateMessageRequest, CreateMessageResponse,
    GetConversationRequest, GetConversationResponse, GetInboxRequest,
    GetInboxResponse, GetMessageRequest, GetMessageResponse, Conversation, Inbox, Message,
};
use tonic::{Request, Response, Status};

#[derive(Default)]
pub struct OmnichannelServiceImpl {}

#[tonic::async_trait]
impl OmnichannelService for OmnichannelServiceImpl {
    async fn create_inbox(
        &self,
        request: Request<CreateInboxRequest>,
    ) -> Result<Response<CreateInboxResponse>, Status> {
        let req = request.into_inner();
        let inbox = Inbox {
            id: uuid::Uuid::new_v4().to_string(),
            tenant_id: req.tenant_id,
            name: req.name,
        };
        Ok(Response::new(CreateInboxResponse {
            inbox: Some(inbox),
        }))
    }

    async fn get_inbox(
        &self,
        request: Request<GetInboxRequest>,
    ) -> Result<Response<GetInboxResponse>, Status> {
        let req = request.into_inner();
        let inbox = Inbox {
            id: req.id,
            tenant_id: "tenant-1".to_string(),
            name: "Default Inbox".to_string(),
        };
        Ok(Response::new(GetInboxResponse {
            inbox: Some(inbox),
        }))
    }

    async fn create_conversation(
        &self,
        request: Request<CreateConversationRequest>,
    ) -> Result<Response<CreateConversationResponse>, Status> {
        let req = request.into_inner();
        let conv = Conversation {
            id: uuid::Uuid::new_v4().to_string(),
            inbox_id: req.inbox_id,
            contact_id: req.contact_id,
            status: "open".to_string(),
        };
        Ok(Response::new(CreateConversationResponse {
            conversation: Some(conv),
        }))
    }

    async fn get_conversation(
        &self,
        request: Request<GetConversationRequest>,
    ) -> Result<Response<GetConversationResponse>, Status> {
        let req = request.into_inner();
        let conv = Conversation {
            id: req.id,
            inbox_id: "inbox-1".to_string(),
            contact_id: "contact-1".to_string(),
            status: "open".to_string(),
        };
        Ok(Response::new(GetConversationResponse {
            conversation: Some(conv),
        }))
    }

    async fn create_message(
        &self,
        request: Request<CreateMessageRequest>,
    ) -> Result<Response<CreateMessageResponse>, Status> {
        let req = request.into_inner();
        let msg = Message {
            id: uuid::Uuid::new_v4().to_string(),
            conversation_id: req.conversation_id,
            content: req.content,
            status: "sent".to_string(),
        };
        Ok(Response::new(CreateMessageResponse {
            message: Some(msg),
        }))
    }

    async fn get_message(
        &self,
        request: Request<GetMessageRequest>,
    ) -> Result<Response<GetMessageResponse>, Status> {
        let req = request.into_inner();
        let msg = Message {
            id: req.id,
            conversation_id: "conv-1".to_string(),
            content: "Hello".to_string(),
            status: "sent".to_string(),
        };
        Ok(Response::new(GetMessageResponse {
            message: Some(msg),
        }))
    }
}
