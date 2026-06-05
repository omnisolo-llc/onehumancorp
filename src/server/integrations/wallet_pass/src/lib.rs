pub mod provider;
pub mod client;

pub use provider::WalletPassProvider;
pub use client::WalletPassClient;

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_generate_pass() {
        let client = WalletPassClient::new();
        let pass_data = client.generate_pass("cust_123", "Maya's Bakery", None, None).await.unwrap();

        let pass_json: serde_json::Value = serde_json::from_slice(&pass_data).unwrap();
        assert_eq!(pass_json["organizationName"], "Maya's Bakery");
        assert_eq!(pass_json["serialNumber"], "cust_123");
        assert_eq!(pass_json["barcode"]["message"], "cust_123");
    }
}
