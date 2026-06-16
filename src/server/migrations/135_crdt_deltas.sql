CREATE TABLE IF NOT EXISTS crdt_deltas (
    tenant_id TEXT NOT NULL,
    id TEXT NOT NULL,
    entity_id TEXT NOT NULL,
    data TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    synced_to_cloud BOOLEAN DEFAULT false,
    PRIMARY KEY (tenant_id, id)
);

ALTER TABLE crdt_deltas ENABLE ROW LEVEL SECURITY;

DROP POLICY IF EXISTS tenant_isolation_crdt_deltas ON crdt_deltas;
CREATE POLICY tenant_isolation_crdt_deltas
ON crdt_deltas
USING (tenant_id = current_setting('app.current_tenant', true))
WITH CHECK (tenant_id = current_setting('app.current_tenant', true));
