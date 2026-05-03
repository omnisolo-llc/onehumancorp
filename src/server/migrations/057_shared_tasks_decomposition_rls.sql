-- Enable Row Level Security and corresponding policies on shared_tasks_decomposition table.

ALTER TABLE shared_tasks_decomposition ENABLE ROW LEVEL SECURITY;
CREATE POLICY tenant_isolation_shared_tasks_decomposition ON shared_tasks_decomposition USING (organization_id = current_setting('app.current_tenant', true) OR current_setting('app.current_tenant', true) = 'system' OR current_setting('app.current_tenant', true) = '');
