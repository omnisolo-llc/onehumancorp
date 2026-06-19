CREATE TABLE IF NOT EXISTS task_dependencies (
    task_id UUID NOT NULL,
    depends_on_task_id UUID NOT NULL,
    tenant_id TEXT NOT NULL DEFAULT 'default_tenant',
    PRIMARY KEY (task_id, depends_on_task_id)
);

ALTER TABLE task_dependencies ENABLE ROW LEVEL SECURITY;
CREATE POLICY tenant_isolation_task_dependencies ON task_dependencies
    USING (tenant_id = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id = current_setting('app.current_tenant', true));
