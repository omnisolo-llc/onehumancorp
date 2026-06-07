use std::fs::OpenOptions;
use std::io::Write;
use chrono::Utc;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditLogEntry {
    pub timestamp: String,
    pub agent_id: String,
    pub tool_name: String,
    pub status: String, // "Approved", "Rejected", "Edited"
    pub reason: String,
}

pub fn log_approval_action(agent_id: &str, tool_name: &str, status: &str, reason: &str, log_path: Option<&str>) {
    let entry = AuditLogEntry {
        timestamp: Utc::now().to_rfc3339(),
        agent_id: agent_id.to_string(),
        tool_name: tool_name.to_string(),
        status: status.to_string(),
        reason: reason.to_string(),
    };

    let log_line = match serde_json::to_string(&entry) {
        Ok(json) => format!("{}\n", json),
        Err(e) => {
            eprintln!("Failed to serialize audit log entry: {}", e);
            return;
        }
    };

    let path = log_path.unwrap_or("/tmp/ohc_audit_log.jsonl");

    if let Ok(mut file) = OpenOptions::new().create(true).append(true).open(path) {
        if let Err(e) = file.write_all(log_line.as_bytes()) {
            eprintln!("Failed to write to audit log: {}", e);
        }
    }
}
