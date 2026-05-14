pub struct HelpChatRouter;

impl HelpChatRouter {
    pub async fn route_query(query: &str) -> String {
        if query.to_lowercase().contains("refund") {
            "To refund a customer, go to your Payments tab, click on the specific transaction, and select the 'Refund' button. [Read more →](/help/payments)".to_string()
        } else {
            "I'm an AI assistant. I can help you find answers in our Help Center. What do you need help with?".to_string()
        }
    }
}
