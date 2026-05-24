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
        
        let plan = UltraPlan {
            id: id.clone(),
            mission_id,
            status: "DELIBERATING".to_string(),
            state_machine,
            created_at: now,
            updated_at: now,
        };
        
        let mut plans = self.plans.write().unwrap();
        plans.insert(id, plan.clone());
        
        Ok(plan)
    }

    pub fn get_ultra_plan(&self, plan_id: &str) -> Result<UltraPlan, String> {
        let plans = self.plans.read().unwrap();
        plans.get(plan_id).cloned().ok_or_else(|| "ultra plan not found".to_string())
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
        assert_eq!(plan.state_machine, state_machine);
        
        let fetched = manager.get_ultra_plan(&plan.id).unwrap();
        assert_eq!(fetched.id, plan.id);
    }
}
