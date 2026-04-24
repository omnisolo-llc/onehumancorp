use super::*;
use std::sync::Arc;
use tokio::sync::broadcast;
use async_trait::async_trait;

struct MockTransport {
    tx: broadcast::Sender<Vec<u8>>,
    rx: broadcast::Receiver<Vec<u8>>,
}

impl MockTransport {
    fn new() -> Self {
        let (tx, rx) = broadcast::channel(100);
        Self { tx, rx }
    }
}

impl mesh::MeshTransport for MockTransport {
    fn publish(&self, _topic: &str, data: &[u8]) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let _ = self.tx.send(data.to_vec());
        Ok(())
    }

    fn subscribe(&self, _topic: &str) -> Result<broadcast::Receiver<Vec<u8>>, Box<dyn std::error::Error + Send + Sync>> {
        Ok(self.tx.subscribe())
    }
}

#[tokio::test]
async fn test_mesh_interop() {
    let transport = Arc::new(MockTransport::new());
    let interop = mesh::MeshInterop::new(transport);

    let mut status_rx = interop.subscribe_job_status().unwrap();

    let status = mesh::mesh_pb::MeshJobStatus {
        job_id: "test-1".to_string(),
        status: "RUNNING".to_string(),
        result: "".to_string(),
    };

    let mut buf = Vec::new();
    prost::Message::encode(&status, &mut buf).unwrap();
    interop.dispatch_job(&mesh::mesh_pb::MeshJobDispatch {
        job_id: "test-1".to_string(),
        tenant_id: "tenant-1".to_string(),
        payload: "{}".to_string(),
    }).unwrap();
}
