use std::sync::RwLock;
use opentelemetry::global;
use opentelemetry::metrics::Counter;

pub struct ViralLoopTracker {
    invites_sent: RwLock<i32>,
    invites_accepted: RwLock<i32>,
    invites_sent_metric: Counter<u64>,
    invites_accepted_metric: Counter<u64>,
}

impl ViralLoopTracker {
    pub fn new() -> Self {
        let meter = global::meter("ohc.growth");
        let invites_sent_metric = meter.u64_counter("ohc.growth.viral_loop.invites_sent").build();
        let invites_accepted_metric = meter.u64_counter("ohc.growth.viral_loop.invites_accepted").build();

        ViralLoopTracker {
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

#[cfg(test)]
mod viral_loop_scenario_tests_1 {
    use super::*;
    use std::sync::Arc;
    use std::thread;

    #[test]
    fn test_concurrent_viral_loop_tracking_1() {
        let tracker = Arc::new(ViralLoopTracker::new());
        let mut handles = vec![];

        for j in 0..20 {
            let tracker_clone = tracker.clone();
            handles.push(thread::spawn(move || {
                tracker_clone.record_invite_sent(&format!("user_concurrent_{}", j));
                if j % 2 == 0 {
                    tracker_clone.record_invite_accepted(&format!("invitee_concurrent_{}", j));
                }
            }));
        }

        for handle in handles {
            handle.join().unwrap();
        }

        let k_factor = tracker.calculate_k_factor();
        assert_eq!(k_factor, 0.5);
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ViralLoopMetrics {
    pub total_sent: i32,
    pub total_accepted: i32,
    pub k_factor: f64,
    pub active_nodes: i32,
}

pub struct AdvancedViralLoopTracker {
    tracker: ViralLoopTracker,
    active_nodes: std::sync::atomic::AtomicI32,
}

impl AdvancedViralLoopTracker {
    pub fn new() -> Self {
        Self {
            tracker: ViralLoopTracker::new(),
            active_nodes: std::sync::atomic::AtomicI32::new(0),
        }
    }

    pub fn record_invite_sent(&self, user_id: &str) {
        self.tracker.record_invite_sent(user_id);
    }

    pub fn record_invite_accepted(&self, invitee_id: &str) {
        self.tracker.record_invite_accepted(invitee_id);
        self.active_nodes.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
    }

    pub fn get_metrics(&self) -> ViralLoopMetrics {
        ViralLoopMetrics {
            total_sent: *self.tracker.invites_sent.read().unwrap(),
            total_accepted: *self.tracker.invites_accepted.read().unwrap(),
            k_factor: self.tracker.calculate_k_factor(),
            active_nodes: self.active_nodes.load(std::sync::atomic::Ordering::SeqCst),
        }
    }
}

#[cfg(test)]
mod advanced_viral_tests {
    use super::*;

    #[test]
    fn test_advanced_viral_metrics() {
        let adv_tracker = AdvancedViralLoopTracker::new();

        adv_tracker.record_invite_sent("u1");
        adv_tracker.record_invite_sent("u2");
        adv_tracker.record_invite_accepted("i1");

        let metrics = adv_tracker.get_metrics();
        assert_eq!(metrics.total_sent, 2);
        assert_eq!(metrics.total_accepted, 1);
        assert_eq!(metrics.k_factor, 0.5);
        assert_eq!(metrics.active_nodes, 1);
    }
}
