//! The focused harness excludes the unrelated provider graph. Every attempt to
//! cross that boundary fails the test; it can never fabricate a dispatch receipt.
#[derive(Default)]
pub struct IntegrationsRegistry;
impl IntegrationsRegistry {
    pub fn new() -> Self {
        Self
    }
    pub async fn send_email(&self, _: &str, _: &str, _: &str, _: &str) -> Result<(), String> {
        panic!("unexpected provider boundary: send_email")
    }
    pub async fn send_sms(&self, _: &str, _: &str, _: &str, _: &str) -> Result<(), String> {
        panic!("unexpected provider boundary: send_sms")
    }
    pub async fn send_message(&self, _: &str, _: &str, _: &str, _: &str) -> Result<(), String> {
        panic!("unexpected provider boundary: send_message")
    }
}
