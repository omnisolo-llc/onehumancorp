use tonic::codec::ProstCodec;
use tonic::transport::Channel;
use tonic::{Request, Status};

#[derive(Clone, PartialEq, prost::Message)]
pub struct Agent {
    #[prost(string, tag = "1")]
    pub id: String,
    #[prost(string, tag = "2")]
    pub name: String,
    #[prost(string, tag = "3")]
    pub role: String,
    #[prost(string, tag = "4")]
    pub organization_id: String,
    #[prost(string, tag = "5")]
    pub status: String,
    #[prost(string, tag = "6")]
    pub provider_type: String,
}

#[derive(Clone, PartialEq, prost::Message)]
pub struct HubMessage {
    #[prost(string, tag = "1")]
    pub id: String,
    #[prost(string, tag = "2")]
    pub from_agent: String,
    #[prost(string, tag = "3")]
    pub to_agent: String,
    #[prost(string, tag = "4")]
    pub r#type: String,
    #[prost(string, tag = "5")]
    pub content: String,
    #[prost(string, tag = "6")]
    pub meeting_id: String,
    #[prost(int64, tag = "7")]
    pub occurred_at_unix: i64,
}

#[derive(Clone, PartialEq, prost::Message)]
pub struct RegisterAgentRequest {
    #[prost(message, optional, tag = "1")]
    pub agent: Option<Agent>,
}

#[derive(Clone, PartialEq, prost::Message)]
pub struct RegisterAgentResponse {
    #[prost(bool, tag = "1")]
    pub success: bool,
}

#[derive(Clone, PartialEq, prost::Message)]
pub struct PublishMessageRequest {
    #[prost(message, optional, tag = "1")]
    pub message: Option<HubMessage>,
}

#[derive(Clone, PartialEq, prost::Message)]
pub struct PublishMessageResponse {
    #[prost(bool, tag = "1")]
    pub success: bool,
}

#[derive(Clone, PartialEq, prost::Message)]
pub struct StreamMessagesRequest {
    #[prost(string, tag = "1")]
    pub agent_id: String,
}

pub struct HubServiceClient {
    inner: tonic::client::Grpc<Channel>,
}

impl HubServiceClient {
    pub async fn connect(endpoint: String) -> anyhow::Result<Self> {
        let channel = tonic::transport::Channel::from_shared(endpoint)?
            .connect()
            .await?;
        Ok(Self {
            inner: tonic::client::Grpc::new(channel),
        })
    }

    pub async fn register_agent(
        &mut self,
        request: RegisterAgentRequest,
    ) -> Result<RegisterAgentResponse, Status> {
        self.inner.ready().await.map_err(|e| {
            Status::new(tonic::Code::Unknown, format!("service not ready: {}", e))
        })?;
        let codec = ProstCodec::<RegisterAgentRequest, RegisterAgentResponse>::default();
        let path = http::uri::PathAndQuery::from_static(
            "/ohc.orchestration.HubService/RegisterAgent",
        );
        self.inner
            .unary(Request::new(request), path, codec)
            .await
            .map(|r| r.into_inner())
    }

    pub async fn publish(
        &mut self,
        request: PublishMessageRequest,
    ) -> Result<PublishMessageResponse, Status> {
        self.inner.ready().await.map_err(|e| {
            Status::new(tonic::Code::Unknown, format!("service not ready: {}", e))
        })?;
        let codec = ProstCodec::<PublishMessageRequest, PublishMessageResponse>::default();
        let path =
            http::uri::PathAndQuery::from_static("/ohc.orchestration.HubService/Publish");
        self.inner
            .unary(Request::new(request), path, codec)
            .await
            .map(|r| r.into_inner())
    }

    pub async fn stream_messages(
        &mut self,
        agent_id: String,
    ) -> Result<tonic::Streaming<HubMessage>, Status> {
        self.inner.ready().await.map_err(|e| {
            Status::new(tonic::Code::Unknown, format!("service not ready: {}", e))
        })?;
        let codec = ProstCodec::<StreamMessagesRequest, HubMessage>::default();
        let path = http::uri::PathAndQuery::from_static(
            "/ohc.orchestration.HubService/StreamMessages",
        );
        self.inner
            .server_streaming(
                Request::new(StreamMessagesRequest { agent_id }),
                path,
                codec,
            )
            .await
            .map(|r| r.into_inner())
    }
}
