// Compile the maintained implementation directly, without a second copy.
extern crate self as omnisolo_builtin_agent;
extern crate self as omnisolo_builtin_agent_core;
#[path = "../../../src/agents/builtin/types.rs"]
pub mod types;
include!("generated-tool-base.rs");
#[path = "../../../src/agents/builtin/tools/marketplace.rs"]
pub mod marketplace;
#[path = "../../../src/agents/builtin/tools/marketplace_tool.rs"]
pub mod marketplace_tool;
#[path = "../../../src/agents/builtin/tools/pydantic.rs"]
pub mod pydantic;
pub mod tools {
    pub use crate::marketplace;
}
#[cfg(test)]
mod generated;

#[cfg(test)]
mod configuration_tests {
    use super::generated::{MarketplaceQuery, configured_app_marketplace, list_marketplace_agents};
    use axum::{extract::Query, response::IntoResponse};
    static ENVIRONMENT: tokio::sync::Mutex<()> = tokio::sync::Mutex::const_new(());

    // Only these serialized tests mutate this dedicated configuration variable.
    struct Restore(Option<std::ffi::OsString>);
    impl Drop for Restore {
        fn drop(&mut self) {
            unsafe {
                match self.0.take() {
                    Some(value) => std::env::set_var("AGENT_MARKETPLACE_URL", value),
                    None => std::env::remove_var("AGENT_MARKETPLACE_URL"),
                }
            }
        }
    }
    #[tokio::test]
    async fn unconfigured_native_clients_never_fabricate_a_catalog() {
        let _serial = ENVIRONMENT.lock().await;
        let _restore = Restore(std::env::var_os("AGENT_MARKETPLACE_URL"));
        for value in ["", "https://marketplace.example.com", "not a registry URL"] {
            unsafe {
                std::env::set_var("AGENT_MARKETPLACE_URL", value);
            }
            assert!(
                configured_app_marketplace().search("").await.is_err(),
                "unconfigured value {value} must not return a mock catalog"
            );
        }
    }
    #[tokio::test]
    async fn unconfigured_mounted_marketplace_listing_returns_unavailable() {
        let _serial = ENVIRONMENT.lock().await;
        let _restore = Restore(std::env::var_os("AGENT_MARKETPLACE_URL"));
        unsafe {
            std::env::remove_var("AGENT_MARKETPLACE_URL");
        }
        let response = list_marketplace_agents(Query(MarketplaceQuery { q: Some("".into()) }))
            .await
            .into_response();
        assert_eq!(response.status(), 503);
        let body = axum::body::to_bytes(response.into_body(), 4096)
            .await
            .unwrap();
        let value: serde_json::Value = serde_json::from_slice(&body).unwrap();
        assert!(value.get("error").is_some());
        assert!(!value.is_array());
    }
}

#[cfg(test)]
#[path = "generated-publication.rs"]
mod publication;
#[cfg(test)]
mod publication_tests {
    use super::{
        marketplace::{MarketplaceAgent, MarketplaceClient, MarketplaceProvider},
        publication::{JsonRpcRequest, PublicationCall},
    };
    use std::sync::{Arc, RwLock};
    struct Registry(Arc<RwLock<Vec<MarketplaceAgent>>>);
    #[async_trait::async_trait]
    impl MarketplaceProvider for Registry {
        async fn search(&self, _: &str) -> Result<Vec<MarketplaceAgent>, String> {
            Ok(self.0.read().unwrap().clone())
        }
        async fn fetch_agent(&self, _: &str) -> Result<MarketplaceAgent, String> {
            Err("not found".into())
        }
        async fn publish_agent(
            &self,
            mut agent: MarketplaceAgent,
        ) -> Result<MarketplaceAgent, String> {
            if agent.id.is_empty() {
                agent.id = "provider-id".into();
            }
            self.0.write().unwrap().push(agent.clone());
            Ok(agent)
        }
    }
    fn descriptor() -> serde_json::Value {
        serde_json::json!({ "name":"Reviewed agent", "description":"All fields", "author":"Reviewed author", "version":"2.3.4", "endpoint":"https://registry.example.test/definitions/reviewed" })
    }
    fn request(params: serde_json::Value) -> JsonRpcRequest {
        serde_json::from_value(serde_json::json!({ "jsonrpc":"2.0", "id":"original-rpc-id", "method":"am_publish_agent", "params":params })).unwrap()
    }
    #[tokio::test]
    async fn rpc_publication_preserves_reviewed_descriptor_fields_and_correlation() {
        let recorded = Arc::new(RwLock::new(Vec::new()));
        let call = PublicationCall {
            marketplace: Arc::new(MarketplaceClient::new(Box::new(Registry(recorded.clone())))),
        };
        let reply: serde_json::Value =
            serde_json::from_str(&call.publish(request(descriptor())).await).unwrap();
        assert_eq!(reply["id"], "original-rpc-id");
        assert!(reply.get("error").is_none());
        for key in ["name", "description", "author", "version", "endpoint"] {
            assert_eq!(reply["result"][key], descriptor()[key]);
        }
        assert_eq!(reply["result"]["id"], "provider-id");
        assert_eq!(recorded.read().unwrap().len(), 1);
    }
    #[tokio::test]
    async fn rpc_publication_rejects_full_agent_and_unknown_fields_before_provider_effects() {
        for params in [
            serde_json::json!({ "name":"Private", "description":"Draft", "role":"Writer", "system_prompt":"Private prompt" }),
            {
                let mut params = descriptor();
                params["system_prompt"] = serde_json::json!("Must never disappear");
                params
            },
            serde_json::json!({ "name":"Incomplete" }),
        ] {
            let recorded = Arc::new(RwLock::new(Vec::new()));
            let call = PublicationCall {
                marketplace: Arc::new(MarketplaceClient::new(Box::new(Registry(recorded.clone())))),
            };
            let reply: serde_json::Value =
                serde_json::from_str(&call.publish(request(params)).await).unwrap();
            assert_eq!(reply["id"], "original-rpc-id");
            assert_eq!(reply["error"]["code"], -32602);
            assert!(reply.get("result").is_none());
            assert!(recorded.read().unwrap().is_empty());
        }
    }
}
