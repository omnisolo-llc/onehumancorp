ALTER TABLE sync_events ADD COLUMN IF NOT EXISTS entity_id TEXT;
ALTER TABLE sync_events ADD COLUMN IF NOT EXISTS base_version BIGINT;

CREATE TABLE IF NOT EXISTS conflict_queue (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    sync_event_id TEXT NOT NULL,
    entity_id TEXT NOT NULL,
    base_version BIGINT,
    payload TEXT NOT NULL,
    status TEXT NOT NULL DEFAULT 'pending',
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);
ALTER TABLE conflict_queue ENABLE ROW LEVEL SECURITY;
CREATE POLICY tenant_isolation_conflict_queue ON conflict_queue
    USING (tenant_id = current_setting('app.current_tenant', TRUE))
    WITH CHECK (tenant_id = current_setting('app.current_tenant', TRUE));

CREATE TABLE IF NOT EXISTS entity_versions (
    entity_id TEXT,
    tenant_id TEXT NOT NULL,
    version BIGINT NOT NULL DEFAULT 1,
    PRIMARY KEY (tenant_id, entity_id)
);
ALTER TABLE entity_versions ENABLE ROW LEVEL SECURITY;
CREATE POLICY tenant_isolation_entity_versions ON entity_versions
    USING (tenant_id = current_setting('app.current_tenant', TRUE))
    WITH CHECK (tenant_id = current_setting('app.current_tenant', TRUE));
