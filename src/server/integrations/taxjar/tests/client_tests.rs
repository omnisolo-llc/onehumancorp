use server_integrations_taxjar::client::{TaxJarClient, TaxJarParams};

#[tokio::test]
async fn test_calculate_tax_requires_real_taxjar_credentials() {
    let client = TaxJarClient::new("dummy_token".to_string());
    let params = TaxJarParams {
        amount: 100.0,
        shipping: 10.0,
        to_country: "US",
        to_zip: "90002",
        to_state: "CA",
        from_country: "US",
        from_zip: "92093",
        from_state: "CA",
    };
    let err = client.calculate_tax(params).await.unwrap_err();
    assert!(err.contains("TaxJar API token is required"));
}
