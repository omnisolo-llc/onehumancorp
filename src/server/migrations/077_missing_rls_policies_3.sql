-- Missing RLS tables fix

ALTER TABLE state_machine_transitions ENABLE ROW LEVEL SECURITY;
CREATE POLICY "Tenant isolation for state_machine_transitions" ON state_machine_transitions
    USING (tenant_id = current_setting('app.current_tenant_id')::uuid);

ALTER TABLE swarm_tasks ENABLE ROW LEVEL SECURITY;
CREATE POLICY "Tenant isolation for swarm_tasks" ON swarm_tasks
    USING (tenant_id = current_setting('app.current_tenant_id')::uuid);

ALTER TABLE task_dependencies ENABLE ROW LEVEL SECURITY;
CREATE POLICY "Tenant isolation for task_dependencies" ON task_dependencies
    USING (tenant_id = current_setting('app.current_tenant_id')::uuid);

ALTER TABLE tasks ENABLE ROW LEVEL SECURITY;
CREATE POLICY "Tenant isolation for tasks" ON tasks
    USING (tenant_id = current_setting('app.current_tenant_id')::uuid);
