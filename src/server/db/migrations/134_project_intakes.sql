-- +goose Up
CREATE TABLE IF NOT EXISTS project_intakes (
    id UUID PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    customer_id UUID,
    raw_request TEXT NOT NULL,
    parsed_requirements JSONB,
    status TEXT NOT NULL DEFAULT 'NEW' CHECK (status IN ('NEW', 'PROCESSED', 'REJECTED')),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

ALTER TABLE quotes ADD COLUMN IF NOT EXISTS signature_hash TEXT;

ALTER TABLE project_intakes ENABLE ROW LEVEL SECURITY;
CREATE POLICY tenant_isolation_project_intakes ON project_intakes USING (tenant_id = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id = current_setting('app.current_tenant', true));

-- +goose Down
DROP TABLE IF EXISTS project_intakes;
ALTER TABLE quotes DROP COLUMN IF EXISTS signature_hash;
