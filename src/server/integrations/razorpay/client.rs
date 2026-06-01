pub struct RazorpayClient {
    pub api_key: String,
    pub api_secret: String,
}

impl RazorpayClient {
    pub fn new(api_key: String, api_secret: String) -> Self {
        Self {
            api_key,
            api_secret,
        }
    }

    pub async fn create_checkout_preference(&self, price_id: &str, tenant_id: &str) -> Result<String, String> {
        Ok(format!("razorpay_checkout_{}_{}", price_id, tenant_id))
    }

    pub async fn create_payment(&self, _amount: f64, _description: &str, _payer_email: &str) -> Result<String, String> {
        // Mock returning a transaction ID
        Ok("mock_razorpay_txn_123".to_string())
    }

    pub async fn handle_webhook(&self, _payload: &str) -> Result<(), String> {
        // Mock handle webhook
        Ok(())
    }
}
