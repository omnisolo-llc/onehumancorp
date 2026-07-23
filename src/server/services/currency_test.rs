use crate::hub::Hub;
use crate::services::currency::CurrencyService;
use std::sync::Arc;

#[tokio::test]
async fn test_currency_conversion() {
    let hub = Arc::new(Hub::new(tokio::sync::mpsc::channel(100).0, sqlx::PgPool::connect("postgres://postgres:postgres@localhost:5432/postgres").await.unwrap()));
    let service = CurrencyService::new(hub.clone());

    // Just compile-check
}
