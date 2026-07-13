-- +goose Up

-- telemetry_buffer
ALTER TABLE IF EXISTS telemetry_buffer ADD COLUMN IF NOT EXISTS tenant_id TEXT NOT NULL DEFAULT 'default_tenant';
ALTER TABLE IF EXISTS telemetry_buffer ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_telemetry_buffer ON telemetry_buffer;
CREATE POLICY tenant_isolation_telemetry_buffer ON telemetry_buffer FOR ALL
    USING (tenant_id = current_setting('app.current_tenant', true))
    WITH CHECK (tenant_id = current_setting('app.current_tenant', true));

-- epics
ALTER TABLE IF EXISTS epics ADD COLUMN IF NOT EXISTS tenant_id TEXT NOT NULL DEFAULT 'default_tenant';
ALTER TABLE IF EXISTS epics ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_epics ON epics;
CREATE POLICY tenant_isolation_epics ON epics FOR ALL
    USING (tenant_id = current_setting('app.current_tenant', true))
    WITH CHECK (tenant_id = current_setting('app.current_tenant', true));

-- tasks
ALTER TABLE IF EXISTS tasks ADD COLUMN IF NOT EXISTS tenant_id TEXT NOT NULL DEFAULT 'default_tenant';
ALTER TABLE IF EXISTS tasks ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_tasks ON tasks;
CREATE POLICY tenant_isolation_tasks ON tasks FOR ALL
    USING (tenant_id = current_setting('app.current_tenant', true))
    WITH CHECK (tenant_id = current_setting('app.current_tenant', true));

-- embedding_cache
ALTER TABLE IF EXISTS embedding_cache ADD COLUMN IF NOT EXISTS tenant_id TEXT NOT NULL DEFAULT 'default_tenant';
ALTER TABLE IF EXISTS embedding_cache ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_embedding_cache ON embedding_cache;
CREATE POLICY tenant_isolation_embedding_cache ON embedding_cache FOR ALL
    USING (tenant_id = current_setting('app.current_tenant', true))
    WITH CHECK (tenant_id = current_setting('app.current_tenant', true));

-- task_dependencies
ALTER TABLE IF EXISTS task_dependencies ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_task_dependencies ON task_dependencies;
CREATE POLICY tenant_isolation_task_dependencies ON task_dependencies FOR ALL
    USING (
        EXISTS (
            SELECT 1 FROM tasks WHERE tasks.id::text = task_dependencies.task_id AND tasks.tenant_id = current_setting('app.current_tenant', true)
        )
    )
    WITH CHECK (
        EXISTS (
            SELECT 1 FROM tasks WHERE tasks.id::text = task_dependencies.task_id AND tasks.tenant_id = current_setting('app.current_tenant', true)
        )
    );

-- shared_task_dependencies
ALTER TABLE IF EXISTS shared_task_dependencies ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_shared_task_dependencies ON shared_task_dependencies;
CREATE POLICY tenant_isolation_shared_task_dependencies ON shared_task_dependencies FOR ALL
    USING (
        EXISTS (
            SELECT 1 FROM shared_tasks WHERE shared_tasks.id = shared_task_dependencies.task_id AND shared_tasks.tenant_id = current_setting('app.current_tenant', true)
        )
    )
    WITH CHECK (
        EXISTS (
            SELECT 1 FROM shared_tasks WHERE shared_tasks.id = shared_task_dependencies.task_id AND shared_tasks.tenant_id = current_setting('app.current_tenant', true)
        )
    );

-- +goose Down
-- Intentionally empty
