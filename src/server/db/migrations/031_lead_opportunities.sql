-- +goose Up
CREATE TABLE IF NOT EXISTS leads (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    source TEXT NOT NULL,
    contact_info TEXT NOT NULL,
    context TEXT,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS opportunities (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    lead_id TEXT REFERENCES leads(id),
    title TEXT NOT NULL,
    stage TEXT NOT NULL,
    estimated_value DOUBLE PRECISION DEFAULT 0.0,
    priority TEXT NOT NULL,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

DO $$
BEGIN
    IF to_regclass('leads') IS NOT NULL THEN
        ALTER TABLE leads ENABLE ROW LEVEL SECURITY;
        CREATE POLICY tenant_isolation_leads ON leads USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
    END IF;
    IF to_regclass('opportunities') IS NOT NULL THEN
        ALTER TABLE opportunities ENABLE ROW LEVEL SECURITY;
        CREATE POLICY tenant_isolation_opportunities ON opportunities USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
    END IF;
END
$$;

-- +goose Down
DO $$
BEGIN
    DROP POLICY IF EXISTS tenant_isolation_leads ON leads;
    DROP POLICY IF EXISTS tenant_isolation_opportunities ON opportunities;
END
$$;

DROP TABLE IF EXISTS opportunities CASCADE;
DROP TABLE IF EXISTS leads CASCADE;
