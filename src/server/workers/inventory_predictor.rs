use uuid::Uuid;

pub struct InventoryPredictor {}

impl InventoryPredictor {
    pub fn new() -> Self {
        Self {}
    }

    pub fn predict_stockout(&self, tenant_id: Uuid, product_id: Uuid, current_stock: i32, daily_sales_velocity: f32) -> Option<u32> {
        if daily_sales_velocity <= 0.0 {
            return None;
        }
        let days_until_stockout = (current_stock as f32 / daily_sales_velocity).ceil() as u32;
        if days_until_stockout <= 5 {
             return Some(days_until_stockout); // Alert if stockout within 5 days
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_predict_stockout() {
        let predictor = InventoryPredictor::new();
        let tenant_id = Uuid::new_v4();
        let product_id = Uuid::new_v4();

        // Predict stockout in 2 days
        let days = predictor.predict_stockout(tenant_id, product_id, 10, 5.0);
        assert_eq!(days, Some(2));

        // No stockout soon
        let days = predictor.predict_stockout(tenant_id, product_id, 100, 1.0);
        assert_eq!(days, None);
    }
}
