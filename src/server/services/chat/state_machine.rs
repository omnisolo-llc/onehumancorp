use uuid::Uuid;

#[derive(Debug, PartialEq, Clone)]
pub enum ConversationStatus {
    Open,
    Snoozed,
    Resolved,
    BotHandled,
}

impl ConversationStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            ConversationStatus::Open => "open",
            ConversationStatus::Snoozed => "snoozed",
            ConversationStatus::Resolved => "resolved",
            ConversationStatus::BotHandled => "bot_handled",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "open" => Some(ConversationStatus::Open),
            "snoozed" => Some(ConversationStatus::Snoozed),
            "resolved" => Some(ConversationStatus::Resolved),
            "bot_handled" => Some(ConversationStatus::BotHandled),
            _ => None,
        }
    }
}

pub struct ConversationStateMachine {
    pub conversation_id: Uuid,
    pub current_status: ConversationStatus,
}

impl ConversationStateMachine {
    pub fn new(conversation_id: Uuid, initial_status: ConversationStatus) -> Self {
        Self {
            conversation_id,
            current_status: initial_status,
        }
    }

    pub fn transition_to(&mut self, new_status: ConversationStatus) -> Result<(), String> {
        match (&self.current_status, &new_status) {
            (ConversationStatus::Open, ConversationStatus::Resolved) => {
                self.current_status = new_status;
                Ok(())
            },
            (ConversationStatus::Open, ConversationStatus::Snoozed) => {
                self.current_status = new_status;
                Ok(())
            },
            (ConversationStatus::Open, ConversationStatus::BotHandled) => {
                self.current_status = new_status;
                Ok(())
            },
            (ConversationStatus::Snoozed, ConversationStatus::Open) => {
                self.current_status = new_status;
                Ok(())
            },
            (ConversationStatus::BotHandled, ConversationStatus::Open) => {
                self.current_status = new_status;
                Ok(())
            },
            (ConversationStatus::BotHandled, ConversationStatus::Resolved) => {
                self.current_status = new_status;
                Ok(())
            },
            (ConversationStatus::Resolved, ConversationStatus::Open) => {
                self.current_status = new_status;
                Ok(())
            },
            (a, b) if a == b => {
                Ok(())
            },
            _ => Err(format!("Invalid transition from {:?} to {:?}", self.current_status, new_status))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use uuid::Uuid;

    #[test]
    fn test_status_conversion() {
        assert_eq!(ConversationStatus::Open.as_str(), "open");
        assert_eq!(ConversationStatus::from_str("open"), Some(ConversationStatus::Open));
        assert_eq!(ConversationStatus::from_str("invalid"), None);
    }

    #[test]
    fn test_valid_transitions() {
        let id = Uuid::new_v4();
        let mut sm = ConversationStateMachine::new(id, ConversationStatus::Open);

        assert!(sm.transition_to(ConversationStatus::Snoozed).is_ok());
        assert_eq!(sm.current_status, ConversationStatus::Snoozed);

        assert!(sm.transition_to(ConversationStatus::Open).is_ok());
        assert_eq!(sm.current_status, ConversationStatus::Open);

        assert!(sm.transition_to(ConversationStatus::BotHandled).is_ok());
        assert_eq!(sm.current_status, ConversationStatus::BotHandled);

        assert!(sm.transition_to(ConversationStatus::Resolved).is_ok());
        assert_eq!(sm.current_status, ConversationStatus::Resolved);
    }

    #[test]
    fn test_invalid_transitions() {
        let id = Uuid::new_v4();
        let mut sm = ConversationStateMachine::new(id, ConversationStatus::Snoozed);

        // Cannot go directly from Snoozed to Resolved
        assert!(sm.transition_to(ConversationStatus::Resolved).is_err());
    }
}
