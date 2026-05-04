-- PostgreSQL specific RLS policies for Kairos Shared Tasks
-- This migration will only be executed if the database is PostgreSQL.

ALTER TABLE shared_tasks ENABLE ROW LEVEL SECURITY;
CREATE POLICY "tenant_isolation_shared_tasks" ON shared_tasks FOR ALL USING (tenant_id = current_setting('app.current_tenant', true));

ALTER TABLE task_dependencies ENABLE ROW LEVEL SECURITY;
CREATE POLICY "tenant_isolation_task_dependencies" ON task_dependencies FOR ALL USING (tenant_id = current_setting('app.current_tenant', true));
