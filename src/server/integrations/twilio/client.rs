use async_trait::async_trait;

#[async_trait]
pub trait TwilioClientWrapper: Send + Sync {
    async fn send_sms(&self, to: &str, from: &str, body: &str) -> Result<(), String>;
    async fn send_whatsapp(&self, to: &str, from: &str, body: &str) -> Result<(), String>;
    async fn provision_number(&self, area_code: &str) -> Result<String, String>;
}

use reqwest::Client;

pub struct RealTwilioClient {
    pub account_sid: String,
    pub auth_token: String,
    http_client: Client,
    provisioning_api: String,
}

impl RealTwilioClient {
    pub fn new(account_sid: String, auth_token: String) -> Self {
        RealTwilioClient {
            account_sid,
            auth_token,
            http_client: Client::new(),
            provisioning_api: "https://api.twilio.com".into(),
        }
    }
}

#[async_trait]
impl TwilioClientWrapper for RealTwilioClient {
    async fn send_sms(&self, to: &str, from: &str, body: &str) -> Result<(), String> {
        let url = format!(
            "https://api.twilio.com/2010-04-01/Accounts/{}/Messages.json",
            self.account_sid
        );

        let params = [("To", to), ("From", from), ("Body", body)];

        let mut retries = 3;
        while retries > 0 {
            let res = self
                .http_client
                .post(&url)
                .basic_auth(&self.account_sid, Some(&self.auth_token))
                .form(&params)
                .send()
                .await;

            match res {
                Ok(resp) => {
                    if resp.status().is_success() {
                        return Ok(());
                    } else if resp.status().is_server_error() {
                        retries -= 1;
                        if retries == 0 {
                            return Err(format!("Twilio API error: {}", resp.status()));
                        }
                        tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
                    } else {
                        return Err(format!("Twilio API error: {}", resp.status()));
                    }
                }
                Err(e) => {
                    retries -= 1;
                    if retries == 0 {
                        return Err(format!("Network error: {}", e));
                    }
                    tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
                }
            }
        }
        Err("Failed to send SMS after retries".to_string())
    }

    async fn provision_number(&self, area_code: &str) -> Result<String, String> {
        if self.account_sid.len() != 34
            || !self.account_sid.starts_with("AC")
            || !self.account_sid[2..]
                .bytes()
                .all(|byte| byte.is_ascii_hexdigit())
            || self.auth_token.trim().is_empty()
        {
            return Err("Twilio number provisioning is unavailable: configure a valid account SID and auth token".into());
        }

        let search_url = format!(
            "{}/2010-04-01/Accounts/{}/AvailablePhoneNumbers/US/Local.json",
            self.provisioning_api, self.account_sid
        );
        let search_res = self
            .http_client
            .get(&search_url)
            .basic_auth(&self.account_sid, Some(&self.auth_token))
            .query(&[("AreaCode", area_code)])
            .send()
            .await
            .map_err(|e| format!("Network error: {}", e))?;

        if !search_res.status().is_success() {
            return Err(format!("Twilio search API error: {}", search_res.status()));
        }

        let search_data: serde_json::Value = search_res
            .json()
            .await
            .map_err(|e| format!("Failed to parse response: {}", e))?;
        let phone_number = search_data
            .get("available_phone_numbers")
            .and_then(|arr| arr.as_array())
            .and_then(|arr| arr.first())
            .and_then(|obj| obj.get("phone_number"))
            .and_then(|s| s.as_str())
            .ok_or("No available numbers found")?;
        if !valid_phone_number(phone_number) {
            return Err("Twilio returned an invalid available phone number".into());
        }

        let provision_url = format!(
            "{}/2010-04-01/Accounts/{}/IncomingPhoneNumbers.json",
            self.provisioning_api, self.account_sid
        );
        let params = [("PhoneNumber", phone_number)];
        let provision_res = self
            .http_client
            .post(&provision_url)
            .basic_auth(&self.account_sid, Some(&self.auth_token))
            .form(&params)
            .send()
            .await
            .map_err(|e| format!("Network error: {}", e))?;

        if provision_res.status() != reqwest::StatusCode::OK
            && provision_res.status() != reqwest::StatusCode::CREATED
        {
            return Err(format!(
                "Twilio provisioning outcome was not confirmed (HTTP {}); reconcile before retrying",
                provision_res.status()
            ));
        }
        let receipt: serde_json::Value = provision_res.json().await.map_err(|_| {
            "Twilio provisioning receipt could not be read; reconcile before retrying".to_string()
        })?;
        let resource = receipt
            .get("sid")
            .and_then(|value| value.as_str())
            .unwrap_or("");
        if resource.len() != 34
            || !resource.starts_with("PN")
            || !resource[2..].bytes().all(|byte| byte.is_ascii_hexdigit())
            || receipt.get("account_sid").and_then(|value| value.as_str())
                != Some(self.account_sid.as_str())
            || receipt.get("phone_number").and_then(|value| value.as_str()) != Some(phone_number)
            || receipt.get("success") == Some(&serde_json::Value::Bool(false))
            || receipt.get("error").is_some_and(|value| !value.is_null())
        {
            return Err("Twilio provisioning receipt did not identify the requested account and number; reconcile before retrying".into());
        }
        Ok(phone_number.to_string())
    }

