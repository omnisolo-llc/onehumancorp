#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ConversationState {
    Open,
    Snoozed,
    BotHandling,
    HumanAssigned,
    Resolved,
}

impl std::str::FromStr for ConversationState {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "open" => Ok(ConversationState::Open),
            "snoozed" => Ok(ConversationState::Snoozed),
            "bothandling" | "bot_handling" | "bot handling" => Ok(ConversationState::BotHandling),
            "humanassigned" | "human_assigned" | "human assigned" => {
                Ok(ConversationState::HumanAssigned)
            }
            "resolved" => Ok(ConversationState::Resolved),
            _ => Err(format!("Invalid state: {}", s)),
        }
    }
}

impl std::fmt::Display for ConversationState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            ConversationState::Open => "open",
            ConversationState::Snoozed => "snoozed",
            ConversationState::BotHandling => "bot_handling",
            ConversationState::HumanAssigned => "human_assigned",
            ConversationState::Resolved => "resolved",
        };
        write!(f, "{}", s)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ConversationEvent {
    IncomingMessage,
    AssignToBot,
    AssignToHuman,
    Resolve,
    Snooze,
    Reopen,
}

pub struct ConversationStateMachine;

impl ConversationStateMachine {
    pub fn transition(
        current_state: &ConversationState,
        event: &ConversationEvent,
    ) -> Result<ConversationState, String> {
        match (current_state, event) {
            // From Open
            (ConversationState::Open, ConversationEvent::AssignToBot) => {
                Ok(ConversationState::BotHandling)
            }
            (ConversationState::Open, ConversationEvent::AssignToHuman) => {
                Ok(ConversationState::HumanAssigned)
            }
            (ConversationState::Open, ConversationEvent::Resolve) => {
                Ok(ConversationState::Resolved)
            }
            (ConversationState::Open, ConversationEvent::Snooze) => Ok(ConversationState::Snoozed),

            // From BotHandling
            (ConversationState::BotHandling, ConversationEvent::AssignToHuman) => {
                Ok(ConversationState::HumanAssigned)
            }
            (ConversationState::BotHandling, ConversationEvent::Resolve) => {
                Ok(ConversationState::Resolved)
            }

            // From HumanAssigned
            (ConversationState::HumanAssigned, ConversationEvent::Resolve) => {
                Ok(ConversationState::Resolved)
            }
            (ConversationState::HumanAssigned, ConversationEvent::Snooze) => {
                Ok(ConversationState::Snoozed)
            }

            // From Snoozed
            (ConversationState::Snoozed, ConversationEvent::IncomingMessage) => {
                Ok(ConversationState::Open)
            }
            (ConversationState::Snoozed, ConversationEvent::Reopen) => Ok(ConversationState::Open),
            (ConversationState::Snoozed, ConversationEvent::Snooze) => {
                Ok(ConversationState::Snoozed)
            } // Added Snoozed -> Snoozed

            // From Resolved
            (ConversationState::Resolved, ConversationEvent::IncomingMessage) => {
                Ok(ConversationState::Open)
            }
            (ConversationState::Resolved, ConversationEvent::Reopen) => Ok(ConversationState::Open),

            // Incoming message in active states (no-op transition)
            (ConversationState::Open, ConversationEvent::IncomingMessage) => {
                Ok(ConversationState::Open)
            }
            (ConversationState::BotHandling, ConversationEvent::IncomingMessage) => {
                Ok(ConversationState::BotHandling)
            }
            (ConversationState::HumanAssigned, ConversationEvent::IncomingMessage) => {
                Ok(ConversationState::HumanAssigned)
            }

            _ => Err(format!(
                "Invalid transition from {:?} with event {:?}",
                current_state, event
            )),
        }
    }
}
