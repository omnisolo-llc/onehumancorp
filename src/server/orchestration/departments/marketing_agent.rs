use crate::orchestration::departments::orchestrator::{BaseAgent, AgentTriggerType, DepartmentOrchestrator, Department};
use crate::orchestration::departments::types::{DepartmentType, DepartmentEvent, DepartmentConfig, ApprovalRequest, ActionRisk};
use std::sync::Arc;
use crate::orchestration::departments::marketing_seo::{SeoClient, RuntimeSeoClient};

#[async_trait::async_trait]
pub trait MarketingCopyClient: Send + Sync {
    async fn draft_caption(&self, prompt: &str, fallback: &str) -> String;
}

#[async_trait::async_trait]
pub trait MarketingImageOptimizer: Send + Sync {
    async fn optimize_product_image(&self, image_url: &str) -> Result<String, String>;
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum MarketingCopyBackend {
    Local,
    Minimax { api_key: String },
}

impl MarketingCopyBackend {
    pub fn from_env() -> Self {
        match std::env::var("OHC_LLM_PROVIDER").as_deref() {
            Ok("minimax") => {
                let api_key = std::env::var("MINIMAX_API_KEY").unwrap_or_default();
                if api_key.trim().is_empty() {
                    Self::Local
                } else {
                    Self::Minimax { api_key }
                }
            }
            _ => Self::Local,
        }
    }
}

struct RuntimeMarketingCopyClient {
    backend: MarketingCopyBackend,
}

impl RuntimeMarketingCopyClient {
    fn from_env() -> Self {
        Self {
            backend: MarketingCopyBackend::from_env(),
        }
    }
}

#[async_trait::async_trait]
impl MarketingCopyClient for RuntimeMarketingCopyClient {
    async fn draft_caption(&self, prompt: &str, fallback: &str) -> String {
        match &self.backend {
            MarketingCopyBackend::Minimax { api_key } => {
                crate::minimax::MinimaxClient::new(api_key.clone())
                    .reason(&crate::pricing::compression::reduce_tokens(prompt))
                    .await
                    .unwrap_or_else(|_| fallback.to_string())
            }
            MarketingCopyBackend::Local => crate::minimax::LocalLLMClient::new()
                .reason(&crate::pricing::compression::reduce_tokens(prompt))
                .await
                .unwrap_or_else(|_| fallback.to_string()),
        }
    }
}

struct RuntimeMarketingImageOptimizer {
    api_url: Option<String>,
    api_key: Option<String>,
}

impl RuntimeMarketingImageOptimizer {
    fn from_env() -> Self {
        Self {
            api_url: std::env::var("OHC_VISION_API_URL")
                .ok()
                .map(|value| value.trim().trim_end_matches('/').to_string())
                .filter(|value| !value.is_empty()),
            api_key: std::env::var("OHC_VISION_API_KEY")
                .ok()
                .map(|value| value.trim().to_string())
                .filter(|value| !value.is_empty()),
        }
    }
}

#[async_trait::async_trait]
impl MarketingImageOptimizer for RuntimeMarketingImageOptimizer {
    async fn optimize_product_image(&self, image_url: &str) -> Result<String, String> {
        let image_url = image_url.trim();
        if image_url.is_empty() {
            return Ok(String::new());
        }

        let Some(api_url) = self.api_url.as_deref() else {
            return Ok(image_url.to_string());
        };

        let mut request = reqwest::Client::new()
            .post(format!("{api_url}/optimize-product-image"))
            .json(&serde_json::json!({
                "image_url": image_url,
                "purpose": "marketing_product_post",
            }));

        if let Some(api_key) = self.api_key.as_deref() {
            request = request.bearer_auth(api_key);
        }

        let response = request
            .send()
            .await
            .map_err(|e| format!("Vision image optimization request failed: {e}"))?;
        let status = response.status();
        let body: serde_json::Value = response
            .json()
            .await
            .map_err(|e| format!("Vision image optimization response was not JSON: {e}"))?;

        if !status.is_success() {
            return Err(format!("Vision image optimization API error {status}: {body}"));
        }

        body.get("optimized_image_url")
            .or_else(|| body.get("cropped_image_url"))
            .or_else(|| body.get("image_url"))
            .and_then(|value| value.as_str())
            .map(str::trim)
            .filter(|value| !value.is_empty())
            .map(ToString::to_string)
            .ok_or_else(|| "Vision image optimization response missing optimized image URL".to_string())
    }
}

#[cfg(test)]
struct PassthroughImageOptimizer;

#[cfg(test)]
#[async_trait::async_trait]
impl MarketingImageOptimizer for PassthroughImageOptimizer {
    async fn optimize_product_image(&self, image_url: &str) -> Result<String, String> {
        Ok(image_url.to_string())
    }
}

pub struct MarketingAgent {
    orchestrator: Option<Arc<DepartmentOrchestrator>>,
    copy_client: Arc<dyn MarketingCopyClient>,
    image_optimizer: Arc<dyn MarketingImageOptimizer>,
    seo_client: Arc<dyn SeoClient>,
}

impl MarketingAgent {
    pub fn new(orchestrator: Arc<DepartmentOrchestrator>) -> Self {
        Self::new_with_clients(
            orchestrator,
            Arc::new(RuntimeMarketingCopyClient::from_env()),
            Arc::new(RuntimeMarketingImageOptimizer::from_env()),
            Arc::new(RuntimeSeoClient),
        )
    }

