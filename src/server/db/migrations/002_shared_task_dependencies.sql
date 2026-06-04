CREATE TABLE IF NOT EXISTS shared_task_dependencies (
    task_id TEXT NOT NULL,
    depends_on_task_id TEXT NOT NULL,
    PRIMARY KEY (task_id, depends_on_task_id),
    FOREIGN KEY (task_id) REFERENCES shared_tasks(id) ON DELETE CASCADE,
    FOREIGN KEY (depends_on_task_id) REFERENCES shared_tasks(id) ON DELETE CASCADE
);
ALTER TABLE shared_task_dependencies ADD COLUMN IF NOT EXISTS organization_id TEXT;
CREATE INDEX IF NOT EXISTS idx_shared_task_dependencies_org_id ON shared_task_dependencies(organization_id);
ALTER TABLE shared_task_dependencies ENABLE ROW LEVEL SECURITY;
CREATE POLICY tenant_isolation_shared_task_dependencies ON shared_task_dependencies USING (organization_id::text = current_setting('app.current_tenant', true)) WITH CHECK (organization_id::text = current_setting('app.current_tenant', true));