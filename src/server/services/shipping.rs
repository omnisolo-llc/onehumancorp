use std::sync::Arc;
use crate::integrations::registry::IntegrationsRegistry;
use uuid::Uuid;

pub struct ShippingService {
    registry: Arc<IntegrationsRegistry>,
}

impl ShippingService {
    pub fn new(registry: Arc<IntegrationsRegistry>) -> Self {
        Self { registry }
    }

    pub async fn get_fulfillment_options(&self, tenant_id: &str, _order_id: &str) -> Result<Vec<crate::integrations::shippo::client::ShippingRate>, String> {
        let instances = self.registry.instances_by_category(tenant_id, "shipping");
        if let Some(_inst) = instances.iter().find(|i| i.id == "shippo" && i.status == "connected") {
            // In a real impl, we would use the provider to fetch real rates
            Ok(vec![
                crate::integrations::shippo::client::ShippingRate {
                    id: format!("rate_{}", Uuid::new_v4()),
                    amount: "5.50".to_string(),
                    provider: "USPS".to_string(),
                }
            ])
        } else {
            Err("Shipping provider not connected".to_string())
        }
    }
}
