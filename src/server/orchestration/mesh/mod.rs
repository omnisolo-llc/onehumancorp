pub mod api;
pub mod transport;

#[cfg(test)]
mod tests {
    use super::transport::{MeshTransport, MemoryMeshTransport};

    #[test]
    fn test_memory_mesh_transport() {
        let rt = tokio::runtime::Builder::new_current_thread().enable_all().build().unwrap();
        rt.block_on(async {
            let transport = MemoryMeshTransport::new();

            let mut rx1 = transport.subscribe("mesh:tasks").await.unwrap();
            let mut rx2 = transport.subscribe("mesh:tasks").await.unwrap();

            let data = b"hello world";

            transport.publish("mesh:tasks", "TASK_TRANSITION", data).await.unwrap();

            let msg1 = rx1.recv().await.unwrap();
            let msg2 = rx2.recv().await.unwrap();

            assert_eq!(msg1, b"hello world".to_vec());
            assert_eq!(msg2, b"hello world".to_vec());
        });
    }
}
