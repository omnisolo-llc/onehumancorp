-- +goose Up
-- Migration 134: Create crdt_deltas table

CREATE TABLE IF NOT EXISTS crdt_deltas (
    tenant_id VARCHAR(255) NOT NULL,
    id VARCHAR(255) NOT NULL,
    entity_id VARCHAR(255) NOT NULL,
    data JSONB NOT NULL,
    updated_at TIMESTAMPTZ NOT NULL,
    synced_to_cloud BOOLEAN NOT NULL DEFAULT FALSE,
    PRIMARY KEY (tenant_id, id)
);

CREATE INDEX IF NOT EXISTS idx_crdt_deltas_updated_at ON crdt_deltas(updated_at);

ALTER TABLE crdt_deltas ENABLE ROW LEVEL SECURITY;
ALTER TABLE crdt_deltas FORCE ROW LEVEL SECURITY;

DROP POLICY IF EXISTS tenant_isolation_crdt_deltas ON crdt_deltas;
CREATE POLICY tenant_isolation_crdt_deltas
ON crdt_deltas
FOR ALL
USING (tenant_id = current_setting('app.current_tenant', true))
WITH CHECK (tenant_id = current_setting('app.current_tenant', true));

-- +goose Down
DROP TABLE IF EXISTS crdt_deltas;
