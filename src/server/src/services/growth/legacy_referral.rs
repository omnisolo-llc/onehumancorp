use crate::analytics::Tracker;
use std::collections::HashMap;
use std::sync::Arc;

pub struct ReferralService {
    tracker: Arc<Tracker>,
}

impl ReferralService {
    pub fn new(tracker: Arc<Tracker>) -> Self {
        ReferralService { tracker }
    }

    pub fn process_invite(&self, sender_id: &str, receiver_email: &str) -> Result<(), String> {
        if sender_id.is_empty() || receiver_email.is_empty() {
            return Err("invalid invite parameters".to_string());
        }

        let mut props = HashMap::new();
        props.insert("sender_id".to_string(), sender_id.to_string());
        props.insert("receiver_email".to_string(), receiver_email.to_string());

        self.tracker.track_event("invite_sent", props);
        Ok(())
    }

    pub fn accept_invite(&self, invite_id: &str) -> Result<(), String> {
        if invite_id.is_empty() {
            return Err("invalid invite ID".to_string());
        }

        let mut props = HashMap::new();
        props.insert("invite_id".to_string(), invite_id.to_string());

        self.tracker.track_event("invite_accepted", props);
        Ok(())
    }

    pub fn process_bulk_invites(&self, sender_id: &str, receiver_emails: Vec<String>) -> Result<(), String> {
        if sender_id.is_empty() || receiver_emails.is_empty() {
            return Err("invalid bulk invite parameters".to_string());
        }

        let mut props = HashMap::new();
        props.insert("sender_id".to_string(), sender_id.to_string());
        props.insert("receiver_emails".to_string(), format!("{:?}", receiver_emails));

        self.tracker.track_event("bulk_invite_sent", props);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::analytics::Tracker;

    #[test]
    fn test_process_invite() {
        let tracker = Arc::new(Tracker::new());
        let service = ReferralService::new(tracker);

        let res = service.process_invite("user-123", "test@example.com");
        assert!(res.is_ok());

        let res = service.process_invite("", "");
        assert!(res.is_err());
    }

    #[test]
    fn test_accept_invite() {
        let tracker = Arc::new(Tracker::new());
        let service = ReferralService::new(tracker);

        let res = service.accept_invite("invite-123");
        assert!(res.is_ok());

        let res = service.accept_invite("");
        assert!(res.is_err());
    }

    #[test]
    fn test_process_bulk_invites() {
        let tracker = Arc::new(Tracker::new());
        let service = ReferralService::new(tracker);

        let res = service.process_bulk_invites("user-123", vec!["test1@example.com".to_string(), "test2@example.com".to_string()]);
        assert!(res.is_ok());

        let res = service.process_bulk_invites("", vec![]);
        assert!(res.is_err());
    }
}
