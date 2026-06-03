use tonic::{Request, Response, Status};
#[allow(unused_imports)]
use ::server_ohc::orchestration::*;
use ::server_ohc::orchestration::unified_support_service_server::UnifiedSupportService;

pub struct ConfidenceRouter {
}

impl ConfidenceRouter {
    pub fn new() -> Self {
        ConfidenceRouter {}
    }
}

#[tonic::async_trait]
impl UnifiedSupportService for ConfidenceRouter {
    async fn process_message(
        &self,
        request: Request<::server_ohc::orchestration::ProcessUnifiedMessageRequest>,
    ) -> Result<Response<::server_ohc::orchestration::ProcessUnifiedMessageResponse>, Status> {
        let req = request.into_inner();
        let mut msg = req.message.unwrap();

        let action_taken = if msg.confidence_score > 0.85 {
            msg.status = "auto_replied".to_string();
            "auto_reply".to_string()
        } else {
            msg.status = "drafted".to_string();
            "escalate".to_string()
        };

        Ok(Response::new(::server_ohc::orchestration::ProcessUnifiedMessageResponse {
            message: Some(msg),
            action_taken,
        }))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_confidence_router_auto_reply() {
        let router = ConfidenceRouter::new();
        let msg = ::server_ohc::orchestration::UnifiedMessage {
            id: "1".to_string(),
            channel: "ig".to_string(),
            channel_id: "c1".to_string(),
            customer_id: "cust1".to_string(),
            intent_category: "faq".to_string(),
            confidence_score: 0.9,
            status: "new".to_string(),
            from_agent: "agent".to_string(),
            content: "hello".to_string(),
            thread_id: "thread".to_string(),
            timestamp_unix: 0,
            enriched_context_json: "{}".to_string(),
        };

        let req = Request::new(::server_ohc::orchestration::ProcessUnifiedMessageRequest {
            message: Some(msg),
        });

        let res = router.process_message(req).await.unwrap().into_inner();
        assert_eq!(res.action_taken, "auto_reply");
        assert_eq!(res.message.unwrap().status, "auto_replied");
    }

    #[tokio::test]
    async fn test_confidence_router_escalate() {
        let router = ConfidenceRouter::new();
        let msg = ::server_ohc::orchestration::UnifiedMessage {
            id: "2".to_string(),
            channel: "whatsapp".to_string(),
            channel_id: "c2".to_string(),
            customer_id: "cust2".to_string(),
            intent_category: "complex".to_string(),
            confidence_score: 0.5,
            status: "new".to_string(),
            from_agent: "agent".to_string(),
            content: "hello".to_string(),
            thread_id: "thread".to_string(),
            timestamp_unix: 0,
            enriched_context_json: "{}".to_string(),
        };

        let req = Request::new(::server_ohc::orchestration::ProcessUnifiedMessageRequest {
            message: Some(msg),
        });

        let res = router.process_message(req).await.unwrap().into_inner();
        assert_eq!(res.action_taken, "escalate");
        assert_eq!(res.message.unwrap().status, "drafted");
    }
}
