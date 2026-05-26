CREATE TABLE IF NOT EXISTS agent_kv_store (
    tenant_id VARCHAR(255) NOT NULL,
    kv_key VARCHAR(255) NOT NULL,
    kv_value TEXT NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY (tenant_id, kv_key)
);
ALTER TABLE agent_kv_store ENABLE ROW LEVEL SECURITY;
CREATE POLICY tenant_isolation_agent_kv_store ON agent_kv_store USING (tenant_id::text = current_setting('app.current_tenant', true));
