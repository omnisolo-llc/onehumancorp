use ohc_agent_startup_auth_contract::{actual_startup, auth};

fn main() {
    let standalone = std::env::var("PROBE_STANDALONE").as_deref() == Ok("true");
    let before = (
        std::env::var("OMNISOLO_AGENT_TOKEN").ok(),
        std::env::var("OMNISOLO_AGENT_AUTH_KEY").ok(),
    );
    let result = actual_startup(standalone);
    let configured_token = matches!(result, Ok(Some(auth::AuthMode::Token { .. })));
    let accepted_fixture = match &result {
        Ok(Some(auth::AuthMode::Token {
            token_hash,
            verification_key,
        })) => auth::check_token("explicit-test-fixture-only", token_hash, verification_key),
        _ => false,
    };
    let rejects_wrong = match &result {
        Ok(Some(auth::AuthMode::Token {
            token_hash,
            verification_key,
        })) => !auth::check_token("wrong-test-fixture", token_hash, verification_key),
        _ => true,
    };
    let known_default_authenticates = match &result {
        Ok(Some(auth::AuthMode::Token {
            token_hash,
            verification_key,
        })) => auth::check_token("e2e-dummy-token", token_hash, verification_key),
        _ => false,
    };
    let after = (
        std::env::var("OMNISOLO_AGENT_TOKEN").ok(),
        std::env::var("OMNISOLO_AGENT_AUTH_KEY").ok(),
    );
    println!(
        "{}",
        serde_json::json!({
            "ok": result.is_ok(), "configured_token": configured_token,
            "accepted_fixture": accepted_fixture, "rejects_wrong": rejects_wrong,
            "credentials_unchanged": before == after,
        "known_default_authenticates": known_default_authenticates,
            "token_present": after.0.is_some(), "key_present": after.1.is_some(),
            "error": result.err().map(|e| e.to_string()),
        })
    );
}
