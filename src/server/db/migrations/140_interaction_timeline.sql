-- +goose Up
CREATE TABLE IF NOT EXISTS interaction_timeline (
    id UUID PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    customer_id TEXT NOT NULL,
    source TEXT NOT NULL,
    sentiment TEXT NOT NULL,
    occurred_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX IF NOT EXISTS idx_interaction_timeline_tenant_customer ON interaction_timeline(tenant_id, customer_id);

ALTER TABLE interaction_timeline ENABLE ROW LEVEL SECURITY;
CREATE POLICY tenant_isolation_interaction_timeline ON interaction_timeline USING (tenant_id = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id = current_setting('app.current_tenant', true));

-- +goose Down
DROP POLICY IF EXISTS tenant_isolation_interaction_timeline ON interaction_timeline;
DROP TABLE IF EXISTS interaction_timeline CASCADE;
