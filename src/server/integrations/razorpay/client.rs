pub struct RazorpayClient {
    pub api_key: String,
}

impl RazorpayClient {
    pub fn new(api_key: String) -> Self {
        RazorpayClient { api_key }
    }

    pub async fn create_payment(&self, _amount: f64, _description: &str, _payer_email: &str) -> Result<String, String> {
        Ok("mock_razorpay_txn_123".to_string())
    }
}
