CREATE TABLE IF NOT EXISTS crdt_deltas (
    id TEXT NOT NULL,
    tenant_id TEXT NOT NULL,
    entity_id TEXT NOT NULL,
    data TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    synced_to_cloud BOOLEAN DEFAULT false,
    PRIMARY KEY (tenant_id, id)
);

CREATE TABLE IF NOT EXISTS crdt_conflict_queue (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    crdt_id TEXT NOT NULL,
    entity_id TEXT NOT NULL,
    local_data TEXT NOT NULL,
    cloud_data TEXT NOT NULL,
    resolved BOOLEAN DEFAULT false,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    resolved_at TIMESTAMPTZ
);

ALTER TABLE crdt_deltas ENABLE ROW LEVEL SECURITY;
CREATE POLICY tenant_isolation_crdt_deltas ON crdt_deltas USING (tenant_id::text = current_setting('app.current_tenant', true));

ALTER TABLE crdt_conflict_queue ENABLE ROW LEVEL SECURITY;
CREATE POLICY tenant_isolation_crdt_conflict_queue ON crdt_conflict_queue USING (tenant_id::text = current_setting('app.current_tenant', true));
