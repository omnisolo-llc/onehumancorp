use std::sync::RwLock;
use opentelemetry::global;
use opentelemetry::metrics::Counter;

pub struct ReferralScoreTracker {
    invites_sent: RwLock<i32>,
    invites_accepted: RwLock<i32>,
    invites_sent_metric: Counter<u64>,
    invites_accepted_metric: Counter<u64>,
}

impl ReferralScoreTracker {
    pub fn new() -> Self {
        let meter = global::meter("ohc.growth");
        let invites_sent_metric = meter.u64_counter("ohc.growth.referral_score.invites_sent").build();
        let invites_accepted_metric = meter.u64_counter("ohc.growth.referral_score.invites_accepted").build();

        ReferralScoreTracker {
            invites_sent: RwLock::new(0),
            invites_accepted: RwLock::new(0),
            invites_sent_metric,
            invites_accepted_metric,
        }
    }

    pub fn record_invite_sent(&self, _user_id: &str) {
        let mut sent = self.invites_sent.write().unwrap();
        *sent += 1;
        self.invites_sent_metric.add(1, &[]);
    }

    pub fn record_invite_accepted(&self, _invitee_id: &str) {
        let mut accepted = self.invites_accepted.write().unwrap();
        *accepted += 1;
        self.invites_accepted_metric.add(1, &[]);
    }

    pub fn calculate_referral_score(&self) -> f64 {
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
    fn test_referral_score_tracker() {
        let tracker = ReferralScoreTracker::new();
        
        tracker.record_invite_sent("user1");
        tracker.record_invite_sent("user2");
        tracker.record_invite_accepted("invitee1");
        
        let score = tracker.calculate_referral_score();
        assert_eq!(score, 0.5);
    }
}
