use std::collections::HashMap;
use std::sync::{Arc, Mutex};

pub struct LocalizationCache {
    translations: HashMap<String, HashMap<String, String>>,
    fx_rates: HashMap<String, f64>,
}

impl LocalizationCache {
    pub fn new() -> Self {
        Self {
            translations: HashMap::new(),
            fx_rates: HashMap::new(),
        }
    }

    pub fn set_translation(&mut self, locale: String, key: String, value: String) {
        self.translations.entry(locale).or_insert_with(HashMap::new).insert(key, value);
    }

    pub fn get_translation(&self, locale: &str, key: &str) -> Option<String> {
        self.translations.get(locale).and_then(|t| t.get(key).cloned())
    }

    pub fn set_fx_rate(&mut self, currency: String, rate: f64) {
        self.fx_rates.insert(currency, rate);
    }

    pub fn get_fx_rate(&self, currency: &str) -> Option<f64> {
        self.fx_rates.get(currency).copied()
    }
}

pub struct LocalizationEngine {
    tenant_caches: Mutex<HashMap<String, Arc<Mutex<LocalizationCache>>>>,
}

impl LocalizationEngine {
    pub fn new() -> Self {
        Self {
            tenant_caches: Mutex::new(HashMap::new()),
        }
    }

    pub fn get_cache(&self, tenant_id: &str) -> Arc<Mutex<LocalizationCache>> {
        let mut caches = self.tenant_caches.lock().unwrap();
        caches.entry(tenant_id.to_string())
            .or_insert_with(|| Arc::new(Mutex::new(LocalizationCache::new())))
            .clone()
    }
}
