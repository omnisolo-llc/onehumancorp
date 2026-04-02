CREATE TABLE IF NOT EXISTS local_cloud_sync_log (
    sync_id TEXT PRIMARY KEY,
    memory_id TEXT NOT NULL,
    cloud_mission_id TEXT,
    synced_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (memory_id) REFERENCES swarm_memory(key) ON DELETE CASCADE
);
