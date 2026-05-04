
-- Enable RLS and add isolation policies for tables containing tenant data

ALTER TABLE hybrid_fs_sync_queue ENABLE ROW LEVEL SECURITY;
CREATE POLICY tenant_isolation_hybrid_fs_sync_queue ON hybrid_fs_sync_queue USING (organization_id = current_setting('app.current_tenant', true));

ALTER TABLE shared_tasks_decomposition ENABLE ROW LEVEL SECURITY;
CREATE POLICY tenant_isolation_shared_tasks_decomposition ON shared_tasks_decomposition USING (organization_id = current_setting('app.current_tenant', true));