    pub fn new_with_copy_client(
        orchestrator: Arc<DepartmentOrchestrator>,
        copy_client: Arc<dyn MarketingCopyClient>,
    ) -> Self {
        Self::new_with_clients(
            orchestrator,
            copy_client,
            Arc::new(RuntimeMarketingImageOptimizer::from_env()),
            Arc::new(RuntimeSeoClient),
        )
    }

    pub fn new_with_clients(
        orchestrator: Arc<DepartmentOrchestrator>,
        copy_client: Arc<dyn MarketingCopyClient>,
        image_optimizer: Arc<dyn MarketingImageOptimizer>,
        seo_client: Arc<dyn SeoClient>,
    ) -> Self {
        Self {
            orchestrator: Some(orchestrator),
            copy_client,
            image_optimizer,
            seo_client,
        }
    }

    #[cfg(test)]
    fn new_for_test(copy_client: Arc<dyn MarketingCopyClient>, seo_client: Arc<dyn SeoClient>) -> Self {
        Self::new_for_test_with_optimizer(copy_client, Arc::new(PassthroughImageOptimizer), seo_client)
    }

    #[cfg(test)]
    fn new_for_test_with_optimizer(
        copy_client: Arc<dyn MarketingCopyClient>,
        image_optimizer: Arc<dyn MarketingImageOptimizer>,
        seo_client: Arc<dyn SeoClient>,
    ) -> Self {
        Self {
            orchestrator: None,
            copy_client,
            image_optimizer,
            seo_client,
        }
    }

    fn orchestrator(&self) -> Result<&Arc<DepartmentOrchestrator>, String> {
        self.orchestrator
            .as_ref()
            .ok_or_else(|| "MarketingAgent orchestrator is not configured".to_string())
    }

    pub async fn draft_product_caption(&self, product_name: &str, description: &str) -> String {
        let prompt = format!("Draft a short, engaging Instagram caption for a new or restocked product named '{}'. Description: '{}'. Keep it energetic and include 3 relevant hashtags.", product_name, description);
        let fallback = format!("Check out our new {}!", product_name);
        self.copy_client.draft_caption(&prompt, &fallback).await
    }

    pub async fn optimize_product_image_url(&self, image_url: &str) -> String {
        match self.image_optimizer.optimize_product_image(image_url).await {
            Ok(url) => url,
            Err(err) => {
                tracing::warn!("Marketing image optimization failed: {}", err);
                image_url.to_string()
            }
        }
    }
}

#[async_trait::async_trait]
impl Department for MarketingAgent {
    fn department_type(&self) -> DepartmentType {
        DepartmentType::Marketing
    }

    fn subscribed_events(&self) -> Vec<String> {
        vec![
            "tenant.insight.trending".to_string(),
            "tenant.product.created".to_string(),
            "tenant.job.completed".to_string(),
            "tenant.product.created".to_string(),
            "tenant.inventory.updated".to_string(),
            "tenant.website.updated".to_string(),
            "loyalty.points_awarded".to_string(),
        ]
    }

