ALTER TABLE shared_tasks_decomposition ADD COLUMN IF NOT EXISTS assigned_agent_id TEXT;
ALTER TABLE shared_tasks_decomposition ADD COLUMN IF NOT EXISTS locked_until TIMESTAMPTZ;
ALTER TABLE shared_tasks_decomposition ADD COLUMN IF NOT EXISTS updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP;

CREATE INDEX IF NOT EXISTS idx_shared_tasks_decomp_org_id ON shared_tasks_decomposition(organization_id);
CREATE INDEX IF NOT EXISTS idx_shared_tasks_decomp_status ON shared_tasks_decomposition(status);

ALTER TABLE shared_tasks_decomposition ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_shared_tasks_decomp ON shared_tasks_decomposition;
CREATE POLICY tenant_isolation_shared_tasks_decomp ON shared_tasks_decomposition USING (organization_id::text = current_setting('app.current_tenant', true));
