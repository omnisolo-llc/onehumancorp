-- 052_sentinel_rls_hardening.sql
-- Lead Sentinel (L7) Hardening: Comprehensive Multi-Tenant Isolation

-- 1. Add organization_id to tables that were missing it for full isolation
ALTER TABLE swarm_tasks ADD COLUMN IF NOT EXISTS organization_id TEXT NOT NULL DEFAULT 'system';
ALTER TABLE llm_completion_cache ADD COLUMN IF NOT EXISTS organization_id TEXT NOT NULL DEFAULT 'system';
ALTER TABLE llm_reason_cache ADD COLUMN IF NOT EXISTS organization_id TEXT NOT NULL DEFAULT 'system';
ALTER TABLE embedding_cache ADD COLUMN IF NOT EXISTS organization_id TEXT NOT NULL DEFAULT 'system';
ALTER TABLE swarm_long_term_memory ADD COLUMN IF NOT EXISTS organization_id TEXT NOT NULL DEFAULT 'system';

-- 2. Enable RLS on these tables (if not already enabled)
ALTER TABLE swarm_tasks ENABLE ROW LEVEL SECURITY;
ALTER TABLE llm_completion_cache ENABLE ROW LEVEL SECURITY;
ALTER TABLE llm_reason_cache ENABLE ROW LEVEL SECURITY;
ALTER TABLE embedding_cache ENABLE ROW LEVEL SECURITY;
ALTER TABLE swarm_long_term_memory ENABLE ROW LEVEL SECURITY;
ALTER TABLE onboarding_state ENABLE ROW LEVEL SECURITY;

-- 3. Create Isolation Policies
-- We use the current_setting('app.current_tenant', true) convention established in 049_missing_rls.sql

CREATE POLICY tenant_isolation_swarm_tasks ON swarm_tasks
USING (organization_id = current_setting('app.current_tenant', true) OR current_setting('app.current_tenant', true) = 'system');

CREATE POLICY tenant_isolation_llm_completion_cache ON llm_completion_cache
USING (organization_id = current_setting('app.current_tenant', true) OR current_setting('app.current_tenant', true) = 'system');

CREATE POLICY tenant_isolation_llm_reason_cache ON llm_reason_cache
USING (organization_id = current_setting('app.current_tenant', true) OR current_setting('app.current_tenant', true) = 'system');

CREATE POLICY tenant_isolation_embedding_cache ON embedding_cache
USING (organization_id = current_setting('app.current_tenant', true) OR current_setting('app.current_tenant', true) = 'system');

CREATE POLICY tenant_isolation_swarm_long_term_memory ON swarm_long_term_memory
USING (organization_id = current_setting('app.current_tenant', true) OR current_setting('app.current_tenant', true) = 'system');

CREATE POLICY tenant_isolation_onboarding_state ON onboarding_state
USING (organization_id = current_setting('app.current_tenant', true) OR current_setting('app.current_tenant', true) = 'system');

-- 4. Harden existing policies if necessary
-- Ensure shared_tasks_decomposition also has RLS
ALTER TABLE shared_tasks_decomposition ENABLE ROW LEVEL SECURITY;
CREATE POLICY tenant_isolation_shared_tasks_decomposition ON shared_tasks_decomposition
USING (organization_id = current_setting('app.current_tenant', true) OR current_setting('app.current_tenant', true) = 'system');

-- Ensure sub_agent_jobs also has RLS
ALTER TABLE sub_agent_jobs ENABLE ROW LEVEL SECURITY;
ALTER TABLE sub_agent_jobs ADD COLUMN IF NOT EXISTS organization_id TEXT NOT NULL DEFAULT 'system';
CREATE POLICY tenant_isolation_sub_agent_jobs ON sub_agent_jobs
USING (organization_id = current_setting('app.current_tenant', true) OR current_setting('app.current_tenant', true) = 'system');
