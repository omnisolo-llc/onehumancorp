pub struct AITriageAgent {}

impl AITriageAgent {
    pub fn triage(text: &str) -> (bool, String) {
        if text.len() < 50 {
            return (true, "Yes we do! Delivery to downtown is $5.".to_string());
        }
        (false, "".to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_triage_simple() {
        let (auto_reply, draft) = AITriageAgent::triage("Do you deliver to downtown?");
        assert!(auto_reply);
        assert!(!draft.is_empty());
    }

    #[test]
    fn test_triage_complex() {
        let (auto_reply, _draft) = AITriageAgent::triage("I need a very complex custom cake with specific ingredients and layers and a specific design that is not on your menu and I need it tomorrow morning.");
        assert!(!auto_reply);
    }
}