    async fn send_whatsapp(&self, to: &str, from: &str, body: &str) -> Result<(), String> {
        let url = format!(
            "https://api.twilio.com/2010-04-01/Accounts/{}/Messages.json",
            self.account_sid
        );

        let formatted_to = if to.starts_with("whatsapp:") {
            to.to_string()
        } else {
            format!("whatsapp:{}", to)
        };
        let formatted_from = if from.starts_with("whatsapp:") {
            from.to_string()
        } else {
            format!("whatsapp:{}", from)
        };

        let params = [
            ("To", formatted_to.as_str()),
            ("From", formatted_from.as_str()),
            ("Body", body),
        ];

        let mut retries = 3;
        while retries > 0 {
            let res = self
                .http_client
                .post(&url)
                .basic_auth(&self.account_sid, Some(&self.auth_token))
                .form(&params)
                .send()
                .await;

            match res {
                Ok(resp) => {
                    if resp.status().is_success() {
                        return Ok(());
                    } else if resp.status().is_server_error() {
                        retries -= 1;
                        if retries == 0 {
                            return Err(format!("Twilio API error: {}", resp.status()));
                        }
                        tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
                    } else {
                        return Err(format!("Twilio API error: {}", resp.status()));
                    }
                }
                Err(e) => {
                    retries -= 1;
                    if retries == 0 {
                        return Err(format!("Network error: {}", e));
                    }
                    tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
                }
            }
        }
        Err("Failed to send WhatsApp message after retries".to_string())
    }
}

fn valid_phone_number(value: &str) -> bool {
    let bytes = value.as_bytes();
    (3..=16).contains(&bytes.len())
        && bytes[0] == b'+'
        && (b'1'..=b'9').contains(&bytes[1])
        && bytes[2..].iter().all(u8::is_ascii_digit)
}

#[cfg(test)]
mod provisioning_tests {
    use super::*;
    use std::sync::{
        Arc,
        atomic::{AtomicUsize, Ordering},
    };
    use tokio::io::{AsyncReadExt, AsyncWriteExt};
    const ACCOUNT: &str = "AC11111111111111111111111111111111";
    const NUMBER: &str = "+14155550123";
    const RESOURCE: &str = "PN22222222222222222222222222222222";

