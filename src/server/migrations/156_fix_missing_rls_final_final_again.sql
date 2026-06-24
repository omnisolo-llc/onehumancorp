-- +goose Up

-- 054_shared_task_dependencies.sql
ALTER TABLE shared_task_dependencies ADD COLUMN IF NOT EXISTS organization_id TEXT;
CREATE INDEX IF NOT EXISTS idx_shared_task_dependencies_org_id ON shared_task_dependencies(organization_id);
ALTER TABLE shared_task_dependencies ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_shared_task_dependencies ON shared_task_dependencies;
CREATE POLICY tenant_isolation_shared_task_dependencies ON shared_task_dependencies
    USING (organization_id::text = current_setting('app.current_tenant', true))
    WITH CHECK (organization_id::text = current_setting('app.current_tenant', true));

-- 002_missing_tables.sql meeting_rooms
ALTER TABLE meeting_rooms ADD COLUMN IF NOT EXISTS tenant_id TEXT;
ALTER TABLE meeting_rooms ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_meeting_rooms ON meeting_rooms;
CREATE POLICY tenant_isolation_meeting_rooms ON meeting_rooms
    USING (tenant_id::text = current_setting('app.current_tenant', true))
    WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

-- 002_missing_tables.sql meeting_transcripts
ALTER TABLE meeting_transcripts ADD COLUMN IF NOT EXISTS tenant_id TEXT;
ALTER TABLE meeting_transcripts ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_meeting_transcripts ON meeting_transcripts;
CREATE POLICY tenant_isolation_meeting_transcripts ON meeting_transcripts
    USING (tenant_id::text = current_setting('app.current_tenant', true))
    WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

-- 021_epics_tasks.sql
ALTER TABLE epics ADD COLUMN IF NOT EXISTS tenant_id TEXT;
ALTER TABLE legacy_tasks ADD COLUMN IF NOT EXISTS tenant_id TEXT;
ALTER TABLE tasks ADD COLUMN IF NOT EXISTS tenant_id TEXT;

ALTER TABLE epics ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_epics ON epics;
CREATE POLICY tenant_isolation_epics ON epics
    USING (tenant_id::text = current_setting('app.current_tenant', true))
    WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

ALTER TABLE tasks ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_tasks ON tasks;
CREATE POLICY tenant_isolation_tasks ON tasks
    USING (tenant_id::text = current_setting('app.current_tenant', true))
    WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

-- 007_telemetry.sql
ALTER TABLE telemetry_buffer ADD COLUMN IF NOT EXISTS tenant_id TEXT;
ALTER TABLE telemetry_buffer ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_telemetry_buffer ON telemetry_buffer;
CREATE POLICY tenant_isolation_telemetry_buffer ON telemetry_buffer
    USING (tenant_id::text = current_setting('app.current_tenant', true))
    WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

-- 128_quote_requests.sql
ALTER TABLE estimate_line_items ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_estimate_line_items ON estimate_line_items;
CREATE POLICY tenant_isolation_estimate_line_items ON estimate_line_items
    USING (estimate_id IN (SELECT id FROM estimates WHERE tenant_id::text = current_setting('app.current_tenant', true)))
    WITH CHECK (estimate_id IN (SELECT id FROM estimates WHERE tenant_id::text = current_setting('app.current_tenant', true)));

-- 051_task_dependencies.sql
ALTER TABLE task_dependencies ADD COLUMN IF NOT EXISTS tenant_id TEXT;
ALTER TABLE task_dependencies ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_task_dependencies ON task_dependencies;
CREATE POLICY tenant_isolation_task_dependencies ON task_dependencies
    USING (task_id IN (SELECT id FROM legacy_tasks WHERE tenant_id::text = current_setting('app.current_tenant', true)))
    WITH CHECK (task_id IN (SELECT id FROM legacy_tasks WHERE tenant_id::text = current_setting('app.current_tenant', true)));

-- 023_embedding_cache_sync.sql
ALTER TABLE embedding_cache ADD COLUMN IF NOT EXISTS tenant_id TEXT;
ALTER TABLE embedding_cache ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_embedding_cache ON embedding_cache;
CREATE POLICY tenant_isolation_embedding_cache ON embedding_cache
    USING (tenant_id::text = current_setting('app.current_tenant', true))
    WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

-- 002_missing_tables.sql swarm_truth_embeddings
ALTER TABLE swarm_truth_embeddings ADD COLUMN IF NOT EXISTS tenant_id TEXT;
ALTER TABLE swarm_truth_embeddings ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_swarm_truth_embeddings ON swarm_truth_embeddings;
CREATE POLICY tenant_isolation_swarm_truth_embeddings ON swarm_truth_embeddings
    USING (tenant_id::text = current_setting('app.current_tenant', true))
    WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

-- 002_missing_tables.sql swarm_tasks
ALTER TABLE swarm_tasks ADD COLUMN IF NOT EXISTS tenant_id TEXT;
ALTER TABLE swarm_tasks ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_swarm_tasks ON swarm_tasks;
CREATE POLICY tenant_isolation_swarm_tasks ON swarm_tasks
    USING (tenant_id::text = current_setting('app.current_tenant', true))
    WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

-- +goose Down
