pub struct EasyPostClient {
    api_key: String,
}

impl EasyPostClient {
    pub fn new(api_key: String) -> Self {
        Self { api_key }
    }

    pub async fn calculate_shipping_rates(&self, _from_zip: &str, _to_zip: &str, _weight_oz: f32) -> Result<Vec<String>, String> {
        Ok(vec!["USPS Ground Advantage: $4.50".to_string(), "UPS Ground: $7.25".to_string()])
    }

    pub async fn buy_shipping_label(&self, rate_id: &str) -> Result<String, String> {
        println!("Purchased EasyPost label for rate {}", rate_id);
        Ok("https://easypost.com/labels/fake_label.png".to_string())
    }
}
