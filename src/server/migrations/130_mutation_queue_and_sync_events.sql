CREATE TABLE IF NOT EXISTS mutation_queue (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    action_type TEXT NOT NULL,
    payload TEXT NOT NULL,
    status TEXT NOT NULL DEFAULT 'pending',
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS sync_events (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    batch_id TEXT,
    action_type TEXT NOT NULL,
    payload TEXT NOT NULL,
    synced_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

ALTER TABLE mutation_queue ENABLE ROW LEVEL SECURITY;
ALTER TABLE sync_events ENABLE ROW LEVEL SECURITY;

CREATE POLICY tenant_isolation_mutation_queue ON mutation_queue
    USING (tenant_id = current_setting('app.current_tenant', TRUE))
WITH CHECK (tenant_id = current_setting('app.current_tenant', true)::uuid);

CREATE POLICY tenant_isolation_sync_events ON sync_events
    USING (tenant_id = current_setting('app.current_tenant', TRUE))
WITH CHECK (tenant_id = current_setting('app.current_tenant', true)::uuid);
