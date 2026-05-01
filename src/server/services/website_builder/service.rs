use tonic::{Request, Response, Status};
use crate::ohc::orchestration::*;
use crate::ohc::orchestration::website_builder_service_server::WebsiteBuilderService;
use std::sync::Arc;
use crate::hub::Hub;

pub struct MyWebsiteBuilderService {
    hub: Arc<Hub>,
}

impl MyWebsiteBuilderService {
    pub fn new(hub: Arc<Hub>) -> Self {
        MyWebsiteBuilderService { hub }
    }
}

#[tonic::async_trait]
impl WebsiteBuilderService for MyWebsiteBuilderService {
    async fn publish_site(
        &self,
        request: Request<PublishSiteRequest>,
    ) -> Result<Response<PublishSiteResponse>, Status> {
        let md = request.metadata().clone();
        let org_id = md.get("organization_id")
            .and_then(|v| v.to_str().ok())
            .unwrap_or("unknown_org")
            .to_string();

        let req = request.into_inner();

        let url = if req.domain_choice == "custom" {
            let suffix = uuid::Uuid::new_v4().to_string().chars().take(8).collect::<String>();
            format!("https://{}-{}.ohc.store", req.product_name.to_lowercase().replace(" ", "-"), suffix)
        } else {
            "https://my-awesome-site.ohc.store".to_string()
        };

        let pool = self.hub.get_pool();
        let query = "
            INSERT INTO website_configurations (organization_id, template, primary_color, domain_choice, url)
            VALUES ($1, $2, $3, $4, $5)
            ON CONFLICT (organization_id) DO UPDATE SET
                template = EXCLUDED.template,
                primary_color = EXCLUDED.primary_color,
                domain_choice = EXCLUDED.domain_choice,
                url = EXCLUDED.url,
                updated_at = CURRENT_TIMESTAMP
        ";
        let _ = sqlx::query(query)
            .bind(&org_id)
            .bind(&req.template)
            .bind(&req.primary_color)
            .bind(&req.domain_choice)
            .bind(&url)
            .execute(&pool)
            .await;

        let payload = serde_json::json!({
            "action": "publish_website",
            "template": req.template,
            "primary_color": req.primary_color,
            "product_name": req.product_name,
            "domain_choice": req.domain_choice,
            "target_url": url.clone(),
        });

        // Trigger Promoter agent
        let task = crate::ohc::orchestration::Message {
            id: format!("msg-{}", uuid::Uuid::new_v4()),
            from_agent: "SYSTEM".to_string(),
            to_agent: "Promoter".to_string(),
            r#type: "task".to_string(),
            content: payload.to_string(),
            meeting_id: "".to_string(),
            occurred_at_unix: chrono::Utc::now().timestamp(),
        };

        let hub_arc = self.hub.clone();
        let _ = hub_arc.delegate_task("SYSTEM".to_string(), "Promoter".to_string(), task);

        let response = PublishSiteResponse {
            url,
            status: "published".to_string(),
        };

        Ok(Response::new(response))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tonic::Request;

    #[tokio::test]
    async fn test_publish_site() {
        let (tx, _rx) = tokio::sync::mpsc::channel(100);
        let pool = sqlx::postgres::PgPoolOptions::new()
            .connect("postgres://postgres:postgres@localhost:5432/ohc")
            .await;

        // Skip test if DB is not available
        if pool.is_err() {
            return;
        }

        let hub = Arc::new(Hub::new(tx, pool.unwrap()));
        let service = MyWebsiteBuilderService::new(hub);

        let request = PublishSiteRequest {
            template: "E-commerce".to_string(),
            primary_color: "#34C759".to_string(),
            product_name: "My Custom Product".to_string(),
            product_price: "19.99".to_string(),
            product_description: "A great custom product.".to_string(),
            domain_choice: "custom".to_string(),
        };

        let response = service.publish_site(Request::new(request)).await.unwrap().into_inner();

        assert_eq!(response.url, "https://my-custom-product.ohc.store");
        assert_eq!(response.status, "published");
    }
}
