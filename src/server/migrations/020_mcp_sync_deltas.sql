CREATE TABLE IF NOT EXISTS mcp_sync_deltas (
    id VARCHAR(255) PRIMARY KEY,
    entity_type VARCHAR(255) NOT NULL,
    entity_id VARCHAR(255) NOT NULL,
    payload TEXT NOT NULL,
    updated_at BIGINT NOT NULL
);
