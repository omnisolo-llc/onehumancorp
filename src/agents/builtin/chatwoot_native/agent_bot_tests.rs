#[cfg(test)]
mod tests {
    use super::super::agent_bot::{AgentBot, BotType};
    use super::super::conversation::{Conversation, ConversationStatus};
    use serde_json::json;

    #[test]
    fn test_agent_bot_creation() {
        let bot = AgentBot::new(1, "Test Bot".to_string());
        assert_eq!(bot.id, 1);
        assert_eq!(bot.name, "Test Bot");
        assert_eq!(bot.bot_type, BotType::Webhook);
        assert!(bot.system_bot());
    }

    #[test]
    fn test_agent_bot_webhook_data() {
        let bot = AgentBot::new(2, "Support Bot".to_string());
        let webhook_data = bot.webhook_data();
        assert_eq!(
            webhook_data,
            json!({
                "id": 2,
                "name": "Support Bot",
                "type": "agent_bot"
            })
        );
    }

    #[test]
    fn test_conversation_bot_handoff() {
        let mut conversation = Conversation::new(1, 100);
        conversation.status = ConversationStatus::Bot;
        conversation.assignee_agent_bot_id = Some(42);

        conversation.bot_handoff();

        assert_eq!(conversation.status, ConversationStatus::Open);
        assert!(conversation.assignee_agent_bot_id.is_none());
    }
}
