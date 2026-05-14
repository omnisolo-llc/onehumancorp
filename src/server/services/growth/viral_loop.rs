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

#[cfg(test)]
mod viral_loop_scenario_tests_2 {
    use super::*;
    use std::sync::Arc;
    use std::thread;

    #[test]
    fn test_concurrent_viral_loop_tracking_2() {
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

#[cfg(test)]
mod viral_loop_scenario_tests_3 {
    use super::*;
    use std::sync::Arc;
    use std::thread;

    #[test]
    fn test_concurrent_viral_loop_tracking_3() {
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

#[cfg(test)]
mod viral_loop_scenario_tests_4 {
    use super::*;
    use std::sync::Arc;
    use std::thread;

    #[test]
    fn test_concurrent_viral_loop_tracking_4() {
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

#[cfg(test)]
mod viral_loop_scenario_tests_5 {
    use super::*;
    use std::sync::Arc;
    use std::thread;

    #[test]
    fn test_concurrent_viral_loop_tracking_5() {
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

#[cfg(test)]
mod viral_loop_scenario_tests_6 {
    use super::*;
    use std::sync::Arc;
    use std::thread;

    #[test]
    fn test_concurrent_viral_loop_tracking_6() {
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

#[cfg(test)]
mod viral_loop_scenario_tests_7 {
    use super::*;
    use std::sync::Arc;
    use std::thread;

    #[test]
    fn test_concurrent_viral_loop_tracking_7() {
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

#[cfg(test)]
mod viral_loop_scenario_tests_8 {
    use super::*;
    use std::sync::Arc;
    use std::thread;

    #[test]
    fn test_concurrent_viral_loop_tracking_8() {
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

#[cfg(test)]
mod viral_loop_scenario_tests_9 {
    use super::*;
    use std::sync::Arc;
    use std::thread;

    #[test]
    fn test_concurrent_viral_loop_tracking_9() {
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

#[cfg(test)]
mod viral_loop_scenario_tests_10 {
    use super::*;
    use std::sync::Arc;
    use std::thread;

    #[test]
    fn test_concurrent_viral_loop_tracking_10() {
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

#[cfg(test)]
mod viral_loop_scenario_tests_11 {
    use super::*;
    use std::sync::Arc;
    use std::thread;

    #[test]
    fn test_concurrent_viral_loop_tracking_11() {
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

#[cfg(test)]
mod viral_loop_scenario_tests_12 {
    use super::*;
    use std::sync::Arc;
    use std::thread;

    #[test]
    fn test_concurrent_viral_loop_tracking_12() {
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

#[cfg(test)]
mod viral_loop_scenario_tests_13 {
    use super::*;
    use std::sync::Arc;
    use std::thread;

    #[test]
    fn test_concurrent_viral_loop_tracking_13() {
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

#[cfg(test)]
mod viral_loop_scenario_tests_14 {
    use super::*;
    use std::sync::Arc;
    use std::thread;

    #[test]
    fn test_concurrent_viral_loop_tracking_14() {
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

#[cfg(test)]
mod viral_loop_scenario_tests_15 {
    use super::*;
    use std::sync::Arc;
    use std::thread;

    #[test]
    fn test_concurrent_viral_loop_tracking_15() {
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

#[cfg(test)]
mod viral_loop_scenario_tests_16 {
    use super::*;
    use std::sync::Arc;
    use std::thread;

    #[test]
    fn test_concurrent_viral_loop_tracking_16() {
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

#[cfg(test)]
mod viral_loop_scenario_tests_17 {
    use super::*;
    use std::sync::Arc;
    use std::thread;

    #[test]
    fn test_concurrent_viral_loop_tracking_17() {
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

#[cfg(test)]
mod viral_loop_scenario_tests_18 {
    use super::*;
    use std::sync::Arc;
    use std::thread;

    #[test]
    fn test_concurrent_viral_loop_tracking_18() {
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

#[cfg(test)]
mod viral_loop_scenario_tests_19 {
    use super::*;
    use std::sync::Arc;
    use std::thread;

    #[test]
    fn test_concurrent_viral_loop_tracking_19() {
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

#[cfg(test)]
mod viral_loop_scenario_tests_20 {
    use super::*;
    use std::sync::Arc;
    use std::thread;

    #[test]
    fn test_concurrent_viral_loop_tracking_20() {
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

#[cfg(test)]
mod viral_loop_scenario_tests_21 {
    use super::*;
    use std::sync::Arc;
    use std::thread;

    #[test]
    fn test_concurrent_viral_loop_tracking_21() {
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

#[cfg(test)]
mod viral_loop_scenario_tests_22 {
    use super::*;
    use std::sync::Arc;
    use std::thread;

    #[test]
    fn test_concurrent_viral_loop_tracking_22() {
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

#[cfg(test)]
mod viral_loop_scenario_tests_23 {
    use super::*;
    use std::sync::Arc;
    use std::thread;

    #[test]
    fn test_concurrent_viral_loop_tracking_23() {
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

#[cfg(test)]
mod viral_loop_scenario_tests_24 {
    use super::*;
    use std::sync::Arc;
    use std::thread;

    #[test]
    fn test_concurrent_viral_loop_tracking_24() {
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

#[cfg(test)]
mod viral_loop_scenario_tests_25 {
    use super::*;
    use std::sync::Arc;
    use std::thread;

    #[test]
    fn test_concurrent_viral_loop_tracking_25() {
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

#[cfg(test)]
mod viral_loop_scenario_tests_26 {
    use super::*;
    use std::sync::Arc;
    use std::thread;

    #[test]
    fn test_concurrent_viral_loop_tracking_26() {
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

#[cfg(test)]
mod viral_loop_scenario_tests_27 {
    use super::*;
    use std::sync::Arc;
    use std::thread;

    #[test]
    fn test_concurrent_viral_loop_tracking_27() {
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

#[cfg(test)]
mod viral_loop_scenario_tests_28 {
    use super::*;
    use std::sync::Arc;
    use std::thread;

    #[test]
    fn test_concurrent_viral_loop_tracking_28() {
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

#[cfg(test)]
mod viral_loop_scenario_tests_29 {
    use super::*;
    use std::sync::Arc;
    use std::thread;

    #[test]
    fn test_concurrent_viral_loop_tracking_29() {
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

#[cfg(test)]
mod viral_loop_scenario_tests_30 {
    use super::*;
    use std::sync::Arc;
    use std::thread;

    #[test]
    fn test_concurrent_viral_loop_tracking_30() {
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

#[cfg(test)]
mod viral_loop_scenario_tests_31 {
    use super::*;
    use std::sync::Arc;
    use std::thread;

    #[test]
    fn test_concurrent_viral_loop_tracking_31() {
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

#[cfg(test)]
mod viral_loop_scenario_tests_32 {
    use super::*;
    use std::sync::Arc;
    use std::thread;

    #[test]
    fn test_concurrent_viral_loop_tracking_32() {
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

#[cfg(test)]
mod viral_loop_scenario_tests_33 {
    use super::*;
    use std::sync::Arc;
    use std::thread;

    #[test]
    fn test_concurrent_viral_loop_tracking_33() {
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

#[cfg(test)]
mod viral_loop_scenario_tests_34 {
    use super::*;
    use std::sync::Arc;
    use std::thread;

    #[test]
    fn test_concurrent_viral_loop_tracking_34() {
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

#[cfg(test)]
mod viral_loop_scenario_tests_35 {
    use super::*;
    use std::sync::Arc;
    use std::thread;

    #[test]
    fn test_concurrent_viral_loop_tracking_35() {
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

#[cfg(test)]
mod viral_loop_scenario_tests_36 {
    use super::*;
    use std::sync::Arc;
    use std::thread;

    #[test]
    fn test_concurrent_viral_loop_tracking_36() {
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

#[cfg(test)]
mod viral_loop_scenario_tests_37 {
    use super::*;
    use std::sync::Arc;
    use std::thread;

    #[test]
    fn test_concurrent_viral_loop_tracking_37() {
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

#[cfg(test)]
mod viral_loop_scenario_tests_38 {
    use super::*;
    use std::sync::Arc;
    use std::thread;

    #[test]
    fn test_concurrent_viral_loop_tracking_38() {
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

#[cfg(test)]
mod viral_loop_scenario_tests_39 {
    use super::*;
    use std::sync::Arc;
    use std::thread;

    #[test]
    fn test_concurrent_viral_loop_tracking_39() {
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
