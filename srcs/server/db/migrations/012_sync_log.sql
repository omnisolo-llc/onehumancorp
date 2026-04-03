-- 012_sync_log.sql
-- Track synced memories for Hybrid MCP RAG Protocol

CREATE TABLE IF NOT EXISTS local_cloud_sync_log (
    sync_id TEXT PRIMARY KEY,
    memory_id TEXT NOT NULL,
    cloud_mission_id TEXT,
    synced_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (memory_id) REFERENCES swarm_memory(key) ON DELETE CASCADE
);
