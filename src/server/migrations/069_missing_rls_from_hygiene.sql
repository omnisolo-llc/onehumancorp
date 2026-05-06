ALTER TABLE users ENABLE ROW LEVEL SECURITY;
ALTER TABLE shared_tasks_v4 ENABLE ROW LEVEL SECURITY;

-- Default RLS Policies
CREATE POLICY IF NOT EXISTS tenant_isolation_users ON users
    USING (organization_id = current_setting('app.current_tenant', true))
    WITH CHECK (organization_id = current_setting('app.current_tenant', true));

CREATE POLICY IF NOT EXISTS tenant_isolation_shared_tasks_v4 ON shared_tasks_v4
    USING (organization_id = current_setting('app.current_tenant', true))
    WITH CHECK (organization_id = current_setting('app.current_tenant', true));
