//! Local configuration validation only. Successful validation does not verify a
//! provider account or create a durable connection receipt.

#[derive(Default)]
pub struct RegistryConnectionInput<'a> {
    pub integration_id: &'a str,
    pub base_url: &'a str,
    pub bot_token: &'a str,
    pub webhook_url: &'a str,
    pub api_token: &'a str,
    pub api_key: &'a str,
    pub api_secret: &'a str,
}

pub fn validate_registry_connection(
    input: RegistryConnectionInput<'_>,
) -> Result<(), &'static str> {
    let present = |value: &str| !value.trim().is_empty();
    let valid = match input.integration_id {
        "razorpay" => {
            if !present(input.api_key) || !present(input.api_secret) {
                return Err("Razorpay requires explicit api_key and api_secret fields");
            }
            true
        }
        "trello" | "twilio" => present(input.api_token) && present(input.bot_token),
        "slack" | "telegram" => present(input.bot_token),
        "discord" => present(input.webhook_url),
        "nats" => present(input.base_url),
        "meta" | "whatsapp" | "whatsapp_cloud_api" | "calendly" | "cal_com"
        | "google_workspace" | "google_calendar" | "mailchimp" | "alipay" | "mercadopago"
        | "shippo" | "taxjar" | "zoom" | "jitsi" | "ayrshare" | "listmonk" | "doordash"
        | "easypost" | "manychat" | "resend" | "sendgrid" | "google_analytics" | "github_api"
        | "outlook_calendar" => present(input.api_token),
        _ => return Err("Integration configuration is not supported for this provider"),
    };
    if !valid {
        return Err("Required integration configuration is missing");
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unknown_and_catalog_only_providers_are_rejected() {
        for integration_id in ["", "unknown", "chromadb", "restic", "shipday"] {
            assert!(
                validate_registry_connection(RegistryConnectionInput {
                    integration_id,
                    api_token: "local-test-input",
                    ..Default::default()
                })
                .is_err()
            );
        }
    }

    #[test]
    fn supported_api_providers_require_nonblank_supplied_tokens() {
        for integration_id in [
            "meta",
            "whatsapp",
            "whatsapp_cloud_api",
            "calendly",
            "cal_com",
            "google_workspace",
            "google_calendar",
            "mailchimp",
            "alipay",
            "mercadopago",
            "shippo",
            "taxjar",
            "zoom",
            "jitsi",
            "ayrshare",
            "listmonk",
            "doordash",
            "easypost",
            "manychat",
            "resend",
            "sendgrid",
            "google_analytics",
            "github_api",
            "outlook_calendar",
        ] {
            for api_token in ["", " \n\t"] {
                assert!(
                    validate_registry_connection(RegistryConnectionInput {
                        integration_id,
                        api_token,
                        ..Default::default()
                    })
                    .is_err(),
                    "{integration_id}"
                );
            }
            assert!(
                validate_registry_connection(RegistryConnectionInput {
                    integration_id,
                    api_token: "local-test-input",
                    ..Default::default()
                })
                .is_ok(),
                "{integration_id}"
            );
        }
    }

    #[test]
    fn paired_providers_require_both_input_fields() {
        for integration_id in ["twilio", "trello"] {
            assert!(
                validate_registry_connection(RegistryConnectionInput {
                    integration_id,
                    api_token: "local-test-input",
                    ..Default::default()
                })
                .is_err()
            );
            assert!(
                validate_registry_connection(RegistryConnectionInput {
                    integration_id,
                    bot_token: "local-test-input",
                    ..Default::default()
                })
                .is_err()
            );
            assert!(
                validate_registry_connection(RegistryConnectionInput {
                    integration_id,
                    api_token: "local-test-input",
                    bot_token: "local-test-input",
                    ..Default::default()
                })
                .is_ok()
            );
        }
    }

    #[test]
    fn razorpay_does_not_infer_a_secret_from_an_api_token_or_bot_token() {
        assert!(
            validate_registry_connection(RegistryConnectionInput {
                integration_id: "razorpay",
                api_token: "local-test-input",
                bot_token: "local-test-input",
                ..Default::default()
            })
            .is_err()
        );
        assert!(
            validate_registry_connection(RegistryConnectionInput {
                integration_id: "razorpay",
                api_key: "local-test-input",
                ..Default::default()
            })
            .is_err()
        );
        assert!(
            validate_registry_connection(RegistryConnectionInput {
                integration_id: "razorpay",
                api_secret: "local-test-input",
                ..Default::default()
            })
            .is_err()
        );
        assert!(
            validate_registry_connection(RegistryConnectionInput {
                integration_id: "razorpay",
                api_key: "local-test-key",
                api_secret: "local-test-input",
                ..Default::default()
            })
            .is_ok()
        );
    }

    #[test]
    fn bot_webhook_and_transport_configuration_require_their_actual_inputs() {
        for integration_id in ["slack", "telegram", "discord", "nats"] {
            assert!(
                validate_registry_connection(RegistryConnectionInput {
                    integration_id,
                    api_token: "unrelated-field",
                    ..Default::default()
                })
                .is_err()
            );
            assert!(
                validate_registry_connection(RegistryConnectionInput {
                    integration_id,
                    bot_token: "local-test-input",
                    webhook_url: "https://example.invalid/hook",
                    base_url: "nats://127.0.0.1:4222",
                    ..Default::default()
                })
                .is_ok()
            );
        }
    }
}
