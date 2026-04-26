use std::sync::RwLock;

pub struct ViralLoopTracker {
    invites_sent: RwLock<i32>,
    invites_accepted: RwLock<i32>,
}

impl ViralLoopTracker {
    pub fn new() -> Self {
        ViralLoopTracker {
            invites_sent: RwLock::new(0),
            invites_accepted: RwLock::new(0),
        }
    }

    pub fn record_invite_sent(&self, _user_id: &str) {
        let mut sent = self.invites_sent.write().unwrap();
        *sent += 1;
        // TODO: Track event in analytics
    }

    pub fn record_invite_accepted(&self, _invitee_id: &str) {
        let mut accepted = self.invites_accepted.write().unwrap();
        *accepted += 1;
        // TODO: Track event in analytics
    }

    pub fn calculate_k_factor(&self) -> f64 {
        let sent = self.invites_sent.read().unwrap();
        let accepted = self.invites_accepted.read().unwrap();

        if *sent == 0 {
            return 0.0;
        }

        *accepted as f64 / *sent as f64
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_viral_loop_tracker() {
        let tracker = ViralLoopTracker::new();

        tracker.record_invite_sent("user1");
        tracker.record_invite_sent("user2");
        tracker.record_invite_accepted("invitee1");

        let k_factor = tracker.calculate_k_factor();
        assert_eq!(k_factor, 0.5);
    }
}
