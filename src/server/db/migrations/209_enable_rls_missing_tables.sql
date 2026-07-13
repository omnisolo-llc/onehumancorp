-- +goose Up

-- Fix distributed_locks
ALTER TABLE IF EXISTS distributed_locks ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_distributed_locks ON distributed_locks;
-- For distributed_locks, id includes tenant_id, e.g. ohc:lock:{tenant_id}:{resource_type}:{resource_id}
CREATE POLICY tenant_isolation_distributed_locks ON distributed_locks FOR ALL
    USING (id LIKE 'ohc:lock:' || current_setting('app.current_tenant', true) || ':%')
    WITH CHECK (id LIKE 'ohc:lock:' || current_setting('app.current_tenant', true) || ':%');

-- +goose Down
-- Reverting distributed_locks
DROP POLICY IF EXISTS tenant_isolation_distributed_locks ON distributed_locks;
ALTER TABLE IF EXISTS distributed_locks DISABLE ROW LEVEL SECURITY;
