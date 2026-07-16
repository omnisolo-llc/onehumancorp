CREATE TABLE IF NOT EXISTS sync_conflict_queue (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    event_id TEXT NOT NULL,
    entity_id TEXT NOT NULL,
    entity_type TEXT NOT NULL,
    base_version BIGINT NOT NULL,
    current_version BIGINT NOT NULL,
    payload JSONB NOT NULL,
    status TEXT NOT NULL DEFAULT 'PENDING',
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

ALTER TABLE sync_conflict_queue ENABLE ROW LEVEL SECURITY;
CREATE POLICY tenant_isolation_sync_conflict_queue ON sync_conflict_queue
    USING (tenant_id = current_setting('app.current_tenant', TRUE))
WITH CHECK (tenant_id = current_setting('app.current_tenant', true)::uuid);

CREATE TABLE IF NOT EXISTS test_sync_entities (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    version BIGINT NOT NULL DEFAULT 1
);

ALTER TABLE test_sync_entities ENABLE ROW LEVEL SECURITY;
CREATE POLICY tenant_isolation_test_sync_entities ON test_sync_entities
    USING (tenant_id = current_setting('app.current_tenant', TRUE))
WITH CHECK (tenant_id = current_setting('app.current_tenant', true)::uuid);
