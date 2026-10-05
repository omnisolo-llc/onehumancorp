use super::*;
use axum::http::Request;
use tower::ServiceExt;

const FORM: &str = "Body=first&Body=caf%C3%A9+%F0%9F%98%80=a%3Db%26c&SpeechResult=caf%C3%A9+%F0%9F%98%80=oui&Plus=+%2B&Empty=&Bare&=value&&Broken=%Q1%&Bad=%FF";
// Independently calculated HMAC-SHA1 fixture, retaining duplicate value order.
const SIGNATURE: &str = "WtgTno3gUhpc0+wtchHGhWPCFCs=";

#[test]
fn signature_preserves_duplicate_order_and_rejects_tampering() {
    let url = "https://example.test/twilio?route=one";
    assert!(valid_twilio_signature(
        "fixture-token",
        url,
        FORM.as_bytes(),
        Some(SIGNATURE)
    ));
    for changed in [
        FORM.replace("Body=first&Body=", "Body="),
        FORM.replace("Body=first&", "") + "&Body=first",
        FORM.replace("oui", "non"),
    ] {
        assert!(!valid_twilio_signature(
            "fixture-token",
            url,
            changed.as_bytes(),
            Some(SIGNATURE)
        ));
    }
    assert!(!valid_twilio_signature(
        "fixture-token",
        url,
        FORM.as_bytes(),
        None
    ));
    assert!(!valid_twilio_signature(
        "fixture-token",
        url,
        FORM.as_bytes(),
        Some("invalid")
    ));
}

#[tokio::test]
async fn signed_sms_and_speech_fields_reach_the_parser_with_raw_bytes_intact() {
    let app = axum::Router::new()
        .route(
            "/twilio",
            axum::routing::post(|body: axum::body::Bytes| async move {
                assert_eq!(
                    body.as_ref(),
                    FORM.as_bytes(),
                    "middleware must not reserialize signed bytes"
                );
                axum::Json(parse_form_urlencoded(&body))
            }),
        )
        .route_layer(axum::middleware::from_fn(twilio_signature_middleware));
    temp_env::async_with_vars(
        [
            ("TWILIO_AUTH_TOKEN", Some("fixture-token")),
            ("TWILIO_WEBHOOK_BASE_URL", Some("https://example.test")),
        ],
        async {
            let response = app
                .oneshot(
                    Request::builder()
                        .method("POST")
                        .uri("/twilio?route=one")
                        .header("x-twilio-signature", SIGNATURE)
                        .body(Body::from(FORM))
                        .unwrap(),
                )
                .await
                .unwrap();
            assert_eq!(response.status(), StatusCode::OK);
            let bytes = axum::body::to_bytes(response.into_body(), 4096)
                .await
                .unwrap();
            let values: serde_json::Value = serde_json::from_slice(&bytes).unwrap();
            assert_eq!(values["Body"], "café 😀=a=b&c");
            assert_eq!(values["SpeechResult"], "café 😀=oui");
            assert_eq!(values["Plus"], " +");
            assert_eq!(values["Empty"], "");
            assert_eq!(values["Bare"], "");
            assert_eq!(values["Broken"], "%Q1%");
            assert_eq!(values["Bad"], "�");
        },
    )
    .await;
}

#[tokio::test]
async fn signature_middleware_rejects_invalid_auth_and_oversized_forms_before_handler() {
    let app = axum::Router::new()
        .route(
            "/twilio",
            axum::routing::post(|| async { StatusCode::NO_CONTENT }),
        )
        .route_layer(axum::middleware::from_fn(twilio_signature_middleware));
    temp_env::async_with_vars(
        [
            ("TWILIO_AUTH_TOKEN", Some("fixture-token")),
            ("TWILIO_WEBHOOK_BASE_URL", Some("https://example.test")),
        ],
        async {
            for (body, signature, expected) in [
                (FORM.to_owned(), "invalid", StatusCode::UNAUTHORIZED),
                (
                    FORM.replace("oui", "non"),
                    SIGNATURE,
                    StatusCode::UNAUTHORIZED,
                ),
                (
                    "a".repeat(262_145),
                    SIGNATURE,
                    StatusCode::PAYLOAD_TOO_LARGE,
                ),
            ] {
                let response = app
                    .clone()
                    .oneshot(
                        Request::builder()
                            .method("POST")
                            .uri("/twilio?route=one")
                            .header("x-twilio-signature", signature)
                            .body(Body::from(body))
                            .unwrap(),
                    )
                    .await
                    .unwrap();
                assert_eq!(response.status(), expected);
            }
        },
    )
    .await;
}
