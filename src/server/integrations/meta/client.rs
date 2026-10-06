use async_trait::async_trait;
use lazy_static::lazy_static;
use regex::Regex;
use reqwest::Client;

lazy_static! {
    static ref BSUID_REGEX: Regex =
        Regex::new(r"^[A-Z]{2}\.(?:ENT\.)?[A-Za-z0-9]{1,128}$").unwrap();
}

pub fn is_business_scoped_recipient(to: &str) -> bool {
    BSUID_REGEX.is_match(to)
}

pub(crate) fn build_meta_payload(platform: &str, to: &str, body: &str) -> serde_json::Value {
    if platform == "instagram" || platform == "facebook" {
        serde_json::json!({
            "recipient": {
                "id": to
            },
            "message": {
                "text": body
            }
        })
    } else if platform == "whatsapp" {
        if BSUID_REGEX.is_match(to) {
            serde_json::json!({
                "messaging_product": "whatsapp",
                "recipient_type": "individual",
                "recipient": to,
                "type": "text",
                "text": {
                    "preview_url": false,
                    "body": body
                }
            })
        } else {
            serde_json::json!({
                "messaging_product": "whatsapp",
                "recipient_type": "individual",
                "to": to,
                "type": "text",
                "text": {
                    "preview_url": false,
                    "body": body
                }
            })
        }
    } else {
        serde_json::json!({
            "recipient": {
                "id": to
            },
            "message": {
                "text": body
            },
            "messaging_type": "RESPONSE"
        })
    }
}

#[async_trait]
pub trait MetaClientWrapper: Send + Sync {
    /// Legacy/custom clients cannot claim a provider receipt they do not expose.
    async fn send_message_receipt(
        &self,
        _platform: &str,
        _from: Option<&str>,
        _to: &str,
        _body: &str,
    ) -> Result<MetaMessageReceipt, MetaSendError> {
        Err(MetaSendError::InvalidConfiguration)
    }

    async fn send_message(
        &self,
        platform: &str,
        from: Option<&str>,
        to: &str,
        body: &str,
    ) -> Result<(), String>;
}

/// Provider acceptance only. Delivery requires a separately verified webhook.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MetaMessageReceipt {
    pub message_id: String,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MetaSendError {
    InvalidConfiguration,
    Rejected { status: u16 },
    UnknownOutcome { reason: &'static str },
}
impl std::fmt::Display for MetaSendError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidConfiguration => f.write_str("Meta sender configuration is unavailable"),
            Self::Rejected { status } => write!(f, "Meta rejected the message (HTTP {status})"),
            Self::UnknownOutcome { reason } => write!(
                f,
                "Meta outcome unknown ({reason}); reconcile before retrying"
            ),
        }
    }
}
impl std::error::Error for MetaSendError {}

pub struct RealMetaClient {
    access_token: String,
    http_client: Client,
    base_url: String,
}

impl RealMetaClient {
    pub fn new(access_token: String) -> Self {
        Self {
            access_token,
            http_client: Client::builder()
                .connect_timeout(std::time::Duration::from_secs(5))
                .timeout(std::time::Duration::from_secs(30))
                .retry(reqwest::retry::never())
                .redirect(reqwest::redirect::Policy::none())
                .build()
                .expect("valid Meta HTTP configuration"),
            base_url: "https://graph.facebook.com/v19.0".into(),
        }
    }
}

#[async_trait]
impl MetaClientWrapper for RealMetaClient {
    async fn send_message(
        &self,
        platform: &str,
        from: Option<&str>,
        to: &str,
        body: &str,
    ) -> Result<(), String> {
        // Preserve the legacy token-scoped sender route. Canonical department
        // delivery calls the receipt API with an explicit reviewed account ID.
        self.send_message_receipt(platform, from.or(Some("me")), to, body)
            .await
            .map(|_| ())
            .map_err(|error| error.to_string())
    }

