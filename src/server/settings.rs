use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::RwLock;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AiProvider {
    pub name: String,
    pub api_key: Option<String>,
    pub base_url: Option<String>,
    pub model: String,
    pub enabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppSettings {
    pub listen_addr: String,
    pub db_path: Option<String>,
    pub postgres_url: Option<String>,
    pub redis_url: Option<String>,
    pub centrifuge_url: Option<String>,
    pub minimax_api_key: Option<String>,
    pub ai_providers: Vec<AiProvider>,
    pub extras: HashMap<String, String>,
    pub sms_critical_phone: Option<String>,
    pub sms_alert_urgent_booking: bool,
    pub sms_alert_failed_payment: bool,
    pub sms_alert_new_order: bool,
    pub delivery_enabled: bool,
    pub delivery_radius: Option<f64>,
    pub delivery_fee: Option<f64>,
    pub voice_receptionist_enabled: bool,
    pub voice_receptionist_number: Option<String>,
    pub voice_receptionist_persona: Option<String>,
    pub voice_receptionist_instructions: Option<String>,
    pub product_telemetry_enabled: bool,
}

impl Default for AppSettings {
    fn default() -> Self {
        AppSettings {
            listen_addr: "0.0.0.0:18789".to_string(),
            db_path: Some("ohc.db".to_string()),
            postgres_url: None,
            redis_url: None,
            centrifuge_url: Some("ws://localhost:8000/connection/websocket".to_string()),
            minimax_api_key: None,
            ai_providers: vec![],
            extras: HashMap::new(),
            sms_critical_phone: None,
            sms_alert_urgent_booking: false,
            sms_alert_failed_payment: false,
            sms_alert_new_order: false,
            delivery_enabled: false,
            delivery_radius: Some(5.0),
            delivery_fee: Some(8.50),
            voice_receptionist_enabled: false,
            voice_receptionist_number: None,
            voice_receptionist_persona: Some("Friendly".to_string()),
            voice_receptionist_instructions: None,
            product_telemetry_enabled: false,
        }
    }
}

pub struct Store {
    data: RwLock<AppSettings>,
    path: Option<PathBuf>,
}

use std::sync::Arc;
use std::sync::OnceLock;

static GLOBAL_STORE: OnceLock<Arc<Store>> = OnceLock::new();

impl Store {
    pub fn global() -> Arc<Store> {
        GLOBAL_STORE
            .get_or_init(|| {
                let path = crate::config::get_safe_user_dir().join("settings.json");
                let store = Store::from_file(path).unwrap_or_else(|_| Store::new());
                ::server_config::DYNAMIC_TELEMETRY_ENABLED.store(
                    store.get().product_telemetry_enabled,
                    std::sync::atomic::Ordering::Relaxed,
                );
                Arc::new(store)
            })
            .clone()
    }

    pub fn new() -> Self {
        Store {
            data: RwLock::new(AppSettings::default()),
            path: None,
        }
    }

    pub fn from_file(path: PathBuf) -> Result<Self, String> {
        if !path.exists() {
            return Ok(Store {
                data: RwLock::new(AppSettings::default()),
                path: Some(path),
            });
        }

        let content = std::fs::read_to_string(&path).map_err(|e| e.to_string())?;
        let data: AppSettings = serde_json::from_str(&content).map_err(|e| e.to_string())?;
        ::server_config::DYNAMIC_TELEMETRY_ENABLED.store(
            data.product_telemetry_enabled,
            std::sync::atomic::Ordering::Relaxed,
        );

        Ok(Store {
            data: RwLock::new(data),
            path: Some(path),
        })
    }

    pub fn save(&self) -> Result<(), String> {
        let data = self.data.read().unwrap();
        self.save_snapshot(&data)
    }

    fn save_snapshot(&self, data: &AppSettings) -> Result<(), String> {
        let Some(path) = &self.path else {
            return Ok(()); // Explicitly in-memory settings, not persisted consent.
        };
        let content = serde_json::to_vec_pretty(data).map_err(|e| e.to_string())?;
        // The shared helper stages beside the destination, syncs the file and
        // atomically replaces it. Readers never see truncated settings JSON.
        crate::utils::fs::write_file_atomic(path, &content, 0o600).map_err(|e| e.to_string())
    }

    pub fn get(&self) -> AppSettings {
        self.data.read().unwrap().clone()
    }

    pub(crate) fn has_persistent_storage(&self) -> bool {
        self.path.is_some()
    }

    pub(crate) fn telemetry_snapshot(&self) -> (bool, bool) {
        let data = self.data.read().unwrap();
        (
            data.product_telemetry_enabled,
            ::server_config::DYNAMIC_TELEMETRY_ENABLED.load(std::sync::atomic::Ordering::Acquire),
        )
    }

    pub fn set_extra(&self, key: String, value: String) -> Result<(), String> {
        let mut data = self.data.write().unwrap();
        data.extras.insert(key, value);
        drop(data);
        self.save()
    }

    pub fn set_sms_preferences(
        &self,
        phone: String,
        urgent_booking: bool,
        failed_payment: bool,
        new_order: bool,
    ) -> Result<(), String> {
        let mut data = self.data.write().unwrap();
        data.sms_critical_phone = Some(phone);
        data.sms_alert_urgent_booking = urgent_booking;
        data.sms_alert_failed_payment = failed_payment;
        data.sms_alert_new_order = new_order;
        drop(data);
        self.save()
    }

    pub fn set_delivery_settings(
        &self,
        enabled: bool,
        radius: Option<f64>,
        fee: Option<f64>,
    ) -> Result<(), String> {
        let mut data = self.data.write().unwrap();
        data.delivery_enabled = enabled;
        data.delivery_radius = radius;
        data.delivery_fee = fee;
        drop(data);
        self.save()
    }

    pub fn set_voice_settings(
        &self,
        enabled: bool,
        number: Option<String>,
        persona: Option<String>,
        instructions: Option<String>,
    ) -> Result<(), String> {
        let mut data = self.data.write().unwrap();
        data.voice_receptionist_enabled = enabled;
        data.voice_receptionist_number = number;
        data.voice_receptionist_persona = persona;
        data.voice_receptionist_instructions = instructions;
        drop(data);
        self.save()
    }

    pub fn set_product_telemetry(&self, enabled: bool) -> Result<(), String> {
        if self.path.is_none() {
            return Err("Persistent settings storage is unavailable".to_string());
        }
        // Keep the candidate private and serialize the entire persist/publish
        // transition. A failed write must not change effective collection.
        let mut data = self.data.write().unwrap();
        let mut candidate = data.clone();
        candidate.product_telemetry_enabled = enabled;
        self.save_snapshot(&candidate)?;
        *data = candidate;
        ::server_config::DYNAMIC_TELEMETRY_ENABLED
            .store(enabled, std::sync::atomic::Ordering::Release);
        Ok(())
    }
}

impl Default for Store {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use std::sync::atomic::{AtomicBool, Ordering};
    static TELEMETRY_TEST_LOCK: std::sync::Mutex<()> = std::sync::Mutex::new(());
    pub(crate) struct TelemetryTestGuard {
        _lock: std::sync::MutexGuard<'static, ()>,
        original: bool,
    }
    impl Drop for TelemetryTestGuard {
        fn drop(&mut self) {
            ::server_config::DYNAMIC_TELEMETRY_ENABLED.store(self.original, Ordering::Relaxed);
        }
    }
    pub(crate) fn telemetry_guard() -> TelemetryTestGuard {
        let lock = TELEMETRY_TEST_LOCK
            .lock()
            .unwrap_or_else(|error| error.into_inner());
        let original = ::server_config::DYNAMIC_TELEMETRY_ENABLED.swap(false, Ordering::Relaxed);
        TelemetryTestGuard {
            _lock: lock,
            original,
        }
    }

    #[test]
    fn test_settings_default() {
        let _guard = telemetry_guard();
        let settings = AppSettings::default();
        assert_eq!(settings.listen_addr, "0.0.0.0:18789");
        assert_eq!(settings.db_path, Some("ohc.db".to_string()));
        assert!(!settings.voice_receptionist_enabled);
        assert_eq!(settings.voice_receptionist_number, None);
        assert_eq!(
            settings.voice_receptionist_persona,
            Some("Friendly".to_string())
        );
        assert_eq!(settings.voice_receptionist_instructions, None);
    }

    #[test]
    fn test_store_save_and_load() {
        let _guard = telemetry_guard();
        let temp_dir = tempfile::tempdir().unwrap();
        let file_path = temp_dir.path().join("test_settings.json");

        let store = Store::from_file(file_path.clone()).unwrap();
        store
            .set_extra("key1".to_string(), "value1".to_string())
            .unwrap();

        assert!(file_path.exists());

        let store2 = Store::from_file(file_path.clone()).unwrap();
        let settings = store2.get();
        assert_eq!(settings.extras.get("key1").unwrap(), "value1");
    }

    #[test]
    fn test_store_from_file_errors() {
        let _guard = telemetry_guard();
        // Bad JSON
        let temp_dir = tempfile::tempdir().unwrap();
        let file_path = temp_dir.path().join("bad_settings.json");
        std::fs::write(&file_path, "{bad json").unwrap();

        let result = Store::from_file(file_path.clone());
        assert!(result.is_err());

        std::fs::remove_file(&file_path).unwrap();

        // Unreadable file (directory)
        let dir_path = temp_dir.path().join("some_dir");
        std::fs::create_dir(&dir_path).unwrap();
        let result = Store::from_file(dir_path.clone());
        assert!(result.is_err());
        std::fs::remove_dir(&dir_path).unwrap();
    }

    #[test]
    fn test_store_save_errors() {
        let _guard = telemetry_guard();
        let temp_dir = tempfile::tempdir().unwrap();
        let parent = temp_dir.path().join("not_a_directory");
        std::fs::write(&parent, b"owned test fixture").unwrap();
        let store = Store {
            data: RwLock::new(AppSettings::default()),
            path: Some(parent.join("settings.json")),
        };
        let result = store.save();
        assert!(result.is_err());
    }
    #[test]
    fn failed_telemetry_enable_does_not_publish_memory_or_collection_flag() {
        let _guard = telemetry_guard();
        let directory = tempfile::tempdir().unwrap();
        let blocked = directory.path().join("settings.json");
        std::fs::create_dir(&blocked).unwrap();
        let store = Store {
            data: RwLock::new(AppSettings::default()),
            path: Some(blocked.clone()),
        };
        assert!(store.set_product_telemetry(true).is_err());
        assert!(!store.get().product_telemetry_enabled);
        assert!(!::server_config::DYNAMIC_TELEMETRY_ENABLED.load(Ordering::Relaxed));
        assert!(blocked.is_dir());
        assert_eq!(
            std::fs::read_dir(directory.path()).unwrap().count(),
            1,
            "failed staging must be cleaned up"
        );
    }

    #[test]
    fn failed_telemetry_disable_preserves_last_committed_choice() {
        let _guard = telemetry_guard();
        let directory = tempfile::tempdir().unwrap();
        let path = directory.path().join("settings.json");
        let backup = directory.path().join("committed.json");
        let store = Store::from_file(path.clone()).unwrap();
        store.set_product_telemetry(true).unwrap();
        std::fs::rename(&path, &backup).unwrap();
        std::fs::create_dir(&path).unwrap();
        assert!(store.set_product_telemetry(false).is_err());
        assert!(store.get().product_telemetry_enabled);
        assert!(::server_config::DYNAMIC_TELEMETRY_ENABLED.load(Ordering::Relaxed));
        let persisted: AppSettings =
            serde_json::from_slice(&std::fs::read(backup).unwrap()).unwrap();
        assert!(persisted.product_telemetry_enabled);
    }

    #[test]
    fn concurrent_telemetry_transitions_leave_one_committed_choice_and_intact_json() {
        let _guard = telemetry_guard();
        let directory = tempfile::tempdir().unwrap();
        let path = directory.path().join("settings.json");
        let store = Arc::new(Store::from_file(path.clone()).unwrap());
        store
            .set_extra("preserved".into(), "x".repeat(64 * 1024))
            .unwrap();
        store.set_product_telemetry(false).unwrap();
        let running = Arc::new(AtomicBool::new(true));
        let reader_running = running.clone();
        let reader_path = path.clone();
        let reader = std::thread::spawn(move || {
            let mut reads = 0;
            while reader_running.load(Ordering::Acquire) {
                let data = std::fs::read(&reader_path).unwrap();
                let parsed: AppSettings = serde_json::from_slice(&data)
                    .expect("settings writes must never expose partial JSON");
                assert_eq!(parsed.extras["preserved"].len(), 64 * 1024);
                reads += 1;
            }
            reads
        });
        let barrier = Arc::new(std::sync::Barrier::new(4));
        let mut writers = Vec::new();
        for thread in 0..4 {
            let store = store.clone();
            let barrier = barrier.clone();
            writers.push(std::thread::spawn(move || {
                barrier.wait();
                for change in 0..20 {
                    store
                        .set_product_telemetry((thread + change) % 2 == 0)
                        .unwrap();
                }
            }));
        }
        for writer in writers {
            writer.join().unwrap();
        }
        running.store(false, Ordering::Release);
        assert!(reader.join().unwrap() > 0);
        let data = store.get();
        let persisted: AppSettings = serde_json::from_slice(&std::fs::read(path).unwrap()).unwrap();
        assert_eq!(
            data.product_telemetry_enabled,
            persisted.product_telemetry_enabled
        );
        assert_eq!(
            data.product_telemetry_enabled,
            ::server_config::DYNAMIC_TELEMETRY_ENABLED.load(Ordering::Relaxed)
        );
        assert_eq!(
            std::fs::read_dir(directory.path()).unwrap().count(),
            1,
            "successful staging must not leak files"
        );
    }
    #[test]
    fn telemetry_consent_requires_persistent_storage() {
        let _guard = telemetry_guard();
        let store = Store::new();
        assert!(store.set_product_telemetry(true).is_err());
        assert!(!store.get().product_telemetry_enabled);
        assert!(!::server_config::DYNAMIC_TELEMETRY_ENABLED.load(Ordering::Relaxed));
    }
}
