-- 080_tenant_isolation_audit.sql
-- Missing RLS from table diff
ALTER TABLE builder_blocks ENABLE ROW LEVEL SECURITY;
ALTER TABLE builder_blocks ADD COLUMN IF NOT EXISTS tenant_id TEXT NOT NULL DEFAULT 'system';
CREATE POLICY tenant_isolation_builder_blocks ON builder_blocks USING (tenant_id::text = current_setting('app.current_tenant', true) OR current_setting('app.current_tenant', true) = 'system');

ALTER TABLE builder_pages ENABLE ROW LEVEL SECURITY;
ALTER TABLE builder_pages ADD COLUMN IF NOT EXISTS tenant_id TEXT NOT NULL DEFAULT 'system';
CREATE POLICY tenant_isolation_builder_pages ON builder_pages USING (tenant_id::text = current_setting('app.current_tenant', true) OR current_setting('app.current_tenant', true) = 'system');

ALTER TABLE builder_sites ENABLE ROW LEVEL SECURITY;
ALTER TABLE builder_sites ADD COLUMN IF NOT EXISTS tenant_id TEXT NOT NULL DEFAULT 'system';
CREATE POLICY tenant_isolation_builder_sites ON builder_sites USING (tenant_id::text = current_setting('app.current_tenant', true) OR current_setting('app.current_tenant', true) = 'system');

ALTER TABLE capability_plugins ENABLE ROW LEVEL SECURITY;
ALTER TABLE capability_plugins ADD COLUMN IF NOT EXISTS organization_id TEXT NOT NULL DEFAULT '';
CREATE POLICY tenant_isolation_capability_plugins ON capability_plugins USING (organization_id::text = current_setting('app.current_tenant', true));

ALTER TABLE local_mcp_rag_tasks ENABLE ROW LEVEL SECURITY;
ALTER TABLE local_mcp_rag_tasks ADD COLUMN IF NOT EXISTS tenant_id TEXT NOT NULL DEFAULT 'system';
CREATE POLICY tenant_isolation_local_mcp_rag_tasks ON local_mcp_rag_tasks USING (tenant_id::text = current_setting('app.current_tenant', true) OR current_setting('app.current_tenant', true) = 'system');

ALTER TABLE memory_conflicts ENABLE ROW LEVEL SECURITY;
ALTER TABLE memory_conflicts ADD COLUMN IF NOT EXISTS organization_id TEXT NOT NULL DEFAULT '';
CREATE POLICY tenant_isolation_memory_conflicts ON memory_conflicts USING (organization_id::text = current_setting('app.current_tenant', true));

ALTER TABLE organizations ENABLE ROW LEVEL SECURITY;
ALTER TABLE organizations ADD COLUMN IF NOT EXISTS tenant_id TEXT NOT NULL DEFAULT 'system';
CREATE POLICY tenant_isolation_organizations ON organizations USING (tenant_id::text = current_setting('app.current_tenant', true) OR current_setting('app.current_tenant', true) = 'system');

ALTER TABLE pages ENABLE ROW LEVEL SECURITY;
ALTER TABLE pages ADD COLUMN IF NOT EXISTS tenant_id TEXT NOT NULL DEFAULT 'system';
CREATE POLICY tenant_isolation_pages ON pages USING (tenant_id::text = current_setting('app.current_tenant', true) OR current_setting('app.current_tenant', true) = 'system');

ALTER TABLE scheduled_tasks ENABLE ROW LEVEL SECURITY;
ALTER TABLE scheduled_tasks ADD COLUMN IF NOT EXISTS organization_id TEXT NOT NULL DEFAULT '';
CREATE POLICY tenant_isolation_scheduled_tasks ON scheduled_tasks USING (organization_id::text = current_setting('app.current_tenant', true));

ALTER TABLE swarm_memory ENABLE ROW LEVEL SECURITY;
ALTER TABLE swarm_memory ADD COLUMN IF NOT EXISTS organization_id TEXT NOT NULL DEFAULT '';
CREATE POLICY tenant_isolation_swarm_memory ON swarm_memory USING (organization_id::text = current_setting('app.current_tenant', true));

ALTER TABLE swarm_memory_embeddings ENABLE ROW LEVEL SECURITY;
ALTER TABLE swarm_memory_embeddings ADD COLUMN IF NOT EXISTS organization_id TEXT NOT NULL DEFAULT '';
CREATE POLICY tenant_isolation_swarm_memory_embeddings ON swarm_memory_embeddings USING (organization_id::text = current_setting('app.current_tenant', true));

ALTER TABLE task_dependencies ENABLE ROW LEVEL SECURITY;
ALTER TABLE task_dependencies ADD COLUMN IF NOT EXISTS organization_id TEXT NOT NULL DEFAULT '';
CREATE POLICY tenant_isolation_task_dependencies ON task_dependencies USING (organization_id::text = current_setting('app.current_tenant', true));

ALTER TABLE tasks ENABLE ROW LEVEL SECURITY;
ALTER TABLE tasks ADD COLUMN IF NOT EXISTS organization_id TEXT NOT NULL DEFAULT '';
CREATE POLICY tenant_isolation_tasks ON tasks USING (organization_id::text = current_setting('app.current_tenant', true));

ALTER TABLE tool_integrations ENABLE ROW LEVEL SECURITY;
ALTER TABLE tool_integrations ADD COLUMN IF NOT EXISTS tenant_id TEXT NOT NULL DEFAULT 'system';
CREATE POLICY tenant_isolation_tool_integrations ON tool_integrations USING (tenant_id::text = current_setting('app.current_tenant', true) OR current_setting('app.current_tenant', true) = 'system');

ALTER TABLE usage_events ENABLE ROW LEVEL SECURITY;
ALTER TABLE usage_events ADD COLUMN IF NOT EXISTS organization_id TEXT NOT NULL DEFAULT '';
CREATE POLICY tenant_isolation_usage_events ON usage_events USING (organization_id::text = current_setting('app.current_tenant', true));

ALTER TABLE embedding_cache ENABLE ROW LEVEL SECURITY;
ALTER TABLE embedding_cache ADD COLUMN IF NOT EXISTS organization_id TEXT NOT NULL DEFAULT '';
CREATE POLICY tenant_isolation_embedding_cache ON embedding_cache USING (organization_id::text = current_setting('app.current_tenant', true));

ALTER TABLE local_cloud_sync_log ENABLE ROW LEVEL SECURITY;
ALTER TABLE local_cloud_sync_log ADD COLUMN IF NOT EXISTS organization_id TEXT NOT NULL DEFAULT '';
CREATE POLICY tenant_isolation_local_cloud_sync_log ON local_cloud_sync_log USING (organization_id::text = current_setting('app.current_tenant', true));

ALTER TABLE sub_agent_jobs ENABLE ROW LEVEL SECURITY;
ALTER TABLE sub_agent_jobs ADD COLUMN IF NOT EXISTS organization_id TEXT NOT NULL DEFAULT '';
CREATE POLICY tenant_isolation_sub_agent_jobs ON sub_agent_jobs USING (organization_id::text = current_setting('app.current_tenant', true));