    async fn recorded_provider(
        status: u16,
        body: String,
        drop_reply: bool,
    ) -> (
        RealTwilioClient,
        Arc<AtomicUsize>,
        tokio::task::JoinHandle<()>,
    ) {
        let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
        let address = listener.local_addr().unwrap();
        let calls = Arc::new(AtomicUsize::new(0));
        let count = calls.clone();
        let task = tokio::spawn(async move {
            loop {
                let (mut stream, _) = listener.accept().await.unwrap();
                let mut request = Vec::new();
                let mut buffer = [0_u8; 1024];
                let header_end = loop {
                    let size = stream.read(&mut buffer).await.unwrap();
                    assert!(size > 0);
                    request.extend_from_slice(&buffer[..size]);
                    assert!(request.len() < 16384);
                    if let Some(index) = request.windows(4).position(|part| part == b"\r\n\r\n") {
                        break index + 4;
                    }
                };
                let headers = String::from_utf8(request[..header_end].to_vec()).unwrap();
                let length = headers
                    .lines()
                    .find_map(|line| {
                        line.to_ascii_lowercase()
                            .strip_prefix("content-length:")
                            .map(|value| value.trim().parse::<usize>().unwrap())
                    })
                    .unwrap_or(0);
                while request.len() < header_end + length {
                    let size = stream.read(&mut buffer).await.unwrap();
                    assert!(size > 0);
                    request.extend_from_slice(&buffer[..size]);
                }
                let index = count.fetch_add(1, Ordering::SeqCst);
                assert!(
                    index < 2,
                    "Provisioning must not automatically retry an unknown outcome"
                );
                let (code, response) = if index == 0 {
                    assert!(headers.starts_with(&format!("GET /2010-04-01/Accounts/{ACCOUNT}/AvailablePhoneNumbers/US/Local.json?AreaCode=415 ")));
                    (
                        200,
                        serde_json::json!({"available_phone_numbers":[{"phone_number":NUMBER}]})
                            .to_string(),
                    )
                } else {
                    assert!(headers.starts_with(&format!(
                        "POST /2010-04-01/Accounts/{ACCOUNT}/IncomingPhoneNumbers.json "
                    )));
                    assert_eq!(
                        std::str::from_utf8(&request[header_end..]).unwrap(),
                        "PhoneNumber=%2B14155550123"
                    );
                    if drop_reply {
                        drop(stream);
                        continue;
                    }
                    (status, body.clone())
                };
                let wire = format!(
                    "HTTP/1.1 {code} fixture\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{response}",
                    response.len()
                );
                stream.write_all(wire.as_bytes()).await.unwrap();
                stream.shutdown().await.unwrap();
            }
        });
        let client = RealTwilioClient {
            account_sid: ACCOUNT.into(),
            auth_token: "synthetic-loopback-only".into(),
            http_client: Client::builder()
                .no_proxy()
                .timeout(std::time::Duration::from_secs(2))
                .build()
                .unwrap(),
            provisioning_api: format!("http://{address}"),
        };
        (client, calls, task)
    }
    fn receipt() -> serde_json::Value {
        serde_json::json!({"sid":RESOURCE,"account_sid":ACCOUNT,"phone_number":NUMBER})
    }
    async fn run(status: u16, body: String, drop_reply: bool) -> Result<String, String> {
        let (client, calls, task) = recorded_provider(status, body, drop_reply).await;
        let result = client.provision_number("415").await;
        assert_eq!(calls.load(Ordering::SeqCst), 2);
        assert!(
            !task.is_finished(),
            "recording provider rejected the actual wire request"
        );
        task.abort();
        result
    }
    #[tokio::test]
    async fn real_http_provision_receipt_is_bound_to_number_account_and_resource() {
        assert_eq!(
            run(201, receipt().to_string(), false).await.unwrap(),
            NUMBER
        );
    }
    #[tokio::test]
    async fn http_success_without_a_provisioned_resource_never_claims_a_number() {
        for value in [
            serde_json::json!({}),
            serde_json::json!({"success":false}),
            serde_json::json!({"sid":RESOURCE,"account_sid":ACCOUNT}),
        ] {
            assert!(
                run(201, value.to_string(), false).await.is_err(),
                "Unacknowledged response was accepted: {value}"
            );
        }
    }
    #[tokio::test]
    async fn foreign_or_contradictory_provision_receipts_are_rejected() {
        for (key, value) in [
            ("account_sid", "AC33333333333333333333333333333333"),
            ("phone_number", "+14155550999"),
            ("sid", "not-a-phone-resource"),
        ] {
            let mut body = receipt();
            body[key] = value.into();
            assert!(
                run(201, body.to_string(), false).await.is_err(),
                "Mismatched {key} was accepted"
            );
        }
        let mut body = receipt();
        body["success"] = false.into();
        assert!(run(201, body.to_string(), false).await.is_err());
    }
    #[tokio::test]
    async fn malformed_and_unfinished_provider_responses_remain_unknown() {
        assert!(run(201, "{".into(), false).await.is_err());
        assert!(run(202, receipt().to_string(), false).await.is_err());
        assert!(run(201, receipt().to_string(), true).await.is_err());
    }
    #[tokio::test]
    async fn provider_rejections_never_produce_a_number() {
        for status in [401, 403, 500] {
            assert!(run(status, receipt().to_string(), false).await.is_err());
        }
    }
}
