use super::*;

    #[tokio::test]
    async fn test_ipc_lock() {
        let db_url = "sqlite::memory:";
        let bus = IpcBus::new(db_url).await.unwrap();

        let acquired1 = bus.acquire_lock("test_res", "owner1", 10).await.unwrap();
        assert!(acquired1);

        let acquired2 = bus.acquire_lock("test_res", "owner2", 10).await.unwrap();
        assert!(!acquired2);

        bus.release_lock("test_res", "owner1").await.unwrap();

        let acquired3 = bus.acquire_lock("test_res", "owner2", 10).await.unwrap();
        assert!(acquired3);
    }
