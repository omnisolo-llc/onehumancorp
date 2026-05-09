use std::collections::HashMap;
use std::sync::RwLock;
use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};
use flate2::write::GzEncoder;
use flate2::read::GzDecoder;
use flate2::Compression;
use std::io::{Read, Write};
use base64::{Engine as _, engine::general_purpose};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[allow(dead_code)]
pub struct UltraPlan {
    pub id: String,
    pub mission_id: String,
    pub status: String,
    pub state_machine: serde_json::Value,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[allow(dead_code)]
#[allow(dead_code)]
pub struct UltraPlanManager {
    plans: RwLock<HashMap<String, UltraPlan>>,
}

#[allow(dead_code)]
impl UltraPlanManager {
    pub fn new() -> Self {
        UltraPlanManager {
            plans: RwLock::new(HashMap::new()),
        }
    }

    pub fn create_plan(&self, mission_id: String, state_machine: serde_json::Value) -> Result<UltraPlan, String> {
        let id = uuid::Uuid::new_v4().to_string();
        let now = Utc::now();
        
        let state_machine_str = serde_json::to_string(&state_machine).map_err(|e| e.to_string())?;
        let compressed_base64 = compress_ultraplan_data(state_machine_str.as_bytes())?;
        let compressed_state_machine = serde_json::json!({
            "_compressed_base64": compressed_base64
        });

        let plan = UltraPlan {
            id: id.clone(),
            mission_id,
            status: "DELIBERATING".to_string(),
            state_machine: compressed_state_machine,
            created_at: now,
            updated_at: now,
        };
        
        let mut plans = self.plans.write().unwrap();
        plans.insert(id, plan.clone());
        
        Ok(plan)
    }

    pub fn get_ultra_plan(&self, plan_id: &str) -> Result<UltraPlan, String> {
        let plans = self.plans.read().unwrap();
        let mut plan = plans.get(plan_id).cloned().ok_or_else(|| "ultra plan not found".to_string())?;

        if let Some(obj) = plan.state_machine.as_object() {
            if let Some(compressed) = obj.get("_compressed_base64") {
                if let Some(base64_str) = compressed.as_str() {
                    let decompressed_bytes = decompress_ultraplan_data(base64_str)?;
                    let decompressed_str = String::from_utf8(decompressed_bytes).map_err(|e| e.to_string())?;
                    plan.state_machine = serde_json::from_str(&decompressed_str).map_err(|e| e.to_string())?;
                }
            }
        }

        Ok(plan)
    }

    pub fn update_plan_status(&self, plan_id: &str, status: &str, state_machine: Option<serde_json::Value>) -> Result<(), String> {
        let mut plans = self.plans.write().unwrap();
        if let Some(plan) = plans.get_mut(plan_id) {
            plan.status = status.to_string();
            plan.updated_at = Utc::now();

            if let Some(sm) = state_machine {
                let state_machine_str = serde_json::to_string(&sm).map_err(|e| e.to_string())?;
                let compressed_base64 = compress_ultraplan_data(state_machine_str.as_bytes())?;
                plan.state_machine = serde_json::json!({
                    "_compressed_base64": compressed_base64
                });
            }
            Ok(())
        } else {
            Err("ultra plan not found".to_string())
        }
    }

}

#[allow(dead_code)]
pub fn compress_ultraplan_data(data: &[u8]) -> Result<String, String> {
    let mut encoder = GzEncoder::new(Vec::new(), Compression::default());
    encoder.write_all(data).map_err(|e: std::io::Error| e.to_string())?;
    let compressed = encoder.finish().map_err(|e: std::io::Error| e.to_string())?;
    Ok(general_purpose::STANDARD.encode(compressed))
}

#[allow(dead_code)]
pub fn decompress_ultraplan_data(base64_str: &str) -> Result<Vec<u8>, String> {
    let decoded = general_purpose::STANDARD.decode(base64_str).map_err(|e| e.to_string())?;
    let mut decoder = GzDecoder::new(&decoded[..]);
    let mut decompressed = Vec::new();
    decoder.read_to_end(&mut decompressed).map_err(|e: std::io::Error| e.to_string())?;
    Ok(decompressed)
}

impl Default for UltraPlanManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_compress_decompress() {
        let data = b"hello world hello world hello world";
        let compressed = compress_ultraplan_data(data).unwrap();
        let decompressed = decompress_ultraplan_data(&compressed).unwrap();
        assert_eq!(data.to_vec(), decompressed);
    }

    #[test]
    fn test_create_plan() {
        let manager = UltraPlanManager::new();
        let state_machine = serde_json::json!({"phase": "INIT"});
        let plan = manager.create_plan("mission1".to_string(), state_machine.clone()).unwrap();
        
        assert_eq!(plan.mission_id, "mission1");
        assert_eq!(plan.status, "DELIBERATING");
        
        // Ensure it's stored compressed in the raw struct
        let is_compressed = plan.state_machine.as_object().unwrap().contains_key("_compressed_base64");
        assert!(is_compressed);

        // But get_ultra_plan decompresses it seamlessly
        let fetched = manager.get_ultra_plan(&plan.id).unwrap();
        assert_eq!(fetched.id, plan.id);
        assert_eq!(fetched.state_machine, state_machine);
    }

    #[test]
    fn test_update_plan_status() {
        let manager = UltraPlanManager::new();
        let state_machine = serde_json::json!({"phase": "INIT"});
        let plan = manager.create_plan("mission1".to_string(), state_machine.clone()).unwrap();

        let updated_sm = serde_json::json!({"phase": "DONE"});
        manager.update_plan_status(&plan.id, "COMPLETED", Some(updated_sm.clone())).unwrap();

        let fetched = manager.get_ultra_plan(&plan.id).unwrap();
        assert_eq!(fetched.status, "COMPLETED");
        assert_eq!(fetched.state_machine, updated_sm);
    }
}