    async fn handle_event(&self, event: &DepartmentEvent) -> Result<(), String> {
        if event.event_type == "tenant.product.created" || event.event_type == "tenant.product.updated" {
            let product_id = event.payload.get("product_id").and_then(|v| v.as_str()).unwrap_or("");
            let name = event.payload.get("name").and_then(|v| v.as_str()).unwrap_or("New Product");
            let description = event.payload.get("description").and_then(|v| v.as_str()).unwrap_or("");
            let item_type = event.payload.get("item_type").and_then(|v| v.as_str()).unwrap_or("Product");
            let price = event.payload.get("price").and_then(|v| v.as_f64()).unwrap_or(0.0);

            if !product_id.is_empty() {
                if let Ok((seo_title, seo_desc, seo_schema)) = self.seo_client.generate_seo_metadata(name, description, item_type, price).await {
                    if let Ok(orchestrator) = self.orchestrator() {
                        let pool = orchestrator.db().pool.clone();
                        let tenant_id_str = event.tenant_id.clone();
                        let product_id_str = product_id.to_string();

                        // Spawn a task to update DB, invalidate cache, and enqueue publish job
                        let seo_schema_clone = seo_schema.clone();
                        tokio::spawn(async move {
                            if let Ok(tenant_id) = uuid::Uuid::parse_str(&tenant_id_str) {
                                // Update DB
                                let _ = sqlx::query("UPDATE products SET seo_title = $1, seo_description = $2, seo_schema_json = $3 WHERE tenant_id = $4 AND id = $5")
                                    .bind(seo_title)
                                    .bind(seo_desc)
                                    .bind(seo_schema_clone)
                                    .bind(tenant_id_str.clone())
                                    .bind(product_id_str.clone())
                                    .execute(&pool)
                                    .await;

                                // Invalidate cache
                                let cache = crate::builder::edge::get_edge_cache();
                                cache.invalidate_by_tag(&format!("tenant-id:{}", tenant_id_str)).await;
                                cache.invalidate_by_tag(&format!("entity:product:{}", product_id_str)).await;
                                let cdn_cache = crate::utils::edge_caching_middleware::get_cdn_cache();
                                cdn_cache.invalidate_by_tag(&format!("tenant-id:{}", tenant_id_str)).await;
                                cdn_cache.invalidate_by_tag(&format!("entity:product:{}", product_id_str)).await;


                                let cdn = crate::utils::edge_caching_middleware::get_cdn_cache();
                                cdn.invalidate_by_tag(&format!("tenant-id:{}", tenant_id_str)).await;
                                cdn.invalidate_by_tag(&format!("entity:product:{}", product_id_str)).await;

                                // Proactively pre-render the product cache
                                if let Ok(product_uuid) = uuid::Uuid::parse_str(&product_id_str) {
                                    let cache_key = format!("storefront:product:{}:{}", tenant_id, product_uuid);
                                    let _ = crate::builder::edge::regenerate_product_cache(pool.clone(), tenant_id, product_uuid, cache_key, cache.clone()).await;
                                }



                                // Trigger site publish job for all sites for the tenant
                                if let Ok(sites) = crate::builder::db::list_sites(&pool, tenant_id).await {
                                    for site in sites {
                                        let _ = crate::builder::jobs::enqueue_publish_site_job(&pool, tenant_id, site.id).await;
                                    }
                                }
                            }
                        });
                    }
                }
            }
        }

        if event.event_type == "tenant.website.updated" || event.event_type == "tenant.product.created" || event.event_type == "tenant.product.updated" {
            let site_id = event.payload.get("site_id").and_then(|v| v.as_str()).unwrap_or("unknown");
            let payload = serde_json::json!({
                "site_id": site_id,
            });

            if let Ok(orchestrator) = self.orchestrator() {
                let db = orchestrator.db();
                let pool = db.pool.clone();
                let tenant_id_str = event.tenant_id.clone();
                let site_id_str = site_id.to_string();

                tokio::spawn(async move {
                    if let Ok(tenant_id) = uuid::Uuid::parse_str(&tenant_id_str) {
                        if let Ok(s_id) = uuid::Uuid::parse_str(&site_id_str) {
                            let _ = crate::builder::jobs::enqueue_publish_site_job(&pool, tenant_id, s_id).await;
                        } else {
                            if let Ok(sites) = crate::builder::db::list_sites(&pool, tenant_id).await {
                                for site in sites {
                                    let _ = crate::builder::jobs::enqueue_publish_site_job(&pool, tenant_id, site.id).await;
                                }
                            }
                        }
                    }
                });
            }

            return self.orchestrator()?.execute_action(
                DepartmentType::Marketing,
                "Trigger Agentic SEO Pre-rendering".to_string(),
                event.tenant_id.clone(),
                ActionRisk::AutoExecute,
                payload,
            ).await.map(|_| ());
        }

        let risk = ActionRisk::DraftForReview;


        if event.event_type == "tenant.job.completed" {
            let service_name = event.payload.get("service_name").and_then(|v| v.as_str()).unwrap_or("Service");
            let media = event.payload.get("media").and_then(|v| v.as_array());

            if let Some(media_array) = media {
                if !media_array.is_empty() {
                    let media_url = media_array[0].as_str().unwrap_or("");

                    let draft_copy = format!("Beautiful new {} completed recently. Completed on time and on budget.", service_name.to_lowercase());

                    let payload = serde_json::json!({
                        "feature_type": "case_study",
                        "service_name": service_name,
                        "media_url": media_url,
                        "draft_copy": draft_copy
                    });

                    let description = format!("Draft portfolio case study for {}", service_name);

                    return self.orchestrator()?.execute_action(
                        DepartmentType::Marketing,
                        description,
                        event.tenant_id.clone(),
                        risk,
                        payload,
                    ).await.map(|_| ());
                }
            }
        }

        if event.event_type == "tenant.inventory.updated" {
            let product_name = event.payload.get("name").and_then(|v| v.as_str()).unwrap_or("New Product");
            let description = event.payload.get("description").and_then(|v| v.as_str()).unwrap_or("");
            let images = event.payload.get("images").and_then(|v| v.as_array());

            let image_url = if let Some(imgs) = images {
                if !imgs.is_empty() {
                    imgs[0].as_str().unwrap_or("")
                } else {
                    ""
                }
            } else {
                ""
            };

            let optimized_image_url = self.optimize_product_image_url(image_url).await;

            let draft_copy = self.draft_product_caption(product_name, description).await;

            let payload = serde_json::json!({
                "feature_type": "social_post",
                "product_name": product_name,
                "image_url": optimized_image_url,
                "draft_copy": draft_copy
            });

            let action_desc = format!("Draft Instagram post for {}", product_name);
            return self.orchestrator()?.execute_action(DepartmentType::Marketing, action_desc, event.tenant_id.clone(), risk, payload).await.map(|_| ());
        }

        if event.event_type == "loyalty.points_awarded" {
            let customer_id = event.payload.get("customer_id").and_then(|v| v.as_str()).unwrap_or("Customer");
            let points = event.payload.get("points").and_then(|v| v.as_i64()).unwrap_or(0);
            let total_points = event.payload.get("total_points").and_then(|v| v.as_i64()).unwrap_or(0);

            // Mock threshold logic: check if total points reached a certain tier (e.g. 50, 100)
            if total_points >= 50 && total_points - points < 50 {
                let draft_copy = format!("Hey {}! You just earned a free coffee (or equivalent reward)! Reply 'Claim' to use it on your next pre-order.", customer_id);

                let payload = serde_json::json!({
                    "feature_type": "loyalty_reward_notification",
                    "customer_id": customer_id,
                    "total_points": total_points,
                    "draft_copy": draft_copy,
                    "channel": "sms_or_dm"
                });

                let description = format!("Send reward notification to customer {}", customer_id);

                // Auto execute the sending of the notification for zero-friction loyalty
                return self.orchestrator()?.execute_action(
                    DepartmentType::Marketing,
                    description,
                    event.tenant_id.clone(),
                    ActionRisk::AutoExecute,
                    payload,
                ).await.map(|_| ());
            }

            return Ok(());
        }

        self.orchestrator()?.execute_action(
            DepartmentType::Marketing,
            "Draft social media campaign for trending item".to_string(),
            event.tenant_id.clone(),
            risk,
            event.payload.clone(),
        ).await.map(|_| ())
    }

