-- +goose Up
CREATE TABLE IF NOT EXISTS customer_insights (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    customer_id UUID NOT NULL,
    category TEXT NOT NULL,
    content TEXT NOT NULL,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS customer_interactions (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    customer_id UUID NOT NULL,
    interaction_type TEXT NOT NULL,
    content TEXT NOT NULL,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

ALTER TABLE customer_insights ENABLE ROW LEVEL SECURITY;
CREATE POLICY tenant_isolation_customer_insights ON customer_insights USING (tenant_id = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id = current_setting('app.current_tenant', true));

ALTER TABLE customer_interactions ENABLE ROW LEVEL SECURITY;
CREATE POLICY tenant_isolation_customer_interactions ON customer_interactions USING (tenant_id = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id = current_setting('app.current_tenant', true));

-- +goose Down
DROP POLICY IF EXISTS tenant_isolation_customer_interactions ON customer_interactions;
DROP TABLE IF EXISTS customer_interactions CASCADE;

DROP POLICY IF EXISTS tenant_isolation_customer_insights ON customer_insights;
DROP TABLE IF EXISTS customer_insights CASCADE;
