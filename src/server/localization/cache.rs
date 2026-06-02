use std::collections::HashMap;
use std::sync::{Arc, RwLock};

#[derive(Clone)]
pub struct LocalI18nCache {
    strings: Arc<RwLock<HashMap<String, HashMap<String, String>>>>,
    exchange_rates: Arc<RwLock<HashMap<String, f64>>>,
}

impl LocalI18nCache {
    pub fn new() -> Self {
        Self {
            strings: Arc::new(RwLock::new(HashMap::new())),
            exchange_rates: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub fn set_string(&self, lang: &str, key: &str, value: &str) {
        let mut strings = self.strings.write().unwrap();
        let lang_map = strings.entry(lang.to_string()).or_insert_with(HashMap::new);
        lang_map.insert(key.to_string(), value.to_string());
    }

    pub fn get_string(&self, lang: &str, key: &str) -> Option<String> {
        let strings = self.strings.read().unwrap();
        strings.get(lang).and_then(|lang_map| lang_map.get(key).cloned())
    }

    pub fn set_exchange_rate(&self, currency_pair: &str, rate: f64) {
        let mut rates = self.exchange_rates.write().unwrap();
        rates.insert(currency_pair.to_string(), rate);
    }

    pub fn get_exchange_rate(&self, currency_pair: &str) -> Option<f64> {
        let rates = self.exchange_rates.read().unwrap();
        rates.get(currency_pair).cloned()
    }
}
