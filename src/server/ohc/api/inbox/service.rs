use tonic::{Request, Response, Status};
use uuid::Uuid;
use crate::inbox::inbox_service_server::InboxService;
use crate::inbox::{
    FetchConversationsRequest, FetchConversationsResponse,
    FetchMessagesRequest, FetchMessagesResponse,
    CreateTenantRequest, CreateTenantResponse,
    GetTenantRequest, GetTenantResponse, DeleteTenantRequest, DeleteTenantResponse,
    CreateInboxRequest, CreateInboxResponse, GetInboxRequest, GetInboxResponse,
    CreateChannelRequest, CreateChannelResponse, GetChannelRequest, GetChannelResponse,
    CreateContactRequest, CreateContactResponse, GetContactRequest, GetContactResponse,
    CreateConversationRequest, CreateConversationResponse, GetConversationRequest, GetConversationResponse,
    CreateMessageRequest, CreateMessageResponse, GetMessageRequest, GetMessageResponse,
    Tenant as ProtoTenant, Inbox as ProtoInbox, Channel as ProtoChannel, Contact as ProtoContact,
    Conversation as ProtoConversation, OmniMessage as ProtoMessage,
};
use crate::domain::inbox::repository::InboxRepository;
use std::sync::Arc;

pub struct InboxServiceImpl {
    repo: Arc<InboxRepository>,
}

impl InboxServiceImpl {
    pub fn new(repo: Arc<InboxRepository>) -> Self {
        Self { repo }
    }
}

#[tonic::async_trait]
impl InboxService for InboxServiceImpl {
    async fn fetch_conversations(
        &self,
        _request: Request<FetchConversationsRequest>,
    ) -> Result<Response<FetchConversationsResponse>, Status> {
        Err(Status::unimplemented("fetch_conversations is not implemented"))
    }

    async fn fetch_messages(
        &self,
        _request: Request<FetchMessagesRequest>,
    ) -> Result<Response<FetchMessagesResponse>, Status> {
        Err(Status::unimplemented("fetch_messages is not implemented"))
    }

    async fn create_tenant(
        &self,
        request: Request<CreateTenantRequest>,
    ) -> Result<Response<CreateTenantResponse>, Status> {
        let req = request.into_inner();
        let res = self.repo.create_tenant(req.name).await.map_err(|e| Status::internal(e.to_string()))?;
        Ok(Response::new(CreateTenantResponse {
            tenant: Some(ProtoTenant {
                id: res.id,
                name: res.name,
            }),
        }))
    }

    async fn get_tenant(
        &self,
        request: Request<GetTenantRequest>,
    ) -> Result<Response<GetTenantResponse>, Status> {
        let req = request.into_inner();
        let res = self.repo.get_tenant(&req.id).await.map_err(|e| Status::internal(e.to_string()))?;
        Ok(Response::new(GetTenantResponse {
            tenant: Some(ProtoTenant {
                id: res.id,
                name: res.name,
            }),
        }))
    }

    async fn delete_tenant(
        &self,
        _request: Request<DeleteTenantRequest>,
    ) -> Result<Response<DeleteTenantResponse>, Status> {
        Err(Status::unimplemented("delete_tenant is not implemented"))
    }

    async fn create_inbox(
        &self,
        request: Request<CreateInboxRequest>,
    ) -> Result<Response<CreateInboxResponse>, Status> {
        let req = request.into_inner();
        let res = self.repo.create_inbox(req.tenant_id, req.name).await.map_err(|e| Status::internal(e.to_string()))?;
        Ok(Response::new(CreateInboxResponse {
            inbox: Some(ProtoInbox {
                id: res.id,
                tenant_id: res.tenant_id,
                name: res.name,
            }),
        }))
    }

    async fn get_inbox(
        &self,
        request: Request<GetInboxRequest>,
    ) -> Result<Response<GetInboxResponse>, Status> {
        let req = request.into_inner();
        let res = self.repo.get_inbox(&req.tenant_id, &req.id).await.map_err(|e| Status::internal(e.to_string()))?;
        Ok(Response::new(GetInboxResponse {
            inbox: Some(ProtoInbox {
                id: res.id,
                tenant_id: res.tenant_id,
                name: res.name,
            }),
        }))
    }

    async fn create_channel(
        &self,
        request: Request<CreateChannelRequest>,
    ) -> Result<Response<CreateChannelResponse>, Status> {
        let req = request.into_inner();
        let creds: serde_json::Value = serde_json::from_str(&req.credentials).map_err(|e| Status::invalid_argument(e.to_string()))?;
        let res = self.repo.create_channel(req.tenant_id, req.inbox_id, req.provider_type, creds).await.map_err(|e| Status::internal(e.to_string()))?;
        Ok(Response::new(CreateChannelResponse {
            channel: Some(ProtoChannel {
                id: res.id,
                inbox_id: res.inbox_id,
                provider_type: res.provider_type,
                credentials: res.credentials.to_string(),
            }),
        }))
    }

