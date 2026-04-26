CREATE TABLE IF NOT EXISTS hybrid_mcp_sync_queue (
    id VARCHAR(36) PRIMARY KEY,
    tool_name VARCHAR(255) NOT NULL,
    payload TEXT NOT NULL,
    status VARCHAR(50) NOT NULL DEFAULT 'PENDING',
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    synced_at TIMESTAMP,
    error_message TEXT
);
