use std::collections::HashMap;
use std::sync::RwLock;
use serde::{Serialize, Deserialize};
use std::path::PathBuf;

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

impl AppSettings {
    pub fn default() -> Self {
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

impl Store {
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

        Ok(Store {
            data: RwLock::new(data),
            path: Some(path),
        })
    }

    pub fn save(&self) -> Result<(), String> {
        let data = self.data.read().unwrap();
        let path = match &self.path {
            Some(p) => p,
            None => return Ok(()), // In-memory only
        };

        if let Some(parent) = path.parent() {
             std::fs::create_dir_all(parent).map_err(|e| e.to_string())?;
        }

        let content = serde_json::to_string_pretty(&*data).map_err(|e| e.to_string())?;
        
        // Simple write for now, not atomic!
        std::fs::write(path, content).map_err(|e| e.to_string())?;

        Ok(())
    }

    pub fn get(&self) -> AppSettings {
        self.data.read().unwrap().clone()
    }

    pub fn set_extra(&self, key: String, value: String) -> Result<(), String> {
        let mut data = self.data.write().unwrap();
        data.extras.insert(key, value);
        drop(data);
        self.save()
    }

    pub fn set_sms_preferences(&self, phone: String, urgent_booking: bool, failed_payment: bool, new_order: bool) -> Result<(), String> {
        let mut data = self.data.write().unwrap();
        data.sms_critical_phone = Some(phone);
        data.sms_alert_urgent_booking = urgent_booking;
        data.sms_alert_failed_payment = failed_payment;
        data.sms_alert_new_order = new_order;
        drop(data);
        self.save()
    }

    pub fn set_delivery_settings(&self, enabled: bool, radius: Option<f64>, fee: Option<f64>) -> Result<(), String> {
        let mut data = self.data.write().unwrap();
        data.delivery_enabled = enabled;
        data.delivery_radius = radius;
        data.delivery_fee = fee;
        drop(data);
        self.save()
    }

    pub fn set_voice_settings(&self, enabled: bool, number: Option<String>, persona: Option<String>, instructions: Option<String>) -> Result<(), String> {
        let mut data = self.data.write().unwrap();
        data.voice_receptionist_enabled = enabled;
        data.voice_receptionist_number = number;
        data.voice_receptionist_persona = persona;
        data.voice_receptionist_instructions = instructions;
        drop(data);
        self.save()
    }

    pub fn set_product_telemetry(&self, enabled: bool) -> Result<(), String> {
        let mut data = self.data.write().unwrap();
        data.product_telemetry_enabled = enabled;
        drop(data);
        self.save()
    }
}

impl Default for Store {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_settings_default() {
        let settings = AppSettings::default();
        assert_eq!(settings.listen_addr, "0.0.0.0:18789");
        assert_eq!(settings.db_path, Some("ohc.db".to_string()));
        assert_eq!(settings.voice_receptionist_enabled, false);
        assert_eq!(settings.voice_receptionist_number, None);
        assert_eq!(settings.voice_receptionist_persona, Some("Friendly".to_string()));
        assert_eq!(settings.voice_receptionist_instructions, None);
    }

    #[test]
    fn test_store_save_and_load() {
        let file_path = PathBuf::from("test_settings.json");
        
        // Clean up before test
        if file_path.exists() {
            std::fs::remove_file(&file_path).unwrap();
        }
        
        let store = Store::from_file(file_path.clone()).unwrap();
        store.set_extra("key1".to_string(), "value1".to_string()).unwrap();
        
        assert!(file_path.exists());
        
        let store2 = Store::from_file(file_path.clone()).unwrap();
        let settings = store2.get();
        assert_eq!(settings.extras.get("key1").unwrap(), "value1");
        
        // Clean up after test
        std::fs::remove_file(&file_path).unwrap();
    }

    #[test]
    fn test_store_from_file_errors() {
        // Bad JSON
        let mut file_path = std::env::temp_dir();
        file_path.push("bad_settings.json");
        std::fs::write(&file_path, "{bad json").unwrap();
        
        let result = Store::from_file(file_path.clone());
        assert!(result.is_err());
        
        std::fs::remove_file(&file_path).unwrap();

        // Unreadable file (directory)
        let dir_path = std::env::temp_dir().join("some_dir");
        std::fs::create_dir(&dir_path).unwrap();
        let result = Store::from_file(dir_path.clone());
        assert!(result.is_err());
        std::fs::remove_dir(&dir_path).unwrap();
    }

    #[test]
    fn test_store_save_errors() {
        let store = Store {
            data: RwLock::new(AppSettings::default()),
            path: Some(PathBuf::from("/root/unauthorized/file.json")),
        };
        let result = store.save();
        assert!(result.is_err());
    }
}