    fn get_config(&self, _tenant_id: &str) -> Option<DepartmentConfig> {
        None
    }


    async fn query_memory(&self, _query: &str) -> Result<Vec<String>, String> {
        Ok(vec![])
    }

    async fn request_approval(&self, description: String, tenant_id: String, risk: ActionRisk) -> Result<ApprovalRequest, String> {
        self.orchestrator()?.execute_action(self.department_type(), description.clone(), tenant_id.clone(), risk, serde_json::json!({})).await
    }
}

#[async_trait::async_trait]
impl BaseAgent for MarketingAgent {
    fn agent_id(&self) -> String {
        "marketing_agent".to_string()
    }

    fn trigger_type(&self) -> AgentTriggerType {
        AgentTriggerType::EventDriven
    }

}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;
use crate::orchestration::departments::marketing_seo::{SeoClient, RuntimeSeoClient};

    struct FixedCopyClient;
    struct FixedImageOptimizer;

    #[async_trait::async_trait]
    impl MarketingCopyClient for FixedCopyClient {
        async fn draft_caption(&self, _prompt: &str, _fallback: &str) -> String {
            "Injected caption from test client".to_string()
        }
    }

    #[async_trait::async_trait]
    impl MarketingImageOptimizer for FixedImageOptimizer {
        async fn optimize_product_image(&self, image_url: &str) -> Result<String, String> {
            Ok(format!("{image_url}?vision=cropped"))
        }
    }

