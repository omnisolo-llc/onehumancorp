use prost::Message;
use std::sync::Arc;
use tokio::sync::broadcast;

// Import the generated protobuf messages
pub mod mesh_pb {
    #![allow(clippy::all)]
    include!("gen/ohc.mesh.rs");
}

pub trait MeshTransport: Send + Sync {
    fn publish(&self, topic: &str, data: &[u8]) -> Result<(), Box<dyn std::error::Error + Send + Sync>>;
    fn subscribe(&self, topic: &str) -> Result<broadcast::Receiver<Vec<u8>>, Box<dyn std::error::Error + Send + Sync>>;
}

pub struct MeshInterop {
    transport: Arc<dyn MeshTransport>,
}

impl MeshInterop {
    pub fn new(transport: Arc<dyn MeshTransport>) -> Self {
        Self { transport }
    }

    pub fn dispatch_job(&self, dispatch: &mesh_pb::MeshJobDispatch) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let mut buf = Vec::new();
        dispatch.encode(&mut buf)?;
        self.transport.publish("mesh:jobs:dispatch", &buf)
    }

    pub fn subscribe_job_status(&self) -> Result<broadcast::Receiver<mesh_pb::MeshJobStatus>, Box<dyn std::error::Error + Send + Sync>> {
        let mut raw_rx = self.transport.subscribe("mesh:jobs:status")?;
        let (tx, rx) = broadcast::channel(100);

        tokio::spawn(async move {
            while let Ok(data) = raw_rx.recv().await {
                if let Ok(status) = mesh_pb::MeshJobStatus::decode(&data[..]) {
                    let _ = tx.send(status);
                }
            }
        });

        Ok(rx)
    }

    pub fn sync_context(&self, sync: &mesh_pb::MeshContextSync) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let mut buf = Vec::new();
        sync.encode(&mut buf)?;
        self.transport.publish("mesh:context:sync", &buf)
    }

    pub fn handoff_state(&self, handoff: &mesh_pb::MeshHandoff) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let mut buf = Vec::new();
        handoff.encode(&mut buf)?;
        self.transport.publish("mesh:state:handoff", &buf)
    }

    pub fn subscribe_context_sync(&self) -> Result<broadcast::Receiver<mesh_pb::MeshContextSync>, Box<dyn std::error::Error + Send + Sync>> {
        let mut raw_rx = self.transport.subscribe("mesh:context:sync")?;
        let (tx, rx) = broadcast::channel(100);

        tokio::spawn(async move {
            while let Ok(data) = raw_rx.recv().await {
                if let Ok(sync) = mesh_pb::MeshContextSync::decode(&data[..]) {
                    let _ = tx.send(sync);
                }
            }
        });

        Ok(rx)
    }

    pub fn subscribe_handoff(&self) -> Result<broadcast::Receiver<mesh_pb::MeshHandoff>, Box<dyn std::error::Error + Send + Sync>> {
        let mut raw_rx = self.transport.subscribe("mesh:state:handoff")?;
        let (tx, rx) = broadcast::channel(100);

        tokio::spawn(async move {
            while let Ok(data) = raw_rx.recv().await {
                if let Ok(handoff) = mesh_pb::MeshHandoff::decode(&data[..]) {
                    let _ = tx.send(handoff);
                }
            }
        });

        Ok(rx)
    }
}
