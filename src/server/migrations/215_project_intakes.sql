-- +goose Up

CREATE TABLE IF NOT EXISTS project_intakes (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
    customer_id TEXT,
    inquiry TEXT NOT NULL,
    status TEXT NOT NULL DEFAULT 'PENDING' CHECK (status IN ('PENDING', 'PROPOSAL_DRAFTED', 'PROPOSAL_SENT', 'ACCEPTED', 'REJECTED')),
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX IF NOT EXISTS idx_project_intakes_tenant_id ON project_intakes(tenant_id);

ALTER TABLE project_intakes ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_project_intakes ON project_intakes;
CREATE POLICY tenant_isolation_project_intakes
ON project_intakes
USING (tenant_id::text = current_setting('app.current_tenant', true))
WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

ALTER TABLE proposals ADD COLUMN IF NOT EXISTS project_intake_id TEXT REFERENCES project_intakes(id) ON DELETE SET NULL;

-- +goose Down
ALTER TABLE proposals DROP COLUMN IF EXISTS project_intake_id;
DROP POLICY IF EXISTS tenant_isolation_project_intakes ON project_intakes;
DROP TABLE IF EXISTS project_intakes CASCADE;