    #[tokio::test]
    async fn marketing_agent_uses_injected_copy_client_for_product_captions() {
        let agent = MarketingAgent::new_for_test(Arc::new(FixedCopyClient), Arc::new(RuntimeSeoClient));

        let caption = agent
            .draft_product_caption("Ceramic Mug", "Handmade stoneware")
            .await;

        assert_eq!(caption, "Injected caption from test client");
    }

    struct MockSeoClient;
    #[async_trait::async_trait]
    impl SeoClient for MockSeoClient {
        async fn generate_seo_metadata(&self, name: &str, description: &str, _item_type: &str, _price: f64) -> Result<(String, String, serde_json::Value), String> {
            Ok((
                format!("Mock SEO Title for {}", name),
                format!("Mock SEO Desc for {}", description),
                serde_json::json!({"mock": true})
            ))
        }
    }

    #[tokio::test]
    async fn marketing_agent_uses_injected_seo_client() {
        let agent = MarketingAgent::new_for_test(Arc::new(FixedCopyClient), Arc::new(MockSeoClient));

        // Test SEO client exists and can be called directly
        let res = agent.seo_client.generate_seo_metadata("Cake", "Tasty cake", "Product", 10.0).await.unwrap();
        assert_eq!(res.0, "Mock SEO Title for Cake");
    }

    #[tokio::test]
    async fn marketing_agent_uses_injected_vision_optimizer_for_product_images() {
        let agent = MarketingAgent::new_for_test_with_optimizer(
            Arc::new(FixedCopyClient),
            Arc::new(FixedImageOptimizer),
            Arc::new(RuntimeSeoClient),
        );

        let optimized = agent
            .optimize_product_image_url("https://cdn.example.test/mug.jpg")
            .await;

        assert_eq!(optimized, "https://cdn.example.test/mug.jpg?vision=cropped");
    }

    #[test]
    fn marketing_copy_backend_falls_back_to_local_without_minimax_key() {
        let old_provider = std::env::var("OHC_LLM_PROVIDER").ok();
        let old_key = std::env::var("MINIMAX_API_KEY").ok();

        unsafe {
            std::env::set_var("OHC_LLM_PROVIDER", "minimax");
            std::env::remove_var("MINIMAX_API_KEY");
        }

        assert_eq!(MarketingCopyBackend::from_env(), MarketingCopyBackend::Local);

        unsafe {
            match old_provider {
                Some(value) => std::env::set_var("OHC_LLM_PROVIDER", value),
                None => std::env::remove_var("OHC_LLM_PROVIDER"),
            }
            match old_key {
                Some(value) => std::env::set_var("MINIMAX_API_KEY", value),
                None => std::env::remove_var("MINIMAX_API_KEY"),
            }
        }
    }

    #[test]
    fn marketing_copy_backend_captures_minimax_key_at_construction() {
        let old_provider = std::env::var("OHC_LLM_PROVIDER").ok();
        let old_key = std::env::var("MINIMAX_API_KEY").ok();

        unsafe {
            std::env::set_var("OHC_LLM_PROVIDER", "minimax");
            std::env::set_var("MINIMAX_API_KEY", "configured-key");
        }

        assert_eq!(
            MarketingCopyBackend::from_env(),
            MarketingCopyBackend::Minimax {
                api_key: "configured-key".to_string()
            }
        );

        unsafe {
            match old_provider {
                Some(value) => std::env::set_var("OHC_LLM_PROVIDER", value),
                None => std::env::remove_var("OHC_LLM_PROVIDER"),
            }
            match old_key {
                Some(value) => std::env::set_var("MINIMAX_API_KEY", value),
                None => std::env::remove_var("MINIMAX_API_KEY"),
            }
        }
    }
}