    async fn get_channel(
        &self,
        request: Request<GetChannelRequest>,
    ) -> Result<Response<GetChannelResponse>, Status> {
        let req = request.into_inner();
        let res = self.repo.get_channel(&req.tenant_id, &req.id).await.map_err(|e| Status::internal(e.to_string()))?;
        Ok(Response::new(GetChannelResponse {
            channel: Some(ProtoChannel {
                id: res.id,
                inbox_id: res.inbox_id,
                provider_type: res.provider_type,
                credentials: res.credentials.to_string(),
            }),
        }))
    }

    async fn create_contact(
        &self,
        request: Request<CreateContactRequest>,
    ) -> Result<Response<CreateContactResponse>, Status> {
        let req = request.into_inner();
        let res = self.repo.create_contact(req.tenant_id.clone(), req.name, req.identifier).await.map_err(|e| Status::internal(e.to_string()))?;
        Ok(Response::new(CreateContactResponse {
            contact: Some(ProtoContact {
                id: res.id,
                tenant_id: res.tenant_id,
                name: res.name,
                identifier: res.identifier,
            }),
        }))
    }

    async fn get_contact(
        &self,
        request: Request<GetContactRequest>,
    ) -> Result<Response<GetContactResponse>, Status> {
        let req = request.into_inner();
        let res = self.repo.get_contact(&req.tenant_id, &req.id).await.map_err(|e| Status::internal(e.to_string()))?;
        Ok(Response::new(GetContactResponse {
            contact: Some(ProtoContact {
                id: res.id,
                tenant_id: res.tenant_id,
                name: res.name,
                identifier: res.identifier,
            }),
        }))
    }

    async fn create_conversation(
        &self,
        request: Request<CreateConversationRequest>,
    ) -> Result<Response<CreateConversationResponse>, Status> {
        let req = request.into_inner();
        let res = self.repo.create_conversation(req.tenant_id.clone(), req.inbox_id, req.contact_id, req.status).await.map_err(|e| Status::internal(e.to_string()))?;
        Ok(Response::new(CreateConversationResponse {
            conversation: Some(ProtoConversation {
                id: res.id,
                inbox_id: res.inbox_id,
                contact_id: res.contact_id,
                status: res.status,
                tenant_id: res.tenant_id,
                created_at_unix: res.created_at_unix,
                updated_at_unix: res.updated_at_unix,
                customer_id: String::new(), // Not in model yet
                channel: String::new(), // Not in model yet
            }),
        }))
    }

    async fn get_conversation(
        &self,
        request: Request<GetConversationRequest>,
    ) -> Result<Response<GetConversationResponse>, Status> {
        let req = request.into_inner();
        let res = self.repo.get_conversation(&req.tenant_id, &req.id).await.map_err(|e| Status::internal(e.to_string()))?;
        Ok(Response::new(GetConversationResponse {
            conversation: Some(ProtoConversation {
                id: res.id,
                inbox_id: res.inbox_id,
                contact_id: res.contact_id,
                status: res.status,
                tenant_id: res.tenant_id,
                created_at_unix: res.created_at_unix,
                updated_at_unix: res.updated_at_unix,
                customer_id: String::new(),
                channel: String::new(),
            }),
        }))
    }

    async fn create_message(
        &self,
        request: Request<CreateMessageRequest>,
    ) -> Result<Response<CreateMessageResponse>, Status> {
        let req = request.into_inner();
        let res = self.repo.create_message(req.tenant_id.clone(), req.conversation_id, req.content, req.sender_type, req.sender_id).await.map_err(|e| Status::internal(e.to_string()))?;
        Ok(Response::new(CreateMessageResponse {
            message: Some(ProtoMessage {
                id: res.id,
                conversation_id: res.conversation_id,
                content: res.content,
                sender_type: res.sender_type,
                sender_id: res.sender_id,
                tenant_id: res.tenant_id,
                created_at_unix: res.created_at_unix,
                updated_at_unix: res.updated_at_unix,
                source: String::new(),
                original_content: String::new(),
                translated_content: String::new(),
                source_language: String::new(),
                target_language: String::new(),
                draft_reply: String::new(),
                status: String::new(),
                customer_id: String::new(),
            }),
        }))
    }

    async fn get_message(
        &self,
        request: Request<GetMessageRequest>,
    ) -> Result<Response<GetMessageResponse>, Status> {
        let req = request.into_inner();
        let res = self.repo.get_message(&req.tenant_id, &req.id).await.map_err(|e| Status::internal(e.to_string()))?;
        Ok(Response::new(GetMessageResponse {
            message: Some(ProtoMessage {
                id: res.id,
                conversation_id: res.conversation_id,
                content: res.content,
                sender_type: res.sender_type,
                sender_id: res.sender_id,
                tenant_id: res.tenant_id,
                created_at_unix: res.created_at_unix,
                updated_at_unix: res.updated_at_unix,
                source: String::new(),
                original_content: String::new(),
                translated_content: String::new(),
                source_language: String::new(),
                target_language: String::new(),
                draft_reply: String::new(),
                status: String::new(),
                customer_id: String::new(),
            }),
        }))
    }
}
