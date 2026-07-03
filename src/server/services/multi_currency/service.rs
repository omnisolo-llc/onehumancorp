use crate::services::multi_currency::data::{Currency, ProductPrice, CacheInvalidationEvent};
use uuid::Uuid;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

pub trait MultiCurrencyService {
    fn get_exchange_rate(&self, code: &str) -> Option<f64>;
    fn update_exchange_rate(&self, code: &str, rate: f64);
    fn get_localized_price(&self, product_id: Uuid, currency_code: &str) -> Option<f64>;
    fn set_localized_price(&self, product_id: Uuid, currency_code: &str, price: f64);
    fn trigger_cache_invalidation(&self, path: &str) -> CacheInvalidationEvent;
}

pub struct MyMultiCurrencyService {
    rates: Arc<Mutex<HashMap<String, f64>>>,
    prices: Arc<Mutex<HashMap<(Uuid, String), f64>>>,
}

impl MyMultiCurrencyService {
    pub fn new() -> Self {
        Self {
            rates: Arc::new(Mutex::new(HashMap::new())),
            prices: Arc::new(Mutex::new(HashMap::new())),
        }
    }
}

impl MultiCurrencyService for MyMultiCurrencyService {
    fn get_exchange_rate(&self, code: &str) -> Option<f64> {
        let rates = self.rates.lock().unwrap();
        rates.get(code).cloned()
    }

    fn update_exchange_rate(&self, code: &str, rate: f64) {
        let mut rates = self.rates.lock().unwrap();
        rates.insert(code.to_string(), rate);
    }

    fn get_localized_price(&self, product_id: Uuid, currency_code: &str) -> Option<f64> {
        let prices = self.prices.lock().unwrap();
        prices.get(&(product_id, currency_code.to_string())).cloned()
    }

    fn set_localized_price(&self, product_id: Uuid, currency_code: &str, price: f64) {
        let mut prices = self.prices.lock().unwrap();
        prices.insert((product_id, currency_code.to_string()), price);
    }

    fn trigger_cache_invalidation(&self, path: &str) -> CacheInvalidationEvent {
        CacheInvalidationEvent {
            id: Uuid::new_v4(),
            path: path.to_string(),
            triggered_at: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs() as i64,
        }
    }
}
