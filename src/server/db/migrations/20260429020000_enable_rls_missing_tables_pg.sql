-- +goose Up
-- +goose StatementBegin
ALTER TABLE shared_tasks ENABLE ROW LEVEL SECURITY;
CREATE POLICY "tenant_isolation_policy" ON shared_tasks USING (organization_id = current_setting('app.current_tenant')::varchar);

ALTER TABLE shared_tasks_v2 ENABLE ROW LEVEL SECURITY;
CREATE POLICY "tenant_isolation_policy" ON shared_tasks_v2 USING (organization_id = current_setting('app.current_tenant')::varchar);

ALTER TABLE shared_tasks_v4 ENABLE ROW LEVEL SECURITY;
CREATE POLICY "tenant_isolation_policy" ON shared_tasks_v4 USING (organization_id = current_setting('app.current_tenant')::varchar);

ALTER TABLE shared_tasks_master ENABLE ROW LEVEL SECURITY;
CREATE POLICY "tenant_isolation_policy" ON shared_tasks_master USING (organization_id = current_setting('app.current_tenant')::varchar);

ALTER TABLE shared_tasks_decomposition ENABLE ROW LEVEL SECURITY;
CREATE POLICY "tenant_isolation_policy" ON shared_tasks_decomposition USING (organization_id = current_setting('app.current_tenant')::varchar);
-- +goose StatementEnd

-- +goose Down
-- +goose StatementBegin
DROP POLICY IF EXISTS "tenant_isolation_policy" ON shared_tasks_decomposition;
ALTER TABLE shared_tasks_decomposition DISABLE ROW LEVEL SECURITY;

DROP POLICY IF EXISTS "tenant_isolation_policy" ON shared_tasks_master;
ALTER TABLE shared_tasks_master DISABLE ROW LEVEL SECURITY;

DROP POLICY IF EXISTS "tenant_isolation_policy" ON shared_tasks_v4;
ALTER TABLE shared_tasks_v4 DISABLE ROW LEVEL SECURITY;

DROP POLICY IF EXISTS "tenant_isolation_policy" ON shared_tasks_v2;
ALTER TABLE shared_tasks_v2 DISABLE ROW LEVEL SECURITY;

DROP POLICY IF EXISTS "tenant_isolation_policy" ON shared_tasks;
ALTER TABLE shared_tasks DISABLE ROW LEVEL SECURITY;
-- +goose StatementEnd
