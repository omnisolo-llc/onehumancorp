use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::domain::repository::omnichannel_repo::OmniChannelRepo;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ConversationState {
    Open,
    Snoozed,
    BotHandling,
    HumanAssigned,
    Resolved,
}

impl ToString for ConversationState {
    fn to_string(&self) -> String {
        match self {
            ConversationState::Open => "Open".to_string(),
            ConversationState::Snoozed => "Snoozed".to_string(),
            ConversationState::BotHandling => "BotHandling".to_string(),
            ConversationState::HumanAssigned => "HumanAssigned".to_string(),
            ConversationState::Resolved => "Resolved".to_string(),
        }
    }
}

impl std::str::FromStr for ConversationState {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Open" => Ok(ConversationState::Open),
            "Snoozed" => Ok(ConversationState::Snoozed),
            "BotHandling" => Ok(ConversationState::BotHandling),
            "HumanAssigned" => Ok(ConversationState::HumanAssigned),
            "Resolved" => Ok(ConversationState::Resolved),
            _ => Err(format!("Invalid state: {}", s)),
        }
    }
}

pub struct ConversationStateMachine<'a> {
    repo: &'a OmniChannelRepo,
    conversation_id: Uuid,
    current_state: ConversationState,
}

impl<'a> ConversationStateMachine<'a> {
    pub async fn new(repo: &'a OmniChannelRepo, conversation_id: Uuid) -> Result<Self, String> {
        let conversation = repo
            .get_conversation(conversation_id)
            .await
            .map_err(|e| e.to_string())?
            .ok_or_else(|| "Conversation not found".to_string())?;

        let current_state = conversation
            .status
            .parse::<ConversationState>()
            .map_err(|e| e.to_string())?;

        Ok(Self {
            repo,
            conversation_id,
            current_state,
        })
    }

    pub fn current_state(&self) -> &ConversationState {
        &self.current_state
    }

    pub async fn transition_to(&mut self, new_state: ConversationState) -> Result<(), String> {
        // Validate transition
        let valid_transition = match (&self.current_state, &new_state) {
            (ConversationState::Open, ConversationState::BotHandling) => true,
            (ConversationState::Open, ConversationState::HumanAssigned) => true,
            (ConversationState::Open, ConversationState::Snoozed) => true,
            (ConversationState::Open, ConversationState::Resolved) => true,

            (ConversationState::BotHandling, ConversationState::HumanAssigned) => true,
            (ConversationState::BotHandling, ConversationState::Resolved) => true,
            (ConversationState::BotHandling, ConversationState::Open) => true,

            (ConversationState::HumanAssigned, ConversationState::Snoozed) => true,
            (ConversationState::HumanAssigned, ConversationState::Resolved) => true,
            (ConversationState::HumanAssigned, ConversationState::Open) => true,

            (ConversationState::Snoozed, ConversationState::Open) => true,
            (ConversationState::Snoozed, ConversationState::BotHandling) => true,
            (ConversationState::Snoozed, ConversationState::HumanAssigned) => true,

            (ConversationState::Resolved, ConversationState::Open) => true, // Reopen

            _ => false,
        };

        if !valid_transition {
            return Err(format!(
                "Invalid state transition from {:?} to {:?}",
                self.current_state, new_state
            ));
        }

        // Persist
        self.repo
            .update_conversation_status(self.conversation_id, new_state.to_string())
            .await
            .map_err(|e| e.to_string())?;

        self.current_state = new_state;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_conversation_state_parsing() {
        assert_eq!("Open".parse::<ConversationState>().unwrap(), ConversationState::Open);
        assert_eq!("BotHandling".parse::<ConversationState>().unwrap(), ConversationState::BotHandling);
        assert_eq!("HumanAssigned".parse::<ConversationState>().unwrap(), ConversationState::HumanAssigned);
        assert_eq!("Resolved".parse::<ConversationState>().unwrap(), ConversationState::Resolved);
        assert_eq!("Snoozed".parse::<ConversationState>().unwrap(), ConversationState::Snoozed);

        assert!("InvalidState".parse::<ConversationState>().is_err());
    }

    #[test]
    fn test_conversation_state_to_string() {
        assert_eq!(ConversationState::Open.to_string(), "Open");
        assert_eq!(ConversationState::BotHandling.to_string(), "BotHandling");
        assert_eq!(ConversationState::HumanAssigned.to_string(), "HumanAssigned");
        assert_eq!(ConversationState::Resolved.to_string(), "Resolved");
        assert_eq!(ConversationState::Snoozed.to_string(), "Snoozed");
    }
}
