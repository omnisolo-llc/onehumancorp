-- +goose Up
CREATE TABLE IF NOT EXISTS task_dependencies (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    organization_id VARCHAR NOT NULL,
    task_id UUID NOT NULL REFERENCES shared_tasks(id) ON DELETE CASCADE,
    depends_on_task_id UUID NOT NULL REFERENCES shared_tasks(id) ON DELETE CASCADE,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    UNIQUE(task_id, depends_on_task_id)
);
ALTER TABLE task_dependencies ENABLE ROW LEVEL SECURITY;
CREATE POLICY tenant_isolation_task_dependencies ON task_dependencies USING (organization_id = current_setting('app.current_tenant', true));

-- +goose Down
DROP TABLE IF EXISTS task_dependencies;
