-- PostgreSQL specific RLS policies for missing tables
-- This migration will only be executed if the database is PostgreSQL.

ALTER TABLE hybrid_fs_sync_queue ENABLE ROW LEVEL SECURITY;
CREATE POLICY "tenant_isolation_hybrid_fs_sync_queue" ON hybrid_fs_sync_queue FOR ALL USING (organization_id = current_setting('app.current_tenant', true));

ALTER TABLE shared_tasks_decomposition ENABLE ROW LEVEL SECURITY;
CREATE POLICY "tenant_isolation_shared_tasks_decomposition" ON shared_tasks_decomposition FOR ALL USING (organization_id = current_setting('app.current_tenant', true));

ALTER TABLE state_machine_transitions ENABLE ROW LEVEL SECURITY;
CREATE POLICY "tenant_isolation_state_machine_transitions" ON state_machine_transitions FOR ALL USING (
    task_id IN (SELECT id FROM shared_tasks_decomposition WHERE organization_id = current_setting('app.current_tenant', true))
);
