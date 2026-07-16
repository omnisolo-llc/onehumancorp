CREATE TABLE IF NOT EXISTS entity_versions (
    tenant_id TEXT NOT NULL,
    entity_type TEXT NOT NULL,
    entity_id TEXT NOT NULL,
    current_version BIGINT NOT NULL DEFAULT 1,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY (tenant_id, entity_type, entity_id)
);

CREATE TABLE IF NOT EXISTS sync_events (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    entity_type TEXT NOT NULL,
    entity_id TEXT NOT NULL,
    action_type TEXT NOT NULL,
    payload JSONB NOT NULL DEFAULT '{}'::jsonb,
    base_version BIGINT NOT NULL,
    status TEXT NOT NULL DEFAULT 'PENDING', -- PENDING, APPLIED, CONFLICT, FAILED
    synced_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS conflict_queue (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    sync_event_id TEXT NOT NULL REFERENCES sync_events(id) ON DELETE CASCADE,
    entity_type TEXT NOT NULL,
    entity_id TEXT NOT NULL,
    action_type TEXT NOT NULL,
    payload JSONB NOT NULL,
    base_version BIGINT NOT NULL,
    current_version BIGINT NOT NULL,
    status TEXT NOT NULL DEFAULT 'UNRESOLVED', -- UNRESOLVED, RESOLVED
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

ALTER TABLE entity_versions ENABLE ROW LEVEL SECURITY;
ALTER TABLE sync_events ENABLE ROW LEVEL SECURITY;
ALTER TABLE conflict_queue ENABLE ROW LEVEL SECURITY;

CREATE POLICY tenant_isolation_entity_versions ON entity_versions
    USING (tenant_id = current_setting('app.current_tenant', TRUE))
WITH CHECK (tenant_id = current_setting('app.current_tenant', true)::uuid);

CREATE POLICY tenant_isolation_sync_events ON sync_events
    USING (tenant_id = current_setting('app.current_tenant', TRUE))
WITH CHECK (tenant_id = current_setting('app.current_tenant', true)::uuid);

CREATE POLICY tenant_isolation_conflict_queue ON conflict_queue
    USING (tenant_id = current_setting('app.current_tenant', TRUE))
WITH CHECK (tenant_id = current_setting('app.current_tenant', true)::uuid);
