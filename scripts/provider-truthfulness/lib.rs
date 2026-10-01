//! Exact production files with a negative-only registry effect sentinel.
//! No provider calls, real credentials, or success substitutions are used.
#![allow(dead_code)]
extern crate self as server_omnisolo;
#[path = "../../src/server/api/integrations_settings.rs"]
pub mod integrations_settings;
#[path = "../../src/server/integrations/twilio/client.rs"]
pub mod twilio_client;

pub mod orchestration {
    // The mounted handler's transport DTO; unrelated application modules are
    // deliberately absent from this negative-effect harness.
    pub struct ConnectIntegrationRequest {
        pub integration_id: String,
        pub base_url: String,
        pub bot_token: String,
        pub chat_id: String,
        pub webhook_url: String,
        pub api_token: String,
        pub from_phone: String,
    }
}
pub mod integrations {
    pub mod registry {
        use std::sync::atomic::{AtomicUsize, Ordering};
        #[derive(Default)]
        pub struct IntegrationsRegistry {
            pub calls: AtomicUsize,
        }
        impl IntegrationsRegistry {
            pub fn connect(
                &self,
                _: &str,
                _: &str,
                _: crate::orchestration::ConnectIntegrationRequest,
            ) -> Result<(), String> {
                self.calls.fetch_add(1, Ordering::SeqCst);
                Err("No provider verification exists in this boundary".into())
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};
    use std::sync::{Arc, atomic::Ordering};
    use twilio_client::TwilioClientWrapper;

    async fn assert_unconfigured(sid: &str) {
        let client = twilio_client::RealTwilioClient::new(sid.into(), String::new());
        let outcome = tokio::time::timeout(
            std::time::Duration::from_millis(250),
            client.provision_number("415"),
        )
        .await
        .expect("unconfigured provisioning must reject before network I/O");
        assert!(
            outcome.is_err(),
            "unconfigured client invented a phone number: {outcome:?}"
        );
    }
    #[tokio::test]
    async fn missing_credentials_never_invent_a_phone_number() {
        assert_unconfigured("").await;
    }
    #[tokio::test]
    async fn test_marker_never_invents_a_phone_number() {
        assert_unconfigured("test").await;
    }
    #[tokio::test]
    async fn dummy_marker_never_invents_a_phone_number() {
        assert_unconfigured("dummy").await;
    }

    #[tokio::test]
    async fn cloud_whatsapp_rejects_unverified_registry_effects() {
        let registry = Arc::new(integrations::registry::IntegrationsRegistry::default());
        let response = integrations_settings::connect_whatsapp_cloud_api(
            State(registry.clone()),
            Json(integrations_settings::ConnectWhatsAppCloudApiReq {
                api_token: Some("unverified-local-input".into()),
                phone_number_id: Some("local-phone-id".into()),
                display_phone_number: None,
            }),
        )
        .await
        .into_response();
        assert_eq!(
            registry.calls.load(Ordering::SeqCst),
            0,
            "unverified configuration must not create a connected registry entry"
        );
        assert_eq!(response.status(), StatusCode::NOT_IMPLEMENTED);
        let value: serde_json::Value = serde_json::from_slice(
            &axum::body::to_bytes(response.into_body(), 4096)
                .await
                .unwrap(),
        )
        .unwrap();
        assert_eq!(value["success"], false);
        assert_eq!(value["usable"], false);
        assert_ne!(value["status"], "connected");
    }
    #[tokio::test]
    async fn whatsapp_settings_reject_unverified_registry_effects() {
        let registry = Arc::new(integrations::registry::IntegrationsRegistry::default());
        let response = integrations_settings::connect_whatsapp(
            State(registry.clone()),
            Json(integrations_settings::ConnectWhatsAppReq {
                bot_token: None,
                api_token: Some("unverified-local-input".into()),
                from_phone: None,
                integration_id: None,
                base_url: None,
            }),
        )
        .await
        .into_response();
        assert_eq!(registry.calls.load(Ordering::SeqCst), 0);
        assert_eq!(response.status(), StatusCode::NOT_IMPLEMENTED);
        let value: serde_json::Value = serde_json::from_slice(
            &axum::body::to_bytes(response.into_body(), 4096)
                .await
                .unwrap(),
        )
        .unwrap();
        assert_eq!(value["success"], false);
        assert_eq!(value["usable"], false);
    }
}
