CREATE TABLE IF NOT EXISTS crdt_deltas (
    tenant_id TEXT NOT NULL,
    id TEXT NOT NULL,
    entity_id TEXT NOT NULL,
    data TEXT NOT NULL,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    synced_to_cloud BOOLEAN NOT NULL DEFAULT FALSE,
    PRIMARY KEY (tenant_id, id)
);

CREATE INDEX IF NOT EXISTS idx_crdt_deltas_updated_at ON crdt_deltas(updated_at);
CREATE INDEX IF NOT EXISTS idx_crdt_deltas_entity_id ON crdt_deltas(entity_id);

ALTER TABLE crdt_deltas ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_crdt_deltas ON crdt_deltas;
CREATE POLICY tenant_isolation_crdt_deltas ON crdt_deltas USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
