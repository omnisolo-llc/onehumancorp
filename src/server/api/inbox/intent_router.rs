use crate::proto::inbox::OmniMessage;


pub struct IntentRouter;

impl IntentRouter {
    pub fn new() -> Self {
        Self
    }

    pub fn classify(&self, message: &OmniMessage) -> (String, f64, String) {
        let content = message.original_content.to_lowercase();

        if content.contains("broken") || content.contains("not working") || content.contains("repair") {
            ("operations".to_string(), 0.85, "high".to_string())
        } else if content.contains("buy") || content.contains("price") || content.contains("cost") || content.contains("quote") {
            ("sales".to_string(), 0.90, "medium".to_string())
        } else if content.contains("cancel") || content.contains("refund") || content.contains("angry") {
            ("customer_service".to_string(), 0.95, "high".to_string())
        } else {
            ("general".to_string(), 0.50, "low".to_string())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::proto::inbox::OmniMessage;

    #[test]
    fn test_intent_router_classification() {
        let router = IntentRouter::new();

        let mut msg1 = OmniMessage::default();
        msg1.original_content = "My screen is broken, need repair".to_string();
        let (dept1, conf1, urg1) = router.classify(&msg1);
        assert_eq!(dept1, "operations");
        assert!(conf1 > 0.8);
        assert_eq!(urg1, "high");

        let mut msg2 = OmniMessage::default();
        msg2.original_content = "I want to buy the premium package".to_string();
        let (dept2, conf2, urg2) = router.classify(&msg2);
        assert_eq!(dept2, "sales");
        assert!(conf2 > 0.8);
        assert_eq!(urg2, "medium");

        let mut msg3 = OmniMessage::default();
        msg3.original_content = "Just saying hi!".to_string();
        let (dept3, conf3, urg3) = router.classify(&msg3);
        assert_eq!(dept3, "general");
        assert!(conf3 == 0.5);
        assert_eq!(urg3, "low");
    }
}
