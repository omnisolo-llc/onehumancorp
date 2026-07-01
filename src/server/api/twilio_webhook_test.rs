use crate::api::twilio_webhook::TwilioProvider;
use server_domain::routing::LocalTeammateMesh;
use std::sync::Arc;

#[test]
fn test_twilio_provider_init() {
    let _provider = TwilioProvider::new(
        "test_sid".to_string(),
        "test_token".to_string(),
        Arc::new(LocalTeammateMesh::default()),
    );
    assert!(true);
}
