-- +goose Up
-- Migration 212: Fix missing RLS policies on tables
-- Enforce Strict Multi-Tenancy via PostgreSQL RLS for tables that missed it

ALTER TABLE task_dependencies ADD COLUMN IF NOT EXISTS tenant_id TEXT DEFAULT current_setting('app.current_tenant', true);
UPDATE task_dependencies td SET tenant_id = (SELECT tenant_id FROM tasks t WHERE t.id = td.task_id) WHERE tenant_id IS NULL OR tenant_id = '';
ALTER TABLE IF EXISTS task_dependencies ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_task_dependencies ON task_dependencies;
CREATE POLICY tenant_isolation_task_dependencies ON task_dependencies USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));


ALTER TABLE IF EXISTS swarm_tasks ADD COLUMN IF NOT EXISTS tenant_id TEXT DEFAULT current_setting('app.current_tenant', true);
UPDATE swarm_tasks st SET tenant_id = current_setting('app.current_tenant', true) WHERE tenant_id IS NULL OR tenant_id = '';
ALTER TABLE IF EXISTS swarm_tasks ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_swarm_tasks ON swarm_tasks;
CREATE POLICY tenant_isolation_swarm_tasks ON swarm_tasks USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

-- +goose Down
DROP POLICY IF EXISTS tenant_isolation_swarm_tasks ON swarm_tasks;
ALTER TABLE IF EXISTS swarm_tasks DISABLE ROW LEVEL SECURITY;

DROP POLICY IF EXISTS tenant_isolation_task_dependencies ON task_dependencies;
ALTER TABLE IF EXISTS task_dependencies DISABLE ROW LEVEL SECURITY;
