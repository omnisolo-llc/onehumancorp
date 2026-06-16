CREATE TABLE IF NOT EXISTS crdt_deltas (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    entity_id TEXT NOT NULL,
    data JSONB NOT NULL,
    updated_at TIMESTAMPTZ NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE UNIQUE INDEX IF NOT EXISTS idx_crdt_deltas_tenant_entity ON crdt_deltas(tenant_id, entity_id);

ALTER TABLE crdt_deltas ENABLE ROW LEVEL SECURITY;

DROP POLICY IF EXISTS tenant_isolation_crdt_deltas ON crdt_deltas;

CREATE POLICY tenant_isolation_crdt_deltas
ON crdt_deltas
USING (tenant_id = current_setting('app.current_tenant', true))
WITH CHECK (tenant_id = current_setting('app.current_tenant', true));
