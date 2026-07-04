use crate::domain::storefront::Storefront;

pub struct StorefrontService {}

impl StorefrontService {
    pub async fn create_draft(tenant_id: &str) -> Storefront {
        Storefront {
            tenant_id: tenant_id.to_string(),
            domain: format!("{}.ohc.com", tenant_id),
            active: false,
            html_content: "<html><body><h1>Draft Storefront</h1></body></html>".to_string(),
        }
    }

    pub async fn publish(mut storefront: Storefront) -> Storefront {
        storefront.active = true;
        // Mock CDN push and SSL provisioning
        storefront
    }
}
