use tonic::{Request, Response, Status};
use crate::ohc::inbox::inbox_service_server::InboxService;
use crate::ohc::inbox::{
    CreateInboxRequest, CreateInboxResponse, GetInboxRequest, GetInboxResponse,
    ListInboxesRequest, ListInboxesResponse, CreateContactRequest,
    CreateContactResponse, GetContactRequest, GetContactResponse,
    ListContactsRequest, ListContactsResponse, CreateConversationRequest,
    CreateConversationResponse, GetConversationRequest, GetConversationResponse,
    ListConversationsRequest, ListConversationsResponse, CreateMessageRequest,
    CreateMessageResponse, ListMessagesRequest, ListMessagesResponse,
};

#[derive(Default)]
pub struct InboxServiceImplementation {}

#[tonic::async_trait]
impl InboxService for InboxServiceImplementation {
    async fn create_inbox(
        &self,
        _request: Request<CreateInboxRequest>,
    ) -> Result<Response<CreateInboxResponse>, Status> {
        Err(Status::unimplemented("Not yet implemented"))
    }

    async fn get_inbox(
        &self,
        _request: Request<GetInboxRequest>,
    ) -> Result<Response<GetInboxResponse>, Status> {
        Err(Status::unimplemented("Not yet implemented"))
    }

    async fn list_inboxes(
        &self,
        _request: Request<ListInboxesRequest>,
    ) -> Result<Response<ListInboxesResponse>, Status> {
        Err(Status::unimplemented("Not yet implemented"))
    }

    async fn create_contact(
        &self,
        _request: Request<CreateContactRequest>,
    ) -> Result<Response<CreateContactResponse>, Status> {
        Err(Status::unimplemented("Not yet implemented"))
    }

    async fn get_contact(
        &self,
        _request: Request<GetContactRequest>,
    ) -> Result<Response<GetContactResponse>, Status> {
        Err(Status::unimplemented("Not yet implemented"))
    }

    async fn list_contacts(
        &self,
        _request: Request<ListContactsRequest>,
    ) -> Result<Response<ListContactsResponse>, Status> {
        Err(Status::unimplemented("Not yet implemented"))
    }

    async fn create_conversation(
        &self,
        _request: Request<CreateConversationRequest>,
    ) -> Result<Response<CreateConversationResponse>, Status> {
        Err(Status::unimplemented("Not yet implemented"))
    }

    async fn get_conversation(
        &self,
        _request: Request<GetConversationRequest>,
    ) -> Result<Response<GetConversationResponse>, Status> {
        Err(Status::unimplemented("Not yet implemented"))
    }

    async fn list_conversations(
        &self,
        _request: Request<ListConversationsRequest>,
    ) -> Result<Response<ListConversationsResponse>, Status> {
        Err(Status::unimplemented("Not yet implemented"))
    }

    async fn create_message(
        &self,
        _request: Request<CreateMessageRequest>,
    ) -> Result<Response<CreateMessageResponse>, Status> {
        Err(Status::unimplemented("Not yet implemented"))
    }

    async fn list_messages(
        &self,
        _request: Request<ListMessagesRequest>,
    ) -> Result<Response<ListMessagesResponse>, Status> {
        Err(Status::unimplemented("Not yet implemented"))
    }
}
