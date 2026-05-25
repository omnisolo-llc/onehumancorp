pub struct RazorpayClient {
    pub key_id: String,
    pub key_secret: String,
}

impl RazorpayClient {
    pub fn new(key_id: String, key_secret: String) -> Self {
        Self { key_id, key_secret }
    }

    pub async fn create_payment_link(&self, amount: f64, description: &str, _customer_email: &str) -> Result<String, String> {
        // Mock implementation
        Ok(format!("https://checkout.razorpay.com/pay?amount={}&description={}", amount, description))
    }

    pub async fn fetch_payment(&self, payment_id: &str) -> Result<String, String> {
        // Mock implementation
        Ok(format!("mock_status_for_{}", payment_id))
    }
}
