pub struct HealthMonitor;

impl HealthMonitor {
    pub fn ping(&self, _agent_id: &str) -> bool {
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ping() {
        let monitor = HealthMonitor;
        assert_eq!(monitor.ping("agent1"), true);
    }
}
