CREATE TABLE IF NOT EXISTS agent_kv_store (
    tenant_id VARCHAR(255) NOT NULL,
    kv_key VARCHAR(255) NOT NULL,
    kv_value TEXT NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY (tenant_id, kv_key)
);
