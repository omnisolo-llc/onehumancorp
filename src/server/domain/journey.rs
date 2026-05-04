use crate::ohc::journey::JourneyPhase;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct JourneyStateMachine;

impl JourneyStateMachine {
    /// Returns true if the transition from `current` to `target` is valid.
    pub fn is_valid_transition(current: JourneyPhase, target: JourneyPhase) -> bool {
        match (current, target) {
            // NEW can transition to ONBOARDING_STARTED
            (JourneyPhase::New, JourneyPhase::OnboardingStarted) => true,
            // ONBOARDING_STARTED can transition to STORE_LIVE
            (JourneyPhase::OnboardingStarted, JourneyPhase::StoreLive) => true,
            // STORE_LIVE can transition to FIRST_SALE
            (JourneyPhase::StoreLive, JourneyPhase::FirstSale) => true,
            // FIRST_SALE can transition to RETENTION_PHASE
            (JourneyPhase::FirstSale, JourneyPhase::RetentionPhase) => true,

            // Allow idempotency (transitioning to the same state is OK)
            (curr, targ) if curr == targ => true,

            // All other transitions are invalid
            _ => false,
        }
    }

    /// Convert a string phase to JourneyPhase enum
    pub fn parse_phase(phase_str: &str) -> JourneyPhase {
        match phase_str {
            "NEW" => JourneyPhase::New,
            "ONBOARDING_STARTED" => JourneyPhase::OnboardingStarted,
            "STORE_LIVE" => JourneyPhase::StoreLive,
            "FIRST_SALE" => JourneyPhase::FirstSale,
            "RETENTION_PHASE" => JourneyPhase::RetentionPhase,
            _ => JourneyPhase::New,
        }
    }

    /// Convert JourneyPhase enum to string phase
    pub fn phase_to_string(phase: JourneyPhase) -> &'static str {
        match phase {
            JourneyPhase::New => "NEW",
            JourneyPhase::OnboardingStarted => "ONBOARDING_STARTED",
            JourneyPhase::StoreLive => "STORE_LIVE",
            JourneyPhase::FirstSale => "FIRST_SALE",
            JourneyPhase::RetentionPhase => "RETENTION_PHASE",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_transitions() {
        assert!(JourneyStateMachine::is_valid_transition(JourneyPhase::New, JourneyPhase::OnboardingStarted));
        assert!(JourneyStateMachine::is_valid_transition(JourneyPhase::OnboardingStarted, JourneyPhase::StoreLive));
        assert!(JourneyStateMachine::is_valid_transition(JourneyPhase::StoreLive, JourneyPhase::FirstSale));
        assert!(JourneyStateMachine::is_valid_transition(JourneyPhase::FirstSale, JourneyPhase::RetentionPhase));
    }

    #[test]
    fn test_invalid_transitions() {
        assert!(!JourneyStateMachine::is_valid_transition(JourneyPhase::New, JourneyPhase::StoreLive));
        assert!(!JourneyStateMachine::is_valid_transition(JourneyPhase::FirstSale, JourneyPhase::New));
    }

    #[test]
    fn test_idempotent_transitions() {
        assert!(JourneyStateMachine::is_valid_transition(JourneyPhase::StoreLive, JourneyPhase::StoreLive));
    }
}
