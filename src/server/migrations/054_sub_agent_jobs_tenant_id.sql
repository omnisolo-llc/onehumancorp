-- 054_sub_agent_jobs_tenant_id.sql

ALTER TABLE sub_agent_jobs ADD COLUMN IF NOT EXISTS organization_id VARCHAR NOT NULL DEFAULT 'system';

ALTER TABLE sub_agent_jobs ENABLE ROW LEVEL SECURITY;

CREATE POLICY tenant_isolation_sub_agent_jobs ON sub_agent_jobs
    USING (organization_id = current_setting('app.current_tenant', true) OR current_setting('app.current_tenant', true) = 'system');
