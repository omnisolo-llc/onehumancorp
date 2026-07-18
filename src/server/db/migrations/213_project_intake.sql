-- +goose Up
CREATE TABLE IF NOT EXISTS project_intakes (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    customer_id TEXT,
    raw_inquiry TEXT NOT NULL,
    extracted_intent JSONB,
    status TEXT NOT NULL DEFAULT 'new' CHECK (status IN ('new', 'processing', 'proposal_drafted', 'completed', 'rejected')),
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

-- +goose Down
DROP POLICY IF EXISTS tenant_isolation_project_intakes ON project_intakes;
DROP TABLE IF EXISTS project_intakes CASCADE;
