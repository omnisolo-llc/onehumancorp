use server_domain_inbox::models::{ActionRequiredDraft, IncomingMessage, UnifiedCustomerGraph};
use uuid::Uuid;

pub struct AmbassadorAgent;

impl AmbassadorAgent {
    pub fn process_message(
        message: &IncomingMessage,
        customer_context: Option<&UnifiedCustomerGraph>,
    ) -> ActionRequiredDraft {
        // Mock LLM prompt and response generation
        let drafted_reply = if let Some(context) = customer_context {
            format!(
                "Hi {}! Thanks for reaching out about: '{}'. I see you're interested in our products based on your past orders. How can I help you today?",
                context.name, message.message_content
            )
        } else {
            format!(
                "Hi there! Thanks for reaching out about: '{}'. How can I help you today?",
                message.message_content
            )
        };

        ActionRequiredDraft {
            tenant_id: message.tenant_id.clone(),
            draft_id: Uuid::new_v4().to_string(),
            customer_id: customer_context.map(|c| c.customer_id.clone()).unwrap_or_else(|| "unknown".to_string()),
            original_message: message.clone(),
            drafted_reply,
            status: "pending_approval".to_string(),
        }
    }
}
