-- Create mcp_sync_deltas table for offline sync
CREATE TABLE IF NOT EXISTS mcp_sync_deltas (
    id TEXT PRIMARY KEY,
    entity_type TEXT NOT NULL,
    entity_id TEXT NOT NULL,
    payload TEXT NOT NULL,
    updated_at BIGINT NOT NULL
);

CREATE INDEX IF NOT EXISTS idx_mcp_sync_deltas_updated_at ON mcp_sync_deltas(updated_at);
