use uuid::Uuid;

pub struct InventoryItem {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub product_id: Uuid,
    pub sku: String,
    pub stock: i32,
}

impl InventoryItem {
    pub fn new(tenant_id: Uuid, product_id: Uuid, sku: String, stock: i32) -> Self {
        Self {
            id: Uuid::new_v4(),
            tenant_id,
            product_id,
            sku,
            stock,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_inventory_item() {
        let tenant_id = Uuid::new_v4();
        let product_id = Uuid::new_v4();
        let item = InventoryItem::new(tenant_id, product_id, "TEST-SKU".to_string(), 100);

        assert_eq!(item.stock, 100);
        assert_eq!(item.sku, "TEST-SKU");
    }
}
