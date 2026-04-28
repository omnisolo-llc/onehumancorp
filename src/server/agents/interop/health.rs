use async_trait::async_trait;

#[async_trait]
pub trait HealthProvider: Send + Sync {
    async fn ping(&self) -> Result<String, String>;
}

pub struct CloudHealthProvider {
    // Usually a reqwest client to hit a known ping endpoint
}

impl CloudHealthProvider {
    pub fn new() -> Self {
        CloudHealthProvider {}
    }
}

#[async_trait]
impl HealthProvider for CloudHealthProvider {
    async fn ping(&self) -> Result<String, String> {
        // Mocking external HTTP/gRPC ping for now
        Ok("cloud_pong".to_string())
    }
}

pub struct StandaloneHealthProvider {
    // IPC ping (e.g. over local socket or just direct in-memory call if threaded)
}

impl StandaloneHealthProvider {
    pub fn new() -> Self {
        StandaloneHealthProvider {}
    }
}

#[async_trait]
impl HealthProvider for StandaloneHealthProvider {
    async fn ping(&self) -> Result<String, String> {
        Ok("standalone_pong".to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_cloud_ping() {
        let hp = CloudHealthProvider::new();
        assert_eq!(hp.ping().await.unwrap(), "cloud_pong");
    }

    #[tokio::test]
    async fn test_standalone_ping() {
        let hp = StandaloneHealthProvider::new();
        assert_eq!(hp.ping().await.unwrap(), "standalone_pong");
    }
}
