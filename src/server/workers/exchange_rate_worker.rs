use std::sync::Arc;
use crate::db::DB;
use std::time::Duration;
use tokio::time::sleep;
pub struct ExchangeRateWorker {
    pub db: Arc<DB>,
}
impl ExchangeRateWorker {
    pub fn new(db: Arc<DB>) -> Self {
        Self { db }
    }
    pub fn start(&self) {
        let redis_client = crate::get_redis_client();
        tokio::spawn(async move {
            loop {
                if let Some(ref client) = redis_client {
                    if let Ok(mut con) = client.get_async_connection().await {
                        let mock_rates = [
                            ("EUR_USD", "1.08"),
                            ("GBP_USD", "1.25"),
                            ("JPY_USD", "0.0065"),
                            ("AUD_USD", "0.65"),
                            ("CAD_USD", "0.74"),
                        ];
                        for (pair, rate) in mock_rates.iter() {
                            let _: redis::RedisResult<()> = redis::cmd("SET")
                                .arg(format!("exchange_rate:{}", pair))
                                .arg(rate)
                                .query_async(&mut con)
                                .await;
                        }
                    }
                }
                sleep(Duration::from_secs(3600)).await;
            }
        });
    }
}
