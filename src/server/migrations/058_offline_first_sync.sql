CREATE TABLE IF NOT EXISTS sync_mutation_queue (
    id TEXT PRIMARY KEY,
    organization_id TEXT NOT NULL,
    entity_type TEXT NOT NULL,
    entity_id TEXT NOT NULL,
    mutation_type TEXT NOT NULL,
    payload JSONB NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    synced_to_cloud BOOLEAN NOT NULL DEFAULT FALSE,
    sync_error TEXT,
    last_synced_at TIMESTAMPTZ,
    version BIGINT NOT NULL DEFAULT 1
);

ALTER TABLE sync_mutation_queue ENABLE ROW LEVEL SECURITY;
CREATE POLICY sync_mutation_queue_isolation_policy ON sync_mutation_queue USING (organization_id = current_setting('app.current_tenant'));
