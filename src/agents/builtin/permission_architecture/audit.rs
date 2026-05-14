use std::collections::HashMap;
use std::sync::RwLock;
use async_trait::async_trait;

use super::models::AuditLogEntry;

#[async_trait]
pub trait AuditLogger: std::fmt::Debug + Send + Sync {
    async fn log(&self, entry: AuditLogEntry) -> Result<(), String>;
    async fn get_logs(&self, session_id: &str) -> Result<Vec<AuditLogEntry>, String>;
}

#[derive(Debug)]
pub struct InMemoryAuditLogger {
    logs: RwLock<HashMap<String, Vec<AuditLogEntry>>>,
}

impl InMemoryAuditLogger {
    pub fn new() -> Self {
        Self {
            logs: RwLock::new(HashMap::new()),
        }
    }
}

#[async_trait]
impl AuditLogger for InMemoryAuditLogger {
    async fn log(&self, entry: AuditLogEntry) -> Result<(), String> {
        let mut map = self.logs.write().unwrap();
        map.entry(entry.session_id.clone()).or_default().push(entry);
        Ok(())
    }

    async fn get_logs(&self, session_id: &str) -> Result<Vec<AuditLogEntry>, String> {
        let map = self.logs.read().unwrap();
        Ok(map.get(session_id).cloned().unwrap_or_default())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::permission_architecture::models::PermissionMode;

    #[tokio::test]
    async fn test_in_memory_audit_logger() {
        let logger = InMemoryAuditLogger::new();
        let entry = AuditLogEntry {
            id: "audit-1".to_string(),
            session_id: "sess-1".to_string(),
            tool_name: "test_tool".to_string(),
            action: "requested".to_string(),
            timestamp: 1000,
            mode: PermissionMode::Restrictive,
            details: "Tool requested".to_string(),
        };

        assert!(logger.log(entry.clone()).await.is_ok());

        let logs = logger.get_logs("sess-1").await.unwrap();
        assert_eq!(logs.len(), 1);
        assert_eq!(logs[0].id, "audit-1");

        let logs_empty = logger.get_logs("sess-2").await.unwrap();
        assert!(logs_empty.is_empty());
    }
}
