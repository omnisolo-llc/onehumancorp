-- 064_agent_violations_pg.sql
-- PostgreSQL specific extensions (e.g. JSONB instead of TEXT for details)
ALTER TABLE agent_violations ALTER COLUMN details TYPE JSONB USING details::JSONB;

ALTER TABLE agent_violations ENABLE ROW LEVEL SECURITY;

CREATE POLICY tenant_isolation_agent_violations ON agent_violations
    USING (tenant_id = current_setting('app.current_tenant', true));
