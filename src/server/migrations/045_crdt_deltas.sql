CREATE TABLE IF NOT EXISTS crdt_deltas (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    entity_id TEXT,
    data TEXT,
    updated_at TEXT,
    synced_to_cloud BOOLEAN,
    UNIQUE(tenant_id, id)
);
ALTER TABLE crdt_deltas ENABLE ROW LEVEL SECURITY;
CREATE POLICY crdt_deltas_isolation_policy ON crdt_deltas
    USING (tenant_id = current_setting('app.current_tenant', true))
    WITH CHECK (tenant_id = current_setting('app.current_tenant', true));
