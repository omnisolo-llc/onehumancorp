use serde_json::Value;
use std::process::Command;

fn probe(standalone: bool, values: &[(&str, &str)]) -> Value {
    let mut command = Command::new(env!("CARGO_BIN_EXE_startup-auth-probe"));
    command.env_clear().env(
        "PROBE_STANDALONE",
        if standalone { "true" } else { "false" },
    );
    for (key, value) in values {
        command.env(key, value);
    }
    let output = command.output().expect("run non-cfg(test) startup probe");
    assert!(
        output.status.success(),
        "probe crashed: {:?}",
        output.status
    );
    serde_json::from_slice(&output.stdout).expect("structured probe result")
}
fn fails_without_mutation(result: &Value) {
    assert_eq!(result["ok"], false, "{result}");
    assert_eq!(result["configured_token"], false, "{result}");
    assert_eq!(result["credentials_unchanged"], true, "{result}");
    assert_eq!(result["known_default_authenticates"], false, "{result}");
}
#[test]
fn missing_standalone_configuration_fails_before_any_credential_is_invented() {
    let r = probe(true, &[]);
    fails_without_mutation(&r);
    assert_eq!(r["token_present"], false);
    assert_eq!(r["key_present"], false);
    assert!(
        r["error"]
            .as_str()
            .unwrap()
            .contains("configure OMNISOLO_AGENT_TOKEN")
    );
}
#[test]
fn cluster_startup_does_not_create_or_overwrite_agent_credentials() {
    for vars in [
        vec![],
        vec![("OMNISOLO_AGENT_AUTH_KEY", "existing-test-key")],
    ] {
        let r = probe(false, &vars);
        assert_eq!(r["ok"], true);
        assert_eq!(r["configured_token"], false);
        assert_eq!(r["credentials_unchanged"], true);
        assert_eq!(r["token_present"], false);
    }
}
#[test]
fn explicit_fixture_credentials_are_preserved_and_authenticate_only_the_matching_token() {
    let r = probe(
        true,
        &[
            ("OMNISOLO_AGENT_TOKEN", "explicit-test-fixture-only"),
            (
                "OMNISOLO_AGENT_AUTH_KEY",
                "0123456789abcdef0123456789abcdef",
            ),
        ],
    );
    for key in [
        "ok",
        "configured_token",
        "accepted_fixture",
        "rejects_wrong",
        "credentials_unchanged",
    ] {
        assert_eq!(r[key], true, "{r}");
    }
}
#[test]
fn token_without_key_is_rejected() {
    fails_without_mutation(&probe(
        true,
        &[("OMNISOLO_AGENT_TOKEN", "explicit-test-fixture-only")],
    ));
}
#[test]
fn empty_or_short_configuration_is_rejected_without_replacement() {
    for (token, key) in [
        ("", "0123456789abcdef0123456789abcdef"),
        ("explicit-test-fixture-only", "short"),
    ] {
        fails_without_mutation(&probe(
            true,
            &[
                ("OMNISOLO_AGENT_TOKEN", token),
                ("OMNISOLO_AGENT_AUTH_KEY", key),
            ],
        ));
    }
}
#[test]
fn unsupported_spiffe_never_falls_back_to_a_predictable_token() {
    let r = probe(
        true,
        &[(
            "OMNISOLO_AGENT_SPIFFE_ID",
            "spiffe://ohc.local/org/test/agent/test",
        )],
    );
    fails_without_mutation(&r);
    assert!(r["error"].as_str().unwrap().contains("mTLS"));
    assert_eq!(r["token_present"], false);
}
#[test]
fn production_binary_cannot_disable_auth_even_with_test_environment_label() {
    for env in ["production", "development", "test"] {
        let r = probe(
            true,
            &[
                ("OMNISOLO_AGENT_AUTH_DISABLED", "true"),
                ("OMNISOLO_ENV", env),
            ],
        );
        fails_without_mutation(&r);
        assert!(
            r["error"]
                .as_str()
                .unwrap()
                .contains("not allowed in production binaries")
        );
    }
}
