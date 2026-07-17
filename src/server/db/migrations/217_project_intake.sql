-- +goose Up
CREATE TABLE IF NOT EXISTS project_intakes (
    id UUID PRIMARY KEY,
    tenant_id TEXT NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
    customer_id UUID REFERENCES customers(id) ON DELETE SET NULL,
    inquiry TEXT NOT NULL,
    status TEXT NOT NULL DEFAULT 'new' CHECK (status IN ('new', 'processed')),
    source TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX IF NOT EXISTS idx_project_intakes_tenant_id ON project_intakes(tenant_id);

ALTER TABLE project_intakes ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_project_intakes ON project_intakes;
CREATE POLICY tenant_isolation_project_intakes
ON project_intakes
USING (tenant_id = current_setting('app.current_tenant', true))
WITH CHECK (tenant_id = current_setting('app.current_tenant', true));

CREATE TABLE IF NOT EXISTS project_tasks (
    id UUID PRIMARY KEY,
    tenant_id TEXT NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
    proposal_id UUID REFERENCES proposals(id) ON DELETE CASCADE,
    description TEXT NOT NULL,
    status TEXT NOT NULL DEFAULT 'pending' CHECK (status IN ('pending', 'in_progress', 'completed')),
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX IF NOT EXISTS idx_project_tasks_tenant_id ON project_tasks(tenant_id);

ALTER TABLE project_tasks ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_project_tasks ON project_tasks;
CREATE POLICY tenant_isolation_project_tasks
ON project_tasks
USING (tenant_id = current_setting('app.current_tenant', true))
WITH CHECK (tenant_id = current_setting('app.current_tenant', true));

-- +goose Down
DROP POLICY IF EXISTS tenant_isolation_project_tasks ON project_tasks;
DROP TABLE IF EXISTS project_tasks CASCADE;

DROP POLICY IF EXISTS tenant_isolation_project_intakes ON project_intakes;
DROP TABLE IF EXISTS project_intakes CASCADE;