    async fn send_message_receipt(
        &self,
        platform: &str,
        from: Option<&str>,
        to: &str,
        body: &str,
    ) -> Result<MetaMessageReceipt, MetaSendError> {
        const LIMIT: usize = 64 * 1024;
        let from = from
            .filter(|value| {
                !value.is_empty() && (*value == "me" || value.bytes().all(|b| b.is_ascii_digit()))
            })
            .ok_or(MetaSendError::InvalidConfiguration)?;
        if !matches!(platform, "whatsapp" | "instagram" | "facebook")
            || self.access_token.trim().is_empty()
            || to.trim().is_empty()
            || body.trim().is_empty()
        {
            return Err(MetaSendError::InvalidConfiguration);
        }
        let unknown = |reason| MetaSendError::UnknownOutcome { reason };
        let mut response = self
            .http_client
            .post(format!("{}/{from}/messages", self.base_url))
            .bearer_auth(&self.access_token)
            .json(&build_meta_payload(platform, to, body))
            .send()
            .await
            .map_err(|_| unknown("response unavailable"))?;
        if response.status().is_client_error() {
            return Err(MetaSendError::Rejected {
                status: response.status().as_u16(),
            });
        }
        if !response.status().is_success() {
            return Err(unknown("acceptance not confirmed"));
        }
        if response
            .content_length()
            .is_some_and(|size| size > LIMIT as u64)
        {
            return Err(unknown("receipt too large"));
        }
        let mut bytes = Vec::new();
        while let Some(chunk) = response
            .chunk()
            .await
            .map_err(|_| unknown("receipt unavailable"))?
        {
            if chunk.len() > LIMIT - bytes.len() {
                return Err(unknown("receipt too large"));
            }
            bytes.extend_from_slice(&chunk);
        }
        let receipt: serde_json::Value =
            serde_json::from_slice(&bytes).map_err(|_| unknown("invalid receipt"))?;
        if receipt.get("error").is_some()
            || receipt.get("success") == Some(&serde_json::Value::Bool(false))
        {
            return Err(unknown("receipt reports an error"));
        }
        let id = if platform == "whatsapp" {
            let messages = receipt["messages"]
                .as_array()
                .filter(|items| items.len() == 1)
                .ok_or_else(|| unknown("missing message identity"))?;
            let contacts = receipt["contacts"]
                .as_array()
                .filter(|items| items.len() == 1)
                .ok_or_else(|| unknown("missing recipient identity"))?;
            let target = to.trim_start_matches('+');
            if receipt["messaging_product"] != "whatsapp"
                || !(contacts[0]["wa_id"].as_str() == Some(target)
                    || (BSUID_REGEX.is_match(to) && contacts[0]["user_id"].as_str() == Some(to)))
            {
                return Err(unknown("recipient mismatch"));
            }
            messages[0]["id"]
                .as_str()
                .filter(|id| id.starts_with("wamid.") && id.len() > 6)
        } else {
            if receipt["recipient_id"].as_str() != Some(to) {
                return Err(unknown("recipient mismatch"));
            }
            receipt["message_id"].as_str()
        }
        .filter(|id| !id.trim().is_empty() && id.len() <= 1024)
        .ok_or_else(|| unknown("missing message identity"))?;
        Ok(MetaMessageReceipt {
            message_id: id.to_owned(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_real_client_creation() {
        let client = RealMetaClient::new("token".to_string());
        assert_eq!(client.access_token, "token");
    }

    #[test]
    fn test_build_meta_payload_whatsapp_standard() {
        let payload = build_meta_payload("whatsapp", "1234567890", "hello");
        assert_eq!(payload["to"], "1234567890");
        assert!(payload.get("recipient").is_none());
    }

    #[test]
    fn test_build_meta_payload_whatsapp_bsuid() {
        let payload = build_meta_payload("whatsapp", "BR.ENT.123456789", "hello");
        assert_eq!(payload["recipient"], "BR.ENT.123456789");
        assert!(payload.get("to").is_none());
    }

    #[test]
    fn test_build_meta_payload_other() {
        let payload = build_meta_payload("facebook", "1234567890", "hello");
        assert_eq!(payload["recipient"]["id"], "1234567890");
    }
}

#[cfg(test)]
mod receipt_tests {
    use super::*;
    use std::io::{Read, Write};
    fn fixture(status: u16, body: &str) -> (RealMetaClient, std::thread::JoinHandle<String>) {
        let listener = std::net::TcpListener::bind("127.0.0.1:0").unwrap();
        let address = listener.local_addr().unwrap();
        let body = body.to_owned();
        let worker = std::thread::spawn(move || {
            let (mut stream, _) = listener.accept().unwrap();
            stream
                .set_read_timeout(Some(std::time::Duration::from_secs(2)))
                .unwrap();
            let mut bytes = Vec::new();
            let mut buffer = [0; 4096];
            loop {
                let n = stream.read(&mut buffer).unwrap();
                bytes.extend_from_slice(&buffer[..n]);
                if n == 0 {
                    break;
                }
                if let Some(end) = bytes.windows(4).position(|b| b == b"\r\n\r\n") {
                    let headers = String::from_utf8_lossy(&bytes[..end]);
                    let length = headers
                        .lines()
                        .find_map(|line| {
                            line.split_once(':')
                                .filter(|(name, _)| name.eq_ignore_ascii_case("content-length"))
                                .map(|(_, value)| value.trim().parse::<usize>().unwrap())
                        })
                        .unwrap_or(0);
                    if bytes.len() >= end + 4 + length {
                        break;
                    }
                }
            }
            write!(stream,"HTTP/1.1 {status} fixture\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{body}",body.len()).unwrap();
            String::from_utf8(bytes).unwrap()
        });
        let mut client = RealMetaClient::new("synthetic-local-fixture-token".into());
        client.base_url = format!("http://{address}/v19.0");
        (client, worker)
    }
    #[tokio::test]
    async fn actual_meta_http_adapter_requires_a_correlated_whatsapp_id() {
        let (client, worker) = fixture(
            200,
            r#"{"messaging_product":"whatsapp","contacts":[{"input":"14155550123","wa_id":"14155550123"}],"messages":[{"id":"wamid.fixture"}]}"#,
        );
        assert_eq!(
            client
                .send_message_receipt(
                    "whatsapp",
                    Some("12345"),
                    "14155550123",
                    "Exact approved reply"
                )
                .await
                .unwrap()
                .message_id,
            "wamid.fixture"
        );
        let request = worker.join().unwrap();
        assert!(request.starts_with("POST /v19.0/12345/messages "));
        let body: serde_json::Value =
            serde_json::from_str(request.split_once("\r\n\r\n").unwrap().1).unwrap();
        assert_eq!(body["to"], "14155550123");
        assert_eq!(body["text"]["body"], "Exact approved reply");
    }
    #[tokio::test]
    async fn actual_meta_http_adapter_rejects_empty_malformed_and_mismatched_receipts() {
        for body in [
            "{}",
            "not json",
            r#"{"messages":[{"id":"wamid.fixture"}]}"#,
            r#"{"recipient_id":"other","message_id":"mid.fixture"}"#,
        ] {
            let (client, worker) = fixture(200, body);
            assert!(matches!(
                client
                    .send_message_receipt("instagram", Some("12345"), "target", "hello")
                    .await,
                Err(MetaSendError::UnknownOutcome { .. })
            ));
            worker.join().unwrap();
        }
    }
    #[tokio::test]
    async fn actual_meta_http_adapter_separates_rejection_from_ambiguous_server_outcomes() {
        for (status, rejected) in [
            (400, true),
            (401, true),
            (429, true),
            (500, false),
            (302, false),
        ] {
            let (client, worker) = fixture(status, "{}");
            let result = client
                .send_message_receipt("instagram", Some("12345"), "target", "hello")
                .await;
            assert_eq!(
                matches!(result, Err(MetaSendError::Rejected { .. })),
                rejected
            );
            assert!(result.is_err());
            worker.join().unwrap();
        }
    }
    #[tokio::test]
    async fn actual_meta_http_adapter_returns_instagram_acceptance_not_delivery() {
        let (client, worker) = fixture(
            200,
            r#"{"recipient_id":"target","message_id":"mid.fixture"}"#,
        );
        assert_eq!(
            client
                .send_message_receipt("instagram", Some("12345"), "target", "hello")
                .await
                .unwrap()
                .message_id,
            "mid.fixture"
        );
        worker.join().unwrap();
    }
}
