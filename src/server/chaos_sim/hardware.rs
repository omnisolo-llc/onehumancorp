use super::ChaosConfig;

/// Simulates exotic hardware disruptions like CPU thermal throttling.
pub async fn inject_hardware_chaos(config: &ChaosConfig) -> Result<(), &'static str> {
    if !config.enabled {
        return Ok(());
    }

    if rand::random::<f64>() < (config.drop_rate * 0.005) {
        // Simulates severe clock-speed drop where thread essentially hangs briefly
        std::thread::sleep(std::time::Duration::from_millis(config.latency_ms * 5));
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_hardware_chaos_disabled() {
        let config = ChaosConfig::default();
        assert!(inject_hardware_chaos(&config).await.is_ok());
    }
}
