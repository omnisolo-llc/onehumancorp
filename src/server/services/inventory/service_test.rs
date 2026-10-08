use super::*;
use std::sync::Arc;

#[tokio::test]
async fn test_redlock_key_consistency() {
    let locker = StandaloneInventoryLocker::new();
    let service = InventoryService {
        locker: Box::new(locker),
        redis_client: None,
    };

    let tenant_id = "tenant_xyz";
    let product_id = "prod_123";
    let expected_key = format!("ohc:lock:{}:inventory:{}", tenant_id, product_id);

    assert_eq!(InventoryService::get_lock_key(tenant_id, product_id), expected_key);
}
