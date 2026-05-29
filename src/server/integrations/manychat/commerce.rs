use super::client::ManychatConversation;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ConversationalCommerceIntent {
    ProductQuestion,
    QuoteRequest,
    BookingDeposit,
    CheckoutReady,
    Support,
    Unknown,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CommerceCheckoutSeed {
    pub quote_required: bool,
    pub checkout_link_allowed: bool,
    pub currency: String,
    pub payment_provider: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CommerceConversationHandoff {
    pub tenant_id: String,
    pub source_channel: String,
    pub external_thread_id: String,
    pub external_customer_id: String,
    pub customer_name: String,
    pub latest_customer_message: Option<String>,
    pub inferred_intent: ConversationalCommerceIntent,
    pub checkout_seed: CommerceCheckoutSeed,
}

impl CommerceConversationHandoff {
    pub fn from_manychat(
        tenant_id: impl Into<String>,
        conversation: &ManychatConversation,
    ) -> Self {
        let latest_customer_message = conversation
            .messages
            .iter()
            .rev()
            .find(|message| message.direction == "inbound")
            .map(|message| message.body.clone());

        let inferred_intent = latest_customer_message
            .as_deref()
            .map(infer_commerce_intent)
            .unwrap_or(ConversationalCommerceIntent::Unknown);

        Self {
            tenant_id: tenant_id.into(),
            source_channel: conversation.channel.clone(),
            external_thread_id: conversation.id.clone(),
            external_customer_id: conversation.external_customer_id.clone(),
            customer_name: conversation.customer_name.clone(),
            latest_customer_message,
            checkout_seed: CommerceCheckoutSeed {
                quote_required: matches!(
                    inferred_intent,
                    ConversationalCommerceIntent::QuoteRequest
                        | ConversationalCommerceIntent::BookingDeposit
                ),
                checkout_link_allowed: matches!(
                    inferred_intent,
                    ConversationalCommerceIntent::CheckoutReady
                        | ConversationalCommerceIntent::BookingDeposit
                ),
                currency: "USD".to_string(),
                payment_provider: "stripe".to_string(),
            },
            inferred_intent,
        }
    }
}

fn infer_commerce_intent(message: &str) -> ConversationalCommerceIntent {
    let normalized = message.to_ascii_lowercase();

    if contains_any(
        &normalized,
        &["deposit", "book", "appointment", "available this weekend"],
    ) {
        ConversationalCommerceIntent::BookingDeposit
    } else if contains_any(&normalized, &["pay", "checkout", "buy", "invoice", "link"]) {
        ConversationalCommerceIntent::CheckoutReady
    } else if contains_any(
        &normalized,
        &["quote", "estimate", "custom", "how much", "price"],
    ) {
        ConversationalCommerceIntent::QuoteRequest
    } else if contains_any(&normalized, &["do you have", "in stock", "available"]) {
        ConversationalCommerceIntent::ProductQuestion
    } else if contains_any(&normalized, &["refund", "where is", "problem", "help"]) {
        ConversationalCommerceIntent::Support
    } else {
        ConversationalCommerceIntent::Unknown
    }
}

fn contains_any(haystack: &str, needles: &[&str]) -> bool {
    needles.iter().any(|needle| haystack.contains(needle))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::client::ManychatMessage;

    #[test]
    fn builds_tenant_scoped_booking_checkout_handoff() {
        let conversation = ManychatConversation {
            id: "thread_123".to_string(),
            channel: "instagram".to_string(),
            external_customer_id: "contact_123".to_string(),
            customer_name: "Maya".to_string(),
            status: "open".to_string(),
            messages: vec![ManychatMessage {
                id: "msg_123".to_string(),
                direction: "inbound".to_string(),
                sender_id: "contact_123".to_string(),
                body: "Can I book a vegan cake this weekend and pay a deposit?".to_string(),
                created_at_unix: 0,
            }],
        };

        let handoff = CommerceConversationHandoff::from_manychat("tenant_123", &conversation);

        assert_eq!(handoff.tenant_id, "tenant_123");
        assert_eq!(
            handoff.inferred_intent,
            ConversationalCommerceIntent::BookingDeposit
        );
        assert!(handoff.checkout_seed.quote_required);
        assert!(handoff.checkout_seed.checkout_link_allowed);
        assert_eq!(handoff.checkout_seed.payment_provider, "stripe");
    }

    #[test]
    fn marks_custom_price_requests_as_quote_required() {
        assert_eq!(
            infer_commerce_intent("How much for a custom birthday cake?"),
            ConversationalCommerceIntent::QuoteRequest
        );
    }
}
